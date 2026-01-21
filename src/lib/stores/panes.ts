import { writable, derived } from 'svelte/store';
import type { FileEntry } from '$lib/utils/ipc';

export type SortColumn = 'name' | 'size' | 'modified' | 'extension';
export type SortDirection = 'asc' | 'desc';

export interface PaneState {
  path: string;
  entries: FileEntry[];
  loading: boolean;
  error: string | null;
  sortColumn: SortColumn;
  sortDirection: SortDirection;
  filter: string;
  showHidden: boolean;
}

export function getSortedEntries(state: PaneState): FileEntry[] {
  let result = state.entries;

  // Filter hidden
  if (!state.showHidden) {
    result = result.filter(e => !e.is_hidden);
  }

  // Filter by search
  if (state.filter) {
    const lowerFilter = state.filter.toLowerCase();
    result = result.filter(e => e.name.toLowerCase().includes(lowerFilter));
  }

  // Sort
  const sorted = [...result];
  sorted.sort((a, b) => {
    // Directories always first
    if (a.is_dir !== b.is_dir) {
      return a.is_dir ? -1 : 1;
    }

    let comparison = 0;
    switch (state.sortColumn) {
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

    return state.sortDirection === 'asc' ? comparison : -comparison;
  });

  return sorted;
}

function createPaneStore(initialPath: string) {
  const { subscribe, set, update } = writable<PaneState>({
    path: initialPath,
    entries: [],
    loading: false,
    error: null,
    sortColumn: 'name',
    sortDirection: 'asc',
    filter: '',
    showHidden: false,
  });

  return {
    subscribe,
    setPath: (path: string) => update(s => ({ ...s, path, filter: '' })),
    setEntries: (entries: FileEntry[]) => update(s => ({ ...s, entries, loading: false, error: null })),
    setLoading: (loading: boolean) => update(s => ({ ...s, loading })),
    setError: (error: string | null) => update(s => ({ ...s, error, loading: false })),
    setSort: (column: SortColumn) => update(s => ({
      ...s,
      sortColumn: column,
      sortDirection: s.sortColumn === column && s.sortDirection === 'asc' ? 'desc' : 'asc',
    })),
    setFilter: (filter: string) => update(s => ({ ...s, filter })),
    setShowHidden: (showHidden: boolean) => update(s => ({ ...s, showHidden })),
    reset: () => set({
      path: initialPath,
      entries: [],
      loading: false,
      error: null,
      sortColumn: 'name',
      sortDirection: 'asc',
      filter: '',
      showHidden: false,
    }),
  };
}

export const leftPane = createPaneStore('');
export const rightPane = createPaneStore('');
export const activePane = writable<'left' | 'right'>('left');

export const currentPane = derived(
  [activePane, leftPane, rightPane],
  ([$activePane, $leftPane, $rightPane]) =>
    $activePane === 'left' ? $leftPane : $rightPane
);

export function getActiveStore() {
  let active: 'left' | 'right' = 'left';
  activePane.subscribe(v => active = v)();
  return active === 'left' ? leftPane : rightPane;
}

export function getInactiveStore() {
  let active: 'left' | 'right' = 'left';
  activePane.subscribe(v => active = v)();
  return active === 'left' ? rightPane : leftPane;
}
