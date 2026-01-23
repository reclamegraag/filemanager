import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { writable } from 'svelte/store';

export type IndexStatus = 'idle' | 'scanning' | 'watching' | 'error';

export interface IndexProgress {
  status: IndexStatus;
  indexed_count: number;
  current_path: string | null;
}

function createIndexStore() {
  const { subscribe, set, update } = writable<IndexProgress>({
    status: 'idle',
    indexed_count: 0,
    current_path: null,
  });

  return {
    subscribe,
    init: async () => {
      // Listen for progress events
      await listen<IndexProgress>('index:progress', (event) => {
        set(event.payload);
      });

      // Listen for status changes
      await listen<IndexStatus>('index:status', (event) => {
        update(s => ({ ...s, status: event.payload }));
      });

      // Get initial status
      try {
        const status = await invoke<IndexProgress>('get_index_status');
        set(status);
      } catch (e) {
        console.error('Failed to get index status:', e);
      }
    },
    reset: () => {
      set({
        status: 'idle',
        indexed_count: 0,
        current_path: null,
      });
    }
  };
}

export const indexStore = createIndexStore();
