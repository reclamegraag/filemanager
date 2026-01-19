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
    reset: () => set({
      path: initialPath,
      entries: [],
      loading: false,
      error: null,
      sortColumn: 'name',
      sortDirection: 'asc',
      filter: '',
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
