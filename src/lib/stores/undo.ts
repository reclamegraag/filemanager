import { writable } from 'svelte/store';

export interface UndoOperation {
  id: string;
  type: 'copy' | 'move' | 'delete' | 'rename' | 'create';
  description: string;
  timestamp: number;
  data: {
    paths?: string[];
    sourcePath?: string;
    destPath?: string;
    oldName?: string;
    newName?: string;
  };
}

function createUndoStore() {
  const { subscribe, set, update } = writable<UndoOperation[]>([]);

  return {
    subscribe,

    push: (operation: Omit<UndoOperation, 'id' | 'timestamp'>) => update(ops => [
      {
        ...operation,
        id: crypto.randomUUID(),
        timestamp: Date.now(),
      },
      ...ops,
    ].slice(0, 50)), // Keep last 50 operations

    pop: () => {
      let popped: UndoOperation | undefined;
      update(ops => {
        [popped, ...ops] = ops;
        return ops;
      });
      return popped;
    },

    clear: () => set([]),

    getLast: () => {
      let last: UndoOperation | undefined;
      subscribe(ops => last = ops[0])();
      return last;
    },
  };
}

export const undoStack = createUndoStore();
