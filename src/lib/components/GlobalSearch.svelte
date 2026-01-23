<script lang="ts">
  import Fa from 'svelte-fa';
  import { faSearch, faFolder, faFile, faHdd, faSpinner, faDatabase, faSync } from '@fortawesome/free-solid-svg-icons';
  import { searchIndex, startIndexing, getAvailableDrives, type FileEntry, type DriveInfo } from '$lib/utils/ipc';
  import { indexStore } from '$lib/stores/index';

  interface Props {
    open: boolean;
    onClose: () => void;
    onNavigate: (dirPath: string, fileName: string) => void;
  }

  let { open, onClose, onNavigate }: Props = $props();

  let query = $state('');
  let selectedIndex = $state(0);
  let results = $state<FileEntry[]>([]);
  let drives = $state<DriveInfo[]>([]);
  let selectedDrives = $state<string[]>([]);
  let loading = $state(false);
  let scopeOpen = $state(false);
  let drivesLoaded = $state(false);
  let inputRef: HTMLInputElement;
  let resultsRef: HTMLUListElement;
  let debounceTimer: ReturnType<typeof setTimeout>;

  const indexStatus = $derived($indexStore);

  async function loadDrives() {
    if (drivesLoaded) return;
    try {
      const available = await getAvailableDrives();
      drives = available;
      // Select first drive by default if none selected
      if (available.length > 0 && selectedDrives.length === 0) {
        const cDrive = available.find(d => d.path.toUpperCase().startsWith('C'));
        selectedDrives = [cDrive ? cDrive.path : available[0].path];
      }
      drivesLoaded = true;

      // Auto-start indexing if idle
      if (indexStatus.status === 'idle') {
        startIndexing(selectedDrives);
      }
    } catch (e) {
      console.error('Failed to load drives:', e);
    }
  }

  async function performSearch() {
    if (!query.trim()) {
      results = [];
      return;
    }

    loading = true;
    try {
      // Use indexed search instead of live walkdir
      results = await searchIndex(query, 500);
      selectedIndex = 0;
    } catch (e) {
      console.error('Search error:', e);
      results = [];
    } finally {
      loading = false;
    }
  }

  function handleInput() {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      performSearch();
    }, 150); // Faster debounce for indexed search
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (scopeOpen && event.key === 'Tab') {
      event.preventDefault();
      scopeOpen = false;
      inputRef?.focus();
      return;
    }

    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault();
        selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
        scrollToSelected();
        break;
      case 'ArrowUp':
        event.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, 0);
        scrollToSelected();
        break;
      case 'Enter':
        event.preventDefault();
        if (results[selectedIndex]) {
          selectResult(results[selectedIndex]);
        }
        break;
      case 'Escape':
        event.preventDefault();
        if (scopeOpen) {
          scopeOpen = false;
        } else {
          onClose();
        }
        break;
      case 'Tab':
        event.preventDefault();
        scopeOpen = !scopeOpen;
        break;
    }
  }

  function scrollToSelected() {
    if (resultsRef) {
      const item = resultsRef.children[selectedIndex] as HTMLElement;
      if (item) {
        item.scrollIntoView({ block: 'nearest' });
      }
    }
  }

  function selectResult(entry: FileEntry) {
    const dirPath = entry.is_dir
      ? entry.path
      : entry.path.substring(0, entry.path.lastIndexOf('\\') || entry.path.lastIndexOf('/'));
    const fileName = entry.is_dir ? '' : entry.name;

    onNavigate(dirPath || entry.path, fileName);
    onClose();
  }

  function toggleDrive(path: string) {
    if (selectedDrives.includes(path)) {
      if (selectedDrives.length > 1) {
        selectedDrives = selectedDrives.filter(d => d !== path);
      }
    } else {
      selectedDrives = [...selectedDrives, path];
    }
    // Restart indexing for new scope
    startIndexing(selectedDrives);
  }

  function formatPath(path: string): string {
    if (path.length > 60) {
      const parts = path.split(/[\\\/]/);
      if (parts.length > 4) {
        return `${parts[0]}\\...\\${parts.slice(-2).join('\\')}`;
      }
    }
    return path;
  }

  $effect(() => {
    if (open) {
      loadDrives();
      query = '';
      results = [];
      selectedIndex = 0;
      scopeOpen = false;

      setTimeout(() => {
        inputRef?.focus();
      }, 10);
    }
  });
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_interactive_supports_focus -->
  <div class="overlay" onclick={onClose} role="dialog" tabindex="-1">
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="search-panel" onclick={(e) => e.stopPropagation()} role="document">
      <div class="search-header">
        <div class="search-input-wrapper">
          <Fa icon={faSearch} class="search-icon" />
          <input
            bind:this={inputRef}
            bind:value={query}
            oninput={handleInput}
            onkeydown={handleKeyDown}
            class="search-input"
            type="text"
            placeholder="Search files..."
          />
          {#if loading || indexStatus.status === 'scanning'}
            <span class="loading-indicator" title="Indexing files...">
              <Fa icon={indexStatus.status === 'scanning' ? faSync : faSpinner} spin />
              {#if indexStatus.status === 'scanning'}
                <span class="index-count">{indexStatus.indexed_count}</span>
              {/if}
            </span>
          {/if}
        </div>

        <button class="scope-toggle" onclick={() => scopeOpen = !scopeOpen} class:active={scopeOpen}>
          <Fa icon={faDatabase} />
          <span>{indexStatus.indexed_count > 0 ? `${(indexStatus.indexed_count / 1000).toFixed(1)}k files` : 'Index'}</span>
        </button>
      </div>

      {#if scopeOpen}
        <div class="scope-dropdown">
          <div class="scope-label">Index Scope:</div>
          <div class="drive-list">
            {#each drives as drive}
              <button
                class="drive-option"
                class:selected={selectedDrives.includes(drive.path)}
                onclick={() => toggleDrive(drive.path)}
              >
                <Fa icon={faHdd} />
                <span class="drive-name">{drive.name}</span>
                <span class="drive-path">{drive.path}</span>
              </button>
            {/each}
          </div>
          <div class="index-status-bar">
            Status: <span class="status-val {indexStatus.status}">{indexStatus.status}</span>
            {#if indexStatus.current_path}
              <span class="current-path" title={indexStatus.current_path}>
                {formatPath(indexStatus.current_path)}
              </span>
            {/if}
          </div>
        </div>
      {/if}

      <ul class="results" bind:this={resultsRef}>
        {#each results as entry, i (entry.path)}
          <li>
            <button
              class="result"
              class:selected={i === selectedIndex}
              onclick={() => selectResult(entry)}
            >
              <span class="result-icon" class:is-dir={entry.is_dir}>
                <Fa icon={entry.is_dir ? faFolder : faFile} />
              </span>
              <div class="result-info">
                <span class="result-name">{entry.name}</span>
                <span class="result-path">{formatPath(entry.path)}</span>
              </div>
            </button>
          </li>
        {/each}

        {#if results.length === 0 && query.trim() && !loading}
          <li class="empty">No files found</li>
        {/if}

        {#if results.length === 0 && !query.trim()}
          <li class="empty-state">
            <Fa icon={faSearch} size="3x" class="empty-icon" />
            <p>Type to search instantly</p>
            <span class="index-info">Index contains {indexStatus.indexed_count} files</span>
          </li>
        {/if}
      </ul>

      <footer class="footer">
        <span class="hint"><span class="key">Tab</span> Scope</span>
        <span class="hint"><span class="key">&uarr;&darr;</span> Navigate</span>
        <span class="hint"><span class="key">Enter</span> Open</span>
        <span class="hint"><span class="key">Esc</span> Close</span>
      </footer>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: 80px;
    z-index: 1000;
  }

  .search-panel {
    width: 600px;
    max-width: 90vw;
    max-height: 70vh;
    background: var(--palette-bg);
    border-radius: 12px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .search-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color);
  }

  .search-input-wrapper {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 10px;
    position: relative;
  }

  .search-input-wrapper :global(.search-icon) {
    color: var(--muted-fg);
    font-size: 14px;
  }

  .search-input {
    flex: 1;
    padding: 8px 0;
    font-size: 15px;
    border: none;
    background: transparent;
    color: var(--fg);
    outline: none;
  }

  .search-input::placeholder {
    color: var(--muted-fg);
  }

  .loading-indicator {
    display: flex;
    align-items: center;
    gap: 6px;
    color: var(--accent-fg);
    font-size: 12px;
    background: var(--badge-bg);
    padding: 2px 8px;
    border-radius: 12px;
  }

  .scope-toggle {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
    background: var(--badge-bg);
    border: 1px solid transparent;
    border-radius: 6px;
    color: var(--muted-fg);
    font-size: 12px;
    cursor: pointer;
    transition: all 0.15s;
  }

  .scope-toggle:hover,
  .scope-toggle.active {
    background: var(--hover-bg);
    color: var(--fg);
    border-color: var(--border-color);
  }

  .scope-dropdown {
    padding: 12px 16px;
    border-bottom: 1px solid var(--border-color);
    background: rgba(0, 0, 0, 0.2);
  }

  .scope-label {
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--muted-fg);
    margin-bottom: 8px;
  }

  .drive-list {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-bottom: 12px;
  }

  .drive-option {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 10px;
    background: var(--badge-bg);
    border: 1px solid transparent;
    border-radius: 6px;
    color: var(--muted-fg);
    font-size: 12px;
    cursor: pointer;
    transition: all 0.15s;
  }

  .drive-option:hover {
    background: var(--hover-bg);
    color: var(--fg);
  }

  .drive-option.selected {
    background: var(--accent-bg);
    color: var(--accent-fg);
    border-color: var(--accent-fg);
  }

  .drive-path {
    opacity: 0.6;
    font-family: var(--font-mono);
    font-size: 11px;
  }

  .index-status-bar {
    font-size: 11px;
    color: var(--muted-fg);
    display: flex;
    align-items: center;
    gap: 8px;
    padding-top: 8px;
    border-top: 1px solid rgba(255, 255, 255, 0.05);
  }

  .status-val {
    text-transform: capitalize;
    font-weight: 500;
  }

  .status-val.scanning { color: var(--accent-fg); }
  .status-val.watching { color: #4ade80; }
  .status-val.error { color: #f87171; }

  .current-path {
    font-family: var(--font-mono);
    opacity: 0.5;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 300px;
  }

  .results {
    flex: 1;
    list-style: none;
    padding: 0;
    margin: 0;
    overflow-y: auto;
    min-height: 200px;
    max-height: 400px;
  }

  .result {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    padding: 10px 16px;
    background: none;
    border: none;
    font: inherit;
    color: var(--fg);
    cursor: pointer;
    text-align: left;
  }

  .result:hover,
  .result.selected {
    background: var(--hover-bg);
  }

  .result-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: 6px;
    background: var(--badge-bg);
    color: var(--muted-fg);
    font-size: 14px;
  }

  .result-icon.is-dir {
    color: var(--accent-fg);
  }

  .result-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .result-name {
    font-size: 14px;
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .result-path {
    font-size: 11px;
    color: var(--muted-fg);
    font-family: var(--font-mono);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .empty {
    padding: 40px 16px;
    text-align: center;
    color: var(--muted-fg);
    font-size: 14px;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px 20px;
    color: var(--muted-fg);
    gap: 12px;
  }

  .empty-state :global(.empty-icon) {
    opacity: 0.2;
    margin-bottom: 8px;
  }

  .index-info {
    font-size: 12px;
    opacity: 0.5;
  }

  .footer {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 16px;
    padding: 10px 16px;
    border-top: 1px solid var(--border-color);
    background: rgba(0, 0, 0, 0.2);
  }

  .hint {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    color: var(--muted-fg);
  }

  .key {
    padding: 2px 6px;
    background: var(--badge-bg);
    border-radius: 4px;
    font-family: var(--font-mono);
    font-size: 10px;
    color: var(--fg);
  }
</style>
