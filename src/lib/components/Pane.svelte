<script lang="ts">
  import { onMount } from 'svelte';
  import type { FileEntry } from '$lib/utils/ipc';
  import { readDirectory, getParentDirectory, openFile } from '$lib/utils/ipc';
  import type { SortColumn, PaneState } from '$lib/stores/panes';
  import PathBar from './PathBar.svelte';
  import FileList from './FileList.svelte';

  interface Props {
    pane: PaneState;
    selectedPaths: Set<string>;
    focusedIndex: number;
    active: boolean;
    showHidden: boolean;
    editingPath?: boolean;
    onPathChange: (path: string) => void;
    onEntriesLoaded: (entries: FileEntry[]) => void;
    onSelect: (entry: FileEntry, event: MouseEvent) => void;
    onFocus: () => void;
    onSort: (column: SortColumn) => void;
    onError: (error: string) => void;
    onEditPathEnd?: () => void;
  }

  let {
    pane,
    selectedPaths,
    focusedIndex,
    active,
    showHidden,
    editingPath = false,
    onPathChange,
    onEntriesLoaded,
    onSelect,
    onFocus,
    onSort,
    onError,
    onEditPathEnd,
  }: Props = $props();

  let containerRef: HTMLDivElement;

  async function loadDirectory(path: string) {
    try {
      const entries = await readDirectory(path);
      onEntriesLoaded(entries);
    } catch (e) {
      onError(String(e));
    }
  }

  async function handleNavigate(path: string) {
    onPathChange(path);
    await loadDirectory(path);
  }

  async function handleOpen(entry: FileEntry) {
    if (entry.is_dir) {
      await handleNavigate(entry.path);
    } else {
      try {
        await openFile(entry.path);
      } catch (e) {
        onError(String(e));
      }
    }
  }

  function handlePaneClick() {
    if (!active) {
      onFocus();
    }
  }

  $effect(() => {
    if (pane.path) {
      loadDirectory(pane.path);
    }
  });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_noninteractive_element_interactions -->
<div
  class="pane"
  class:active
  bind:this={containerRef}
  onclick={handlePaneClick}
  role="region"
>
  <PathBar path={pane.path} editing={editingPath} onNavigate={handleNavigate} onEditEnd={onEditPathEnd} />

  {#if pane.loading}
    <div class="loading">Loading...</div>
  {:else if pane.error}
    <div class="error">{pane.error}</div>
  {:else}
    <FileList
      entries={pane.entries}
      {selectedPaths}
      {focusedIndex}
      sortColumn={pane.sortColumn}
      sortDirection={pane.sortDirection}
      filter={pane.filter}
      {showHidden}
      {onSelect}
      onOpen={handleOpen}
      {onSort}
    />
  {/if}

  {#if pane.filter}
    <div class="filter-bar">
      Filter: {pane.filter}
    </div>
  {/if}
</div>

<style>
  .pane {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
    background: var(--pane-bg);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    overflow: hidden;
  }

  .pane.active {
    border-color: var(--focus-border);
  }

  .loading,
  .error {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
    color: var(--muted-fg);
  }

  .error {
    color: var(--error-fg);
  }

  .filter-bar {
    padding: 6px 12px;
    background: var(--accent-bg);
    border-top: 1px solid var(--border-color);
    font-size: 12px;
    color: var(--accent-fg);
  }
</style>
