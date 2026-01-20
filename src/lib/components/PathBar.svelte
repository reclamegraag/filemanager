<script lang="ts">
  import { tick } from 'svelte';
  import { readDirectory, type FileEntry } from '$lib/utils/ipc';

  interface Props {
    path: string;
    editing?: boolean;
    onNavigate: (path: string) => void;
    onEditEnd?: () => void;
  }

  let { path, editing = false, onNavigate, onEditEnd }: Props = $props();

  let inputValue = $state(path);
  let inputRef: HTMLInputElement;
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

    // Handle Windows drive letters
    if (path.match(/^[A-Z]:/i)) {
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

  // Fuzzy match: checks if query characters appear in order within name
  function fuzzyMatch(query: string, name: string): boolean {
    const lowerQuery = query.toLowerCase();
    const lowerName = name.toLowerCase();

    let queryIndex = 0;
    for (let i = 0; i < lowerName.length && queryIndex < lowerQuery.length; i++) {
      if (lowerName[i] === lowerQuery[queryIndex]) {
        queryIndex++;
      }
    }
    return queryIndex === lowerQuery.length;
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

    try {
      const entries = await readDirectory(dir);
      const dirs = entries
        .filter((e: FileEntry) => e.is_dir)
        .filter((e: FileEntry) => {
          if (!partial) return true;
          return fuzzyMatch(partial, e.name);
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
  function handleInput() {
    if (fetchTimeout) clearTimeout(fetchTimeout);
    fetchTimeout = setTimeout(() => {
      fetchSuggestions(inputValue);
    }, 150);
  }

  // Select a suggestion
  function selectSuggestion(suggestion: string) {
    const { dir } = parseInputPath(inputValue);
    const separator = dir.includes('\\') ? '\\' : '/';
    inputValue = dir + suggestion + separator;
    showDropdown = false;
    suggestions = [];
    inputRef?.focus();
    // Trigger new suggestions for the selected directory
    fetchSuggestions(inputValue);
  }

  // Handle Tab key for autocomplete
  function handleTab(): boolean {
    if (!showDropdown || suggestions.length === 0) return false;

    if (suggestions.length === 1) {
      selectSuggestion(suggestions[0]);
      return true;
    }

    // Multiple matches: insert longest common prefix
    const prefix = getLongestCommonPrefix(suggestions);
    const { dir, partial } = parseInputPath(inputValue);

    if (prefix.length > partial.length) {
      const separator = dir.includes('\\') ? '\\' : '/';
      inputValue = dir + prefix;
      fetchSuggestions(inputValue);
      return true;
    }

    return false;
  }

  function handleClick(segment: PathSegment) {
    onNavigate(segment.path);
  }

  function handleInputKeyDown(event: KeyboardEvent) {
    if (event.key === 'ArrowDown') {
      if (showDropdown && suggestions.length > 0) {
        event.preventDefault();
        selectedIndex = (selectedIndex + 1) % suggestions.length;
      }
    } else if (event.key === 'ArrowUp') {
      if (showDropdown && suggestions.length > 0) {
        event.preventDefault();
        selectedIndex = (selectedIndex - 1 + suggestions.length) % suggestions.length;
      }
    } else if (event.key === 'Tab') {
      if (handleTab()) {
        event.preventDefault();
      }
    } else if (event.key === 'Enter') {
      event.preventDefault();
      if (showDropdown && suggestions.length > 0) {
        selectSuggestion(suggestions[selectedIndex]);
      } else if (inputValue.trim()) {
        showDropdown = false;
        onNavigate(inputValue.trim());
        onEditEnd?.();
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
</script>

<nav class="path-bar">
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
        <ul class="suggestions">
          {#each suggestions as suggestion, i}
            <li class:selected={i === selectedIndex}>
              <button
                onmousedown={handleSuggestionMouseDown}
                onclick={() => handleSuggestionClick(suggestion)}
              >
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
    padding: 8px 12px;
    background: var(--pathbar-bg);
    border-bottom: 1px solid var(--border-color);
    font-size: 13px;
    overflow-x: auto;
    white-space: nowrap;
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
    background: var(--panel-bg);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
    list-style: none;
    max-height: 200px;
    overflow-y: auto;
    z-index: 1000;
  }

  .suggestions li button {
    display: block;
    width: 100%;
    padding: 6px 12px;
    background: none;
    border: none;
    font: inherit;
    color: var(--fg);
    text-align: left;
    cursor: pointer;
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
    font-weight: 500;
  }
</style>
