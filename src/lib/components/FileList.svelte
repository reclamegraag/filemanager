<script lang="ts">
  import Fa from 'svelte-fa';
  import { faCaretUp, faCaretDown } from '@fortawesome/free-solid-svg-icons';
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
    onContextMenu: (entry: FileEntry, event: MouseEvent) => void;
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
    onContextMenu,
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

  function isSorted(column: SortColumn): boolean {
    return sortColumn === column;
  }
</script>

<div class="file-list">
  <div class="header" role="row">
    <span class="icon"></span>
    <button class="name" onclick={() => onSort('name')}>
      Name {#if isSorted('name')}<Fa icon={sortDirection === 'asc' ? faCaretUp : faCaretDown} />{/if}
    </button>
    <button class="extension" onclick={() => onSort('extension')}>
      Ext {#if isSorted('extension')}<Fa icon={sortDirection === 'asc' ? faCaretUp : faCaretDown} />{/if}
    </button>
    <button class="size" onclick={() => onSort('size')}>
      Size {#if isSorted('size')}<Fa icon={sortDirection === 'asc' ? faCaretUp : faCaretDown} />{/if}
    </button>
    <button class="modified" onclick={() => onSort('modified')}>
      Modified {#if isSorted('modified')}<Fa icon={sortDirection === 'asc' ? faCaretUp : faCaretDown} />{/if}
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
        {onContextMenu}
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
    grid-template-columns: 28px 1fr 64px 88px 130px;
    gap: 8px;
    padding: 8px 10px;
    background: var(--header-bg);
    border-bottom: 2px solid var(--border-color);
    font-size: 12px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--fg);
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
