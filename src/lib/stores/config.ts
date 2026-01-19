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

  return {
    subscribe,

    setConfig: (config: AppConfig) => set(config),

    addBookmark: (bookmark: Bookmark) => update(c => ({
      ...c,
      bookmarks: [...c.bookmarks, bookmark],
    })),

    removeBookmark: (path: string) => update(c => ({
      ...c,
      bookmarks: c.bookmarks.filter(b => b.path !== path),
    })),

    toggleHidden: () => update(c => ({
      ...c,
      showHidden: !c.showHidden,
    })),

    addRecentPath: (path: string) => update(c => {
      const filtered = c.recentPaths.filter(p => p !== path);
      return {
        ...c,
        recentPaths: [path, ...filtered].slice(0, 10),
      };
    }),
  };
}

export const config = createConfigStore();
