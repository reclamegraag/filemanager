export type KeyAction =
  | 'navigate_up'
  | 'navigate_down'
  | 'navigate_left'
  | 'navigate_right'
  | 'enter_directory'
  | 'parent_directory'
  | 'switch_pane'
  | 'select_all'
  | 'toggle_selection'
  | 'extend_selection_up'
  | 'extend_selection_down'
  | 'first_item'
  | 'last_item'
  | 'page_up'
  | 'page_down'
  | 'copy'
  | 'move'
  | 'delete'
  | 'create_directory'
  | 'rename'
  | 'undo'
  | 'command_palette'
  | 'global_search'
  | 'start_filter'
  | 'clear_filter'
  | 'edit_path'
  | 'add_bookmark'
  | 'bookmark_1'
  | 'bookmark_2'
  | 'bookmark_3'
  | 'bookmark_4'
  | 'bookmark_5'
  | 'bookmark_6'
  | 'bookmark_7'
  | 'bookmark_8'
  | 'bookmark_9'
  | 'help';

export interface KeyBinding {
  key: string;
  ctrl?: boolean;
  shift?: boolean;
  alt?: boolean;
  action: KeyAction;
}

export const defaultKeyBindings: KeyBinding[] = [
  // Navigation
  { key: 'ArrowUp', action: 'navigate_up' },
  { key: 'ArrowDown', action: 'navigate_down' },
  { key: 'ArrowLeft', action: 'navigate_left' },
  { key: 'ArrowRight', action: 'navigate_right' },
  { key: 'Enter', action: 'enter_directory' },
  { key: 'Backspace', action: 'parent_directory' },
  { key: 'Tab', action: 'switch_pane' },
  { key: 'Home', action: 'first_item' },
  { key: 'End', action: 'last_item' },
  { key: 'Home', ctrl: true, action: 'first_item' },
  { key: 'End', ctrl: true, action: 'last_item' },
  { key: 'PageUp', action: 'page_up' },
  { key: 'PageDown', action: 'page_down' },

  // Selection
  { key: 'a', ctrl: true, action: 'select_all' },
  { key: ' ', action: 'toggle_selection' },
  { key: 'ArrowUp', shift: true, action: 'extend_selection_up' },
  { key: 'ArrowDown', shift: true, action: 'extend_selection_down' },

  // File operations
  { key: 'F5', action: 'copy' },
  { key: 'F6', action: 'move' },
  { key: 'F7', action: 'create_directory' },
  { key: 'F8', action: 'delete' },
  { key: 'Delete', action: 'delete' },
  { key: 'F2', action: 'rename' },
  { key: 'z', ctrl: true, action: 'undo' },

  // UI
  { key: 'p', ctrl: true, action: 'command_palette' },
  { key: 'F3', action: 'global_search' },
  { key: 'f', ctrl: true, shift: true, action: 'global_search' },
  { key: 'l', ctrl: true, action: 'edit_path' },
  { key: '/', action: 'start_filter' },
  { key: 'Escape', action: 'clear_filter' },
  { key: 'F1', action: 'help' },

  // Bookmarks
  { key: 'd', ctrl: true, action: 'add_bookmark' },
  { key: '1', ctrl: true, action: 'bookmark_1' },
  { key: '2', ctrl: true, action: 'bookmark_2' },
  { key: '3', ctrl: true, action: 'bookmark_3' },
  { key: '4', ctrl: true, action: 'bookmark_4' },
  { key: '5', ctrl: true, action: 'bookmark_5' },
  { key: '6', ctrl: true, action: 'bookmark_6' },
  { key: '7', ctrl: true, action: 'bookmark_7' },
  { key: '8', ctrl: true, action: 'bookmark_8' },
  { key: '9', ctrl: true, action: 'bookmark_9' },
];

export function matchKeyBinding(event: KeyboardEvent): KeyAction | null {
  for (const binding of defaultKeyBindings) {
    const keyMatch = event.key === binding.key;
    const ctrlMatch = !!binding.ctrl === (event.ctrlKey || event.metaKey);
    const shiftMatch = !!binding.shift === event.shiftKey;
    const altMatch = !!binding.alt === event.altKey;

    if (keyMatch && ctrlMatch && shiftMatch && altMatch) {
      return binding.action;
    }
  }

  return null;
}
