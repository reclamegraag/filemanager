<script lang="ts">
  import type { FileEntry } from '$lib/utils/ipc';
  import type { SortColumn, SortDirection } from '$lib/stores/panes';
  import FileRow from './FileRow.svelte';

  interface Props {
    entries: FileEntry[];
    selectedPaths: Set<string>;
    focusedIndex: number;
    sortColumn: SortColumn;
    sortDirection: SortDirection;
    filter: string;
    showHidden: boolean;
    onSelect: (entry: FileEntry, event: MouseEvent) => void;
    onOpen: (entry: FileEntry) => void;
    onSort: (column: SortColumn) => void;
  }

  let {
    entries,
    selectedPaths,
    focusedIndex,
    sortColumn,
    sortDirection,
    filter,
    showHidden,
    onSelect,
    onOpen,
    onSort,
  }: Props = $props();

  let filteredEntries = $derived(() => {
    let result = entries;

    if (!showHidden) {
      result = result.filter(e => !e.is_hidden);
    }

    if (filter) {
      const lowerFilter = filter.toLowerCase();
      result = result.filter(e => e.name.toLowerCase().includes(lowerFilter));
    }

    return result;
  });

  let sortedEntries = $derived(() => {
    const sorted = [...filteredEntries()];

    sorted.sort((a, b) => {
      // Directories always first
      if (a.is_dir !== b.is_dir) {
        return a.is_dir ? -1 : 1;
      }

      let comparison = 0;

      switch (sortColumn) {
        case 'name':
          comparison = a.name.localeCompare(b.name, undefined, { sensitivity: 'base' });
          break;
        case 'size':
          comparison = (a.size || 0) - (b.size || 0);
          break;
        case 'modified':
          comparison = (a.modified || 0) - (b.modified || 0);
          break;
        case 'extension':
          comparison = (a.extension || '').localeCompare(b.extension || '');
          break;
      }

      return sortDirection === 'asc' ? comparison : -comparison;
    });

    return sorted;
  });

  function getSortIndicator(column: SortColumn): string {
    if (sortColumn !== column) return '';
    return sortDirection === 'asc' ? ' ▲' : ' ▼';
  }
</script>

<div class="file-list">
  <div class="header" role="row">
    <span class="icon"></span>
    <button class="name" onclick={() => onSort('name')}>
      Name{getSortIndicator('name')}
    </button>
    <button class="extension" onclick={() => onSort('extension')}>
      Ext{getSortIndicator('extension')}
    </button>
    <button class="size" onclick={() => onSort('size')}>
      Size{getSortIndicator('size')}
    </button>
    <button class="modified" onclick={() => onSort('modified')}>
      Modified{getSortIndicator('modified')}
    </button>
  </div>

  <div class="entries" role="grid">
    {#each sortedEntries() as entry, index (entry.path)}
      <FileRow
        {entry}
        selected={selectedPaths.has(entry.path)}
        focused={index === focusedIndex}
        {onSelect}
        {onOpen}
      />
    {/each}

    {#if sortedEntries().length === 0}
      <div class="empty">
        {#if filter}
          No matches for "{filter}"
        {:else}
          Empty directory
        {/if}
      </div>
    {/if}
  </div>
</div>

<style>
  .file-list {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .header {
    display: grid;
    grid-template-columns: 24px 1fr 60px 80px 120px;
    gap: 8px;
    padding: 6px 8px;
    background: var(--header-bg);
    border-bottom: 1px solid var(--border-color);
    font-size: 12px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--muted-fg);
  }

  .header button {
    background: none;
    border: none;
    padding: 0;
    font: inherit;
    color: inherit;
    cursor: pointer;
    text-align: left;
  }

  .header button:hover {
    color: var(--fg);
  }

  .header .size,
  .header .modified {
    text-align: right;
  }

  .entries {
    flex: 1;
    overflow-y: auto;
    padding: 4px 0;
  }

  .empty {
    padding: 24px;
    text-align: center;
    color: var(--muted-fg);
  }
</style>
