import { writable } from 'svelte/store';

export interface ContextMenuItem {
  label: string;
  icon?: any;
  shortcut?: string;
  action: () => void;
  danger?: boolean;
  divider?: boolean;
}

export interface ContextMenuState {
  visible: boolean;
  x: number;
  y: number;
  items: ContextMenuItem[];
}

function createContextMenuStore() {
  const { subscribe, set, update } = writable<ContextMenuState>({
    visible: false,
    x: 0,
    y: 0,
    items: [],
  });

  return {
    subscribe,
    show: (x: number, y: number, items: ContextMenuItem[]) => {
      set({ visible: true, x, y, items });
    },
    hide: () => {
      update(s => ({ ...s, visible: false }));
    },
  };
}

export const contextMenu = createContextMenuStore();
