import { writable, derived } from 'svelte/store';

export interface SelectionState {
  selectedPaths: Set<string>;
  focusedIndex: number;
  anchorIndex: number | null;
}

function createSelectionStore() {
  const { subscribe, set, update } = writable<SelectionState>({
    selectedPaths: new Set(),
    focusedIndex: 0,
    anchorIndex: null,
  });

  return {
    subscribe,

    select: (path: string) => update(s => {
      const newSet = new Set([path]);
      return { ...s, selectedPaths: newSet, anchorIndex: null };
    }),

    toggle: (path: string) => update(s => {
      const newSet = new Set(s.selectedPaths);
      if (newSet.has(path)) {
        newSet.delete(path);
      } else {
        newSet.add(path);
      }
      return { ...s, selectedPaths: newSet };
    }),

    addToSelection: (path: string) => update(s => {
      const newSet = new Set(s.selectedPaths);
      newSet.add(path);
      return { ...s, selectedPaths: newSet };
    }),

    selectRange: (paths: string[], fromIndex: number, toIndex: number) => update(s => {
      const start = Math.min(fromIndex, toIndex);
      const end = Math.max(fromIndex, toIndex);
      const newSet = new Set(paths.slice(start, end + 1));
      return { ...s, selectedPaths: newSet };
    }),

    selectAll: (paths: string[]) => update(s => ({
      ...s,
      selectedPaths: new Set(paths),
    })),

    clear: () => update(s => ({
      ...s,
      selectedPaths: new Set(),
      anchorIndex: null,
    })),

    setFocusedIndex: (index: number) => update(s => ({
      ...s,
      focusedIndex: index,
    })),

    setAnchorIndex: (index: number | null) => update(s => ({
      ...s,
      anchorIndex: index,
    })),
  };
}

export const leftSelection = createSelectionStore();
export const rightSelection = createSelectionStore();

export function getSelectionStore(pane: 'left' | 'right') {
  return pane === 'left' ? leftSelection : rightSelection;
}
