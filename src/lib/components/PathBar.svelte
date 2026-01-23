<script lang="ts">
  import { tick } from 'svelte';
  import Fa from 'svelte-fa';
  import { faFolder } from '@fortawesome/free-solid-svg-icons';
  import { readDirectory, normalizeWslPath, type FileEntry } from '$lib/utils/ipc';

  interface Props {
    path: string;
    editing?: boolean;
    onNavigate: (path: string) => void;
    onEditStart?: () => void;
    onEditEnd?: () => void;
  }

  let { path, editing = false, onNavigate, onEditStart, onEditEnd }: Props = $props();

  let inputValue = $state(path);
  let inputRef: HTMLInputElement;
  let suggestionsRef: HTMLUListElement;
  let isInitializing = $state(false);

  // Autocomplete state
  let suggestions = $state<string[]>([]);
  let selectedIndex = $state(0);
  let showDropdown = $state(false);
  let isSelectingSuggestion = $state(false);
  let fetchTimeout: ReturnType<typeof setTimeout> | null = null;

  interface PathSegment {
    name: string;
    path: string;
  }

  let segments = $derived((): PathSegment[] => {
    if (!path) return [];

    const parts = path.split(/[/\\]/).filter(Boolean);
    const result: PathSegment[] = [];

    // Handle UNC paths (\\wsl$\distro\... or \\server\share\...)
    if (path.startsWith('\\\\') || path.startsWith('//')) {
      // UNC paths need at least \\server\share as root
      if (parts.length >= 2) {
        const root = `\\\\${parts[0]}\\${parts[1]}`;
        result.push({ name: `\\\\${parts[0]}\\${parts[1]}`, path: root });

        let currentPath = root;
        for (let i = 2; i < parts.length; i++) {
          currentPath += '\\' + parts[i];
          result.push({ name: parts[i], path: currentPath });
        }
      }
    }
    // Handle Windows drive letters
    else if (path.match(/^[A-Z]:/i)) {
      let currentPath = parts[0] + '\\';
      result.push({ name: parts[0], path: currentPath });

      for (let i = 1; i < parts.length; i++) {
        currentPath += parts[i] + '\\';
        result.push({ name: parts[i], path: currentPath.slice(0, -1) });
      }
    } else {
      // Unix paths
      let currentPath = '/';
      result.push({ name: '/', path: '/' });

      for (const part of parts) {
        currentPath += part + '/';
        result.push({ name: part, path: currentPath.slice(0, -1) });
      }
    }

    return result;
  });

  $effect(() => {
    if (editing) {
      isInitializing = true;
      tick().then(() => {
        if (inputRef) {
          inputValue = path;
          inputRef.focus();
          inputRef.select();
        }
        // Delay flag reset to prevent immediate blur from closing
        setTimeout(() => {
          isInitializing = false;
        }, 50);
      });
    } else {
      // Reset autocomplete state when exiting edit mode
      suggestions = [];
      showDropdown = false;
      selectedIndex = 0;
    }
  });

  // Prefix match: checks if name starts with query (case-insensitive)
  function prefixMatch(query: string, name: string): boolean {
    return name.toLowerCase().startsWith(query.toLowerCase());
  }

  // Get longest common prefix from array of strings
  function getLongestCommonPrefix(strings: string[]): string {
    if (strings.length === 0) return '';
    if (strings.length === 1) return strings[0];

    let prefix = strings[0];
    for (let i = 1; i < strings.length; i++) {
      while (!strings[i].toLowerCase().startsWith(prefix.toLowerCase())) {
        prefix = prefix.slice(0, -1);
        if (prefix === '') return '';
      }
      // Use the actual casing from first match
      prefix = strings[i].slice(0, prefix.length);
    }
    return prefix;
  }

  // Parse input path into directory and partial filename
  function parseInputPath(input: string): { dir: string; partial: string } {
    const separatorMatch = input.match(/[/\\]/);
    const separator = separatorMatch ? separatorMatch[0] : (input.match(/^[A-Z]:/i) ? '\\' : '/');
    const lastSepIndex = Math.max(input.lastIndexOf('/'), input.lastIndexOf('\\'));

    if (lastSepIndex === -1) {
      // No separator, treat as partial in current directory
      return { dir: path, partial: input };
    }

    const dir = input.slice(0, lastSepIndex + 1);
    const partial = input.slice(lastSepIndex + 1);
    return { dir, partial };
  }

  // Fetch directory suggestions
  async function fetchSuggestions(inputPath: string) {
    if (!inputPath.trim()) {
      suggestions = [];
      showDropdown = false;
      return;
    }

    const { dir, partial } = parseInputPath(inputPath);
    const normalizedDir = normalizeWslPath(dir);

    try {
      const entries = await readDirectory(normalizedDir);
      const dirs = entries
        .filter((e: FileEntry) => e.is_dir)
        .filter((e: FileEntry) => {
          if (!partial) return true;
          return prefixMatch(partial, e.name);
        })
        .map((e: FileEntry) => e.name)
        .sort((a, b) => a.toLowerCase().localeCompare(b.toLowerCase()));

      suggestions = dirs;
      selectedIndex = 0;
      showDropdown = dirs.length > 0;
    } catch {
      suggestions = [];
      showDropdown = false;
    }
  }

  // Debounced input handler
  function handleInput(event: Event) {
    const value = (event.target as HTMLInputElement).value;
    if (fetchTimeout) clearTimeout(fetchTimeout);
    fetchTimeout = setTimeout(() => {
      fetchSuggestions(value);
    }, 50);
  }

  // Select a suggestion - navigate directly to the selected folder and close edit mode
  function selectSuggestion(suggestion: string) {
    const { dir } = parseInputPath(inputValue);
    const fullPath = normalizeWslPath(dir + suggestion);

    showDropdown = false;
    suggestions = [];
    onNavigate(fullPath);
    onEditEnd?.();
  }

  // Enter a directory - update input and fetch new suggestions (stay in edit mode)
  function enterDirectory(suggestion: string) {
    const { dir } = parseInputPath(inputValue);
    const separator = dir.includes('\\') || dir.match(/^[A-Z]:/i) ? '\\' : '/';
    const fullPath = dir + suggestion + separator;

    inputValue = fullPath;
    selectedIndex = 0;
    fetchSuggestions(fullPath);
  }

  // Handle Tab key - enter the highlighted directory
  function handleTab(): boolean {
    if (suggestions.length === 0 || selectedIndex >= suggestions.length) return false;

    enterDirectory(suggestions[selectedIndex]);
    return true;
  }

  function handleClick(segment: PathSegment) {
    onNavigate(segment.path);
  }

  async function handleInputKeyDown(event: KeyboardEvent) {
    if (event.key === 'ArrowDown') {
      if (showDropdown && suggestions.length > 0) {
        event.preventDefault();
        selectedIndex = (selectedIndex + 1) % suggestions.length;
        await tick();
        scrollSelectedIntoView();
      }
    } else if (event.key === 'ArrowUp') {
      if (showDropdown && suggestions.length > 0) {
        event.preventDefault();
        selectedIndex = (selectedIndex - 1 + suggestions.length) % suggestions.length;
        await tick();
        scrollSelectedIntoView();
      }
    } else if (event.key === 'Tab') {
      event.preventDefault();
      // Cancel pending debounce
      if (fetchTimeout) {
        clearTimeout(fetchTimeout);
        fetchTimeout = null;
      }
      
      if (showDropdown && suggestions.length > 0 && selectedIndex < suggestions.length) {
        // User is tabbing with a suggestion selected - enter that directory
        enterDirectory(suggestions[selectedIndex]);
      } else if (inputValue.trim()) {
        // No dropdown or nothing selected, try to fetch suggestions first
        await fetchSuggestions(inputValue);
        
        // If we now have suggestions, enter the first one
        if (suggestions.length > 0) {
          enterDirectory(suggestions[0]);
        }
      }
    } else if (event.key === 'Enter') {
      event.preventDefault();
      // Cancel pending debounce
      if (fetchTimeout) {
        clearTimeout(fetchTimeout);
        fetchTimeout = null;
      }
      
      if (inputValue.trim()) {
        // Remember current selection before fetching
        const currentSelection = suggestions.length > 0 ? suggestions[selectedIndex] : null;
        
        // Fetch fresh suggestions to ensure we have the latest
        await fetchSuggestions(inputValue);
        
        // Try to restore the previous selection if it still exists
        if (currentSelection && suggestions.includes(currentSelection)) {
          selectedIndex = suggestions.indexOf(currentSelection);
        }
        
        // Now select the highlighted suggestion if available
        if (suggestions.length > 0 && selectedIndex < suggestions.length) {
          selectSuggestion(suggestions[selectedIndex]);
        } else {
          // No matching suggestions, navigate to the typed path
          showDropdown = false;
          onNavigate(normalizeWslPath(inputValue.trim()));
          onEditEnd?.();
        }
      }
    } else if (event.key === 'Escape') {
      event.preventDefault();
      if (showDropdown) {
        showDropdown = false;
        suggestions = [];
      } else {
        inputValue = path;
        onEditEnd?.();
      }
    }
  }

  function handleInputBlur() {
    // Ignore blur during initialization to prevent race condition
    if (isInitializing) return;
    // Ignore blur when clicking on suggestions
    if (isSelectingSuggestion) return;
    showDropdown = false;
    suggestions = [];
    onEditEnd?.();
  }

  function handleSuggestionMouseDown() {
    isSelectingSuggestion = true;
  }

  function handleSuggestionClick(suggestion: string) {
    selectSuggestion(suggestion);
    isSelectingSuggestion = false;
  }

  function handlePathBarClick(event: MouseEvent) {
    // Only start edit mode if clicking on the path bar itself, not on segment buttons
    if ((event.target as HTMLElement).closest('.segment')) return;
    onEditStart?.();
  }

  function scrollSelectedIntoView() {
    if (!suggestionsRef) return;
    const selectedItem = suggestionsRef.children[selectedIndex] as HTMLElement;
    if (selectedItem) {
      selectedItem.scrollIntoView({ block: 'nearest' });
    }
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_noninteractive_element_interactions -->
<nav class="path-bar" class:editing onclick={handlePathBarClick}>
  {#if editing}
    <div class="input-wrapper">
      <input
        bind:this={inputRef}
        bind:value={inputValue}
        class="path-input"
        type="text"
        onkeydown={handleInputKeyDown}
        onblur={handleInputBlur}
        oninput={handleInput}
        spellcheck="false"
        autocomplete="off"
      />
      {#if showDropdown && suggestions.length > 0}
        <ul class="suggestions" bind:this={suggestionsRef}>
          {#each suggestions as suggestion, i}
            <li class:selected={i === selectedIndex}>
              <button
                onmousedown={handleSuggestionMouseDown}
                onclick={() => handleSuggestionClick(suggestion)}
              >
                <span class="folder-icon"><Fa icon={faFolder} /></span>
                {suggestion}
              </button>
            </li>
          {/each}
        </ul>
      {/if}
    </div>
  {:else}
    {#each segments() as segment, i}
      {#if i > 0}
        <span class="separator">/</span>
      {/if}
      <button
        class="segment"
        class:current={i === segments().length - 1}
        onclick={() => handleClick(segment)}
      >
        {segment.name}
      </button>
    {/each}
  {/if}
</nav>

<style>
  .path-bar {
    display: flex;
    align-items: center;
    gap: 2px;
    padding: 5px 8px;
    background: var(--pathbar-bg);
    border-bottom: 1px solid var(--border-color);
    font-size: 13px;
    overflow-x: auto;
    white-space: nowrap;
    cursor: text;
  }

  .path-bar.editing {
    overflow: visible;
    cursor: default;
  }

  .input-wrapper {
    flex: 1;
    position: relative;
  }

  .path-input {
    width: 100%;
    background: var(--input-bg);
    border: 1px solid var(--focus-border);
    border-radius: 4px;
    padding: 4px 8px;
    font: inherit;
    color: var(--fg);
    outline: none;
    box-sizing: border-box;
  }

  .suggestions {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    margin: 4px 0 0 0;
    padding: 4px 0;
    background: var(--bg, #1e1e1e);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.5);
    list-style: none;
    max-height: 200px;
    overflow-y: auto;
    z-index: 1000;
  }

  .suggestions li button {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 6px 12px;
    background: none;
    border: none;
    font: inherit;
    color: var(--fg);
    text-align: left;
    cursor: pointer;
  }

  .folder-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    height: 16px;
    flex-shrink: 0;
    color: var(--muted-fg);
    font-size: 14px;
  }

  .suggestions li button:hover {
    background: var(--hover-bg);
  }

  .suggestions li.selected button {
    background: var(--selection-bg);
    color: var(--selection-fg);
  }

  .separator {
    color: var(--muted-fg);
    user-select: none;
  }

  .segment {
    background: none;
    border: none;
    padding: 4px 6px;
    border-radius: 4px;
    font: inherit;
    color: var(--link-fg);
    cursor: pointer;
    transition: background-color 0.1s;
  }

  .segment:hover {
    background: var(--hover-bg);
  }

  .segment.current {
    color: var(--fg);
    font-weight: 600;
    background: var(--badge-bg);
  }
</style>
