import { writable } from 'svelte/store';

export interface Bookmark {
  name: string;
  path: string;
  shortcut: number | null;
}

export interface AppConfig {
  bookmarks: Bookmark[];
  showHidden: boolean;
  recentPaths: string[];
}

function createConfigStore() {
  const { subscribe, set, update } = writable<AppConfig>({
    bookmarks: [],
    showHidden: false,
    recentPaths: [],
  });

  let saveCallback: ((config: AppConfig) => Promise<void>) | null = null;

  const triggerSave = async (config: AppConfig) => {
    console.log('[Config Store] triggerSave called, bookmarks:', config.bookmarks.length);
    if (saveCallback) {
      try {
        await saveCallback(config);
        console.log('[Config Store] Save completed successfully');
      } catch (e) {
        console.error('[Config Store] Save failed:', e);
      }
    } else {
      console.warn('[Config Store] No saveCallback registered yet');
    }
  };

  return {
    subscribe,

    setSaveCallback: (callback: (config: AppConfig) => Promise<void>) => {
      saveCallback = callback;
    },

    setConfig: (config: AppConfig) => set(config),

    addBookmark: (bookmark: Bookmark) => update(c => {
      const newConfig = {
        ...c,
        bookmarks: [...c.bookmarks, bookmark],
      };
      triggerSave(newConfig);
      return newConfig;
    }),

    removeBookmark: (path: string) => update(c => {
      const newConfig = {
        ...c,
        bookmarks: c.bookmarks.filter(b => b.path !== path),
      };
      triggerSave(newConfig);
      return newConfig;
    }),

    toggleHidden: () => update(c => {
      const newConfig = {
        ...c,
        showHidden: !c.showHidden,
      };
      triggerSave(newConfig);
      return newConfig;
    }),

    addRecentPath: (path: string) => update(c => {
      const filtered = c.recentPaths.filter(p => p !== path);
      const newConfig = {
        ...c,
        recentPaths: [path, ...filtered].slice(0, 10),
      };
      triggerSave(newConfig);
      return newConfig;
    }),
  };
}

export const config = createConfigStore();
