import { writable } from 'svelte/store';

export type ClipboardOperation = 'copy' | 'cut' | null;

export interface ClipboardState {
  paths: string[];
  operation: ClipboardOperation;
  sourcePath: string | null;
}

function createClipboardStore() {
  const { subscribe, set, update } = writable<ClipboardState>({
    paths: [],
    operation: null,
    sourcePath: null,
  });

  return {
    subscribe,

    copy: (paths: string[], sourcePath: string) => set({
      paths,
      operation: 'copy',
      sourcePath,
    }),

    cut: (paths: string[], sourcePath: string) => set({
      paths,
      operation: 'cut',
      sourcePath,
    }),

    clear: () => set({
      paths: [],
      operation: null,
      sourcePath: null,
    }),

    isEmpty: () => {
      let state: ClipboardState;
      subscribe(s => state = s)();
      return state!.paths.length === 0;
    },
  };
}

export const clipboard = createClipboardStore();
