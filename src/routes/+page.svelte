<script lang="ts">
  import { onMount } from 'svelte';
  import '../app.css';
  import Pane from '$lib/components/Pane.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import StatusBar from '$lib/components/StatusBar.svelte';
  import CommandPalette from '$lib/components/CommandPalette.svelte';
  import HelpOverlay from '$lib/components/HelpOverlay.svelte';
  import BatchRenameDialog from '$lib/components/BatchRenameDialog.svelte';
  import { leftPane, rightPane, activePane } from '$lib/stores/panes';
  import { leftSelection, rightSelection } from '$lib/stores/selection';
  import { clipboard } from '$lib/stores/clipboard';
  import { config } from '$lib/stores/config';
  import type { FileEntry, WslDistro } from '$lib/utils/ipc';
  import {
    getHomeDirectory,
    getWslDistros,
    loadConfig,
    copyFiles,
    moveFiles,
    deleteFiles,
    createDirectory,
    renameFile,
    readDirectory,
  } from '$lib/utils/ipc';
  import { matchKeyBinding, type KeyAction } from '$lib/utils/keybindings';
  import { undoStack } from '$lib/stores/undo';
  import type { SortColumn } from '$lib/stores/panes';

  let wslDistros = $state<WslDistro[]>([]);
  let commandPaletteOpen = $state(false);
  let helpOpen = $state(false);
  let batchRenameOpen = $state(false);
  let showHidden = $state(false);
  let filterMode = $state(false);
  let editingPath = $state<'left' | 'right' | null>(null);
  let batchRenameFiles = $state<{ path: string; name: string }[]>([]);

  const commands = [
    { id: 'copy', label: 'Copy to other pane', shortcut: 'F5', action: () => handleCopy() },
    { id: 'move', label: 'Move to other pane', shortcut: 'F6', action: () => handleMove() },
    { id: 'mkdir', label: 'Create directory', shortcut: 'F7', action: () => handleCreateDir() },
    { id: 'delete', label: 'Delete', shortcut: 'F8', action: () => handleDelete() },
    { id: 'rename', label: 'Rename', shortcut: 'F2', action: () => handleRename() },
    { id: 'batch-rename', label: 'Batch rename', action: () => handleBatchRename() },
    { id: 'refresh', label: 'Refresh', action: () => handleRefresh() },
    { id: 'hidden', label: 'Toggle hidden files', action: () => showHidden = !showHidden },
    { id: 'help', label: 'Keyboard shortcuts', shortcut: 'F1', action: () => helpOpen = true },
  ];

  onMount(async () => {
    try {
      const home = await getHomeDirectory();
      if (home) {
        leftPane.setPath(home);
        rightPane.setPath(home);
      }

      wslDistros = await getWslDistros();

      const savedConfig = await loadConfig();
      config.setConfig({
        bookmarks: savedConfig.bookmarks.map(b => ({
          name: b.name,
          path: b.path,
          shortcut: b.shortcut,
        })),
        showHidden: savedConfig.show_hidden,
        recentPaths: savedConfig.recent_paths,
      });
    } catch (e) {
      console.error('Init error:', e);
    }
  });

  function handleKeyDown(event: KeyboardEvent) {
    // Handle filter mode input
    if (filterMode) {
      if (event.key === 'Escape') {
        filterMode = false;
        const paneStore = $activePane === 'left' ? leftPane : rightPane;
        paneStore.setFilter('');
        event.preventDefault();
        return;
      }

      if (event.key === 'Backspace') {
        const paneStore = $activePane === 'left' ? leftPane : rightPane;
        let state: typeof $leftPane;
        paneStore.subscribe(s => state = s)();
        paneStore.setFilter(state!.filter.slice(0, -1));
        event.preventDefault();
        return;
      }

      if (event.key.length === 1 && !event.ctrlKey && !event.metaKey) {
        const paneStore = $activePane === 'left' ? leftPane : rightPane;
        let state: typeof $leftPane;
        paneStore.subscribe(s => state = s)();
        paneStore.setFilter(state!.filter + event.key);
        event.preventDefault();
        return;
      }
    }

    const action = matchKeyBinding(event);
    if (!action) return;

    event.preventDefault();
    executeAction(action);
  }

  function executeAction(action: KeyAction) {
    const currentSelection = $activePane === 'left' ? leftSelection : rightSelection;
    const currentPaneStore = $activePane === 'left' ? leftPane : rightPane;
    let paneState: typeof $leftPane;
    currentPaneStore.subscribe(s => paneState = s)();

    switch (action) {
      case 'switch_pane':
        activePane.set($activePane === 'left' ? 'right' : 'left');
        break;

      case 'navigate_up': {
        let sel: { focusedIndex: number };
        currentSelection.subscribe(s => sel = s)();
        const newIndex = Math.max(0, sel!.focusedIndex - 1);
        currentSelection.setFocusedIndex(newIndex);
        if (paneState!.entries[newIndex]) {
          currentSelection.select(paneState!.entries[newIndex].path);
        }
        break;
      }

      case 'navigate_down': {
        let sel: { focusedIndex: number };
        currentSelection.subscribe(s => sel = s)();
        const newIndex = Math.min(paneState!.entries.length - 1, sel!.focusedIndex + 1);
        currentSelection.setFocusedIndex(newIndex);
        if (paneState!.entries[newIndex]) {
          currentSelection.select(paneState!.entries[newIndex].path);
        }
        break;
      }

      case 'first_item':
        currentSelection.setFocusedIndex(0);
        if (paneState!.entries[0]) {
          currentSelection.select(paneState!.entries[0].path);
        }
        break;

      case 'last_item': {
        const lastIndex = paneState!.entries.length - 1;
        currentSelection.setFocusedIndex(lastIndex);
        if (paneState!.entries[lastIndex]) {
          currentSelection.select(paneState!.entries[lastIndex].path);
        }
        break;
      }

      case 'select_all':
        currentSelection.selectAll(paneState!.entries.map(e => e.path));
        break;

      case 'command_palette':
        commandPaletteOpen = true;
        break;

      case 'clear_filter':
        currentPaneStore.setFilter('');
        filterMode = false;
        break;

      case 'start_filter':
        filterMode = true;
        break;

      case 'enter_directory': {
        let sel: { focusedIndex: number };
        currentSelection.subscribe(s => sel = s)();
        const entry = paneState!.entries[sel!.focusedIndex];
        if (entry?.is_dir) {
          currentPaneStore.setPath(entry.path);
        }
        break;
      }

      case 'parent_directory': {
        const parent = paneState!.path.split(/[/\\]/).slice(0, -1).join('/') || '/';
        currentPaneStore.setPath(parent);
        break;
      }

      case 'copy':
        handleCopy();
        break;

      case 'move':
        handleMove();
        break;

      case 'delete':
        handleDelete();
        break;

      case 'create_directory':
        handleCreateDir();
        break;

      case 'rename':
        handleRename();
        break;

      case 'help':
        helpOpen = !helpOpen;
        break;

      case 'edit_path':
        editingPath = $activePane;
        break;
    }
  }

  async function handleCopy() {
    const sourceSelection = $activePane === 'left' ? $leftSelection : $rightSelection;
    const sourcePaneState = $activePane === 'left' ? $leftPane : $rightPane;
    const destPaneState = $activePane === 'left' ? $rightPane : $leftPane;

    const paths = Array.from(sourceSelection.selectedPaths);
    if (paths.length === 0) return;

    try {
      await copyFiles(paths, destPaneState.path);
      undoStack.push({
        type: 'copy',
        description: `Copied ${paths.length} item(s)`,
        data: { paths, destPath: destPaneState.path },
      });
      await refreshPane($activePane === 'left' ? 'right' : 'left');
    } catch (e) {
      console.error('Copy error:', e);
    }
  }

  async function handleMove() {
    const sourceSelection = $activePane === 'left' ? $leftSelection : $rightSelection;
    const sourcePaneState = $activePane === 'left' ? $leftPane : $rightPane;
    const destPaneState = $activePane === 'left' ? $rightPane : $leftPane;

    const paths = Array.from(sourceSelection.selectedPaths);
    if (paths.length === 0) return;

    try {
      await moveFiles(paths, destPaneState.path);
      undoStack.push({
        type: 'move',
        description: `Moved ${paths.length} item(s)`,
        data: { paths, sourcePath: sourcePaneState.path, destPath: destPaneState.path },
      });
      await refreshBothPanes();
    } catch (e) {
      console.error('Move error:', e);
    }
  }

  async function handleDelete() {
    const selectionState = $activePane === 'left' ? $leftSelection : $rightSelection;
    const selectionStore = $activePane === 'left' ? leftSelection : rightSelection;
    const paths = Array.from(selectionState.selectedPaths);
    if (paths.length === 0) return;

    try {
      const token = await deleteFiles(paths);
      undoStack.push({
        type: 'delete',
        description: `Deleted ${paths.length} item(s)`,
        data: { paths },
      });
      selectionStore.clear();
      await refreshPane($activePane);
    } catch (e) {
      console.error('Delete error:', e);
    }
  }

  async function handleCreateDir() {
    const paneState = $activePane === 'left' ? $leftPane : $rightPane;
    const name = prompt('New directory name:');
    if (!name) return;

    try {
      await createDirectory(paneState.path, name);
      undoStack.push({
        type: 'create',
        description: `Created directory: ${name}`,
        data: { destPath: paneState.path },
      });
      await refreshPane($activePane);
    } catch (e) {
      console.error('Create directory error:', e);
    }
  }

  async function handleRename() {
    const selection = $activePane === 'left' ? $leftSelection : $rightSelection;
    const paneState = $activePane === 'left' ? $leftPane : $rightPane;

    const paths = Array.from(selection.selectedPaths);
    if (paths.length !== 1) return;

    const oldPath = paths[0];
    const oldName = oldPath.split(/[/\\]/).pop() || '';
    const newName = prompt('New name:', oldName);
    if (!newName || newName === oldName) return;

    try {
      await renameFile(oldPath, newName);
      undoStack.push({
        type: 'rename',
        description: `Renamed: ${oldName} → ${newName}`,
        data: { paths: [oldPath], oldName, newName },
      });
      await refreshPane($activePane);
    } catch (e) {
      console.error('Rename error:', e);
    }
  }

  async function handleRefresh() {
    await refreshBothPanes();
  }

  function handleBatchRename() {
    const selectionState = $activePane === 'left' ? $leftSelection : $rightSelection;
    const paneState = $activePane === 'left' ? $leftPane : $rightPane;

    const paths = Array.from(selectionState.selectedPaths);
    if (paths.length < 2) return;

    batchRenameFiles = paths.map(path => {
      const entry = paneState.entries.find(e => e.path === path);
      return { path, name: entry?.name || path.split(/[/\\]/).pop() || '' };
    });
    batchRenameOpen = true;
  }

  async function refreshPane(pane: 'left' | 'right') {
    const paneStore = pane === 'left' ? leftPane : rightPane;
    let state: typeof $leftPane;
    paneStore.subscribe(s => state = s)();

    try {
      const entries = await readDirectory(state!.path);
      paneStore.setEntries(entries);
    } catch (e) {
      console.error('Refresh error:', e);
    }
  }

  async function refreshBothPanes() {
    await Promise.all([refreshPane('left'), refreshPane('right')]);
  }

  function handleSelect(pane: 'left' | 'right', entry: FileEntry, event: MouseEvent) {
    const selection = pane === 'left' ? leftSelection : rightSelection;
    const paneStore = pane === 'left' ? leftPane : rightPane;

    if (event.ctrlKey || event.metaKey) {
      selection.toggle(entry.path);
    } else if (event.shiftKey) {
      let sel: { anchorIndex: number | null };
      selection.subscribe(s => sel = s)();

      let paneState: typeof $leftPane;
      paneStore.subscribe(s => paneState = s)();

      if (sel!.anchorIndex !== null) {
        const currentIndex = paneState!.entries.findIndex(e => e.path === entry.path);
        selection.selectRange(
          paneState!.entries.map(e => e.path),
          sel!.anchorIndex,
          currentIndex
        );
      } else {
        selection.select(entry.path);
      }
    } else {
      selection.select(entry.path);

      let paneState: typeof $leftPane;
      paneStore.subscribe(s => paneState = s)();
      const index = paneState!.entries.findIndex(e => e.path === entry.path);
      selection.setAnchorIndex(index);
      selection.setFocusedIndex(index);
    }

    if (pane !== $activePane) {
      activePane.set(pane);
    }
  }

  function handleNavigate(path: string) {
    const paneStore = $activePane === 'left' ? leftPane : rightPane;
    paneStore.setPath(path);
    config.addRecentPath(path);
  }
</script>

<svelte:window onkeydown={handleKeyDown} />

<div class="app">
  <Sidebar
    bookmarks={$config.bookmarks}
    {wslDistros}
    recentPaths={$config.recentPaths}
    onNavigate={handleNavigate}
  />

  <main class="main">
    <div class="panes">
      <Pane
        pane={$leftPane}
        selectedPaths={$leftSelection.selectedPaths}
        focusedIndex={$leftSelection.focusedIndex}
        active={$activePane === 'left'}
        {showHidden}
        editingPath={editingPath === 'left'}
        onPathChange={(path) => leftPane.setPath(path)}
        onEntriesLoaded={(entries) => leftPane.setEntries(entries)}
        onSelect={(entry, event) => handleSelect('left', entry, event)}
        onFocus={() => activePane.set('left')}
        onSort={(column) => leftPane.setSort(column)}
        onError={(error) => leftPane.setError(error)}
        onEditPathEnd={() => editingPath = null}
      />

      <Pane
        pane={$rightPane}
        selectedPaths={$rightSelection.selectedPaths}
        focusedIndex={$rightSelection.focusedIndex}
        active={$activePane === 'right'}
        {showHidden}
        editingPath={editingPath === 'right'}
        onPathChange={(path) => rightPane.setPath(path)}
        onEntriesLoaded={(entries) => rightPane.setEntries(entries)}
        onSelect={(entry, event) => handleSelect('right', entry, event)}
        onFocus={() => activePane.set('right')}
        onSort={(column) => rightPane.setSort(column)}
        onError={(error) => rightPane.setError(error)}
        onEditPathEnd={() => editingPath = null}
      />
    </div>

    <StatusBar
      leftPath={$leftPane.path}
      rightPath={$rightPane.path}
      selectedCount={$activePane === 'left' ? $leftSelection.selectedPaths.size : $rightSelection.selectedPaths.size}
      totalCount={$activePane === 'left' ? $leftPane.entries.length : $rightPane.entries.length}
      activePane={$activePane}
    />
  </main>

  <CommandPalette
    open={commandPaletteOpen}
    {commands}
    onClose={() => commandPaletteOpen = false}
  />

  <HelpOverlay
    open={helpOpen}
    onClose={() => helpOpen = false}
  />

  <BatchRenameDialog
    open={batchRenameOpen}
    files={batchRenameFiles}
    onClose={() => batchRenameOpen = false}
    onComplete={() => refreshPane($activePane)}
  />
</div>

<style>
  .app {
    display: flex;
    height: 100vh;
    overflow: hidden;
  }

  .main {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .panes {
    flex: 1;
    display: flex;
    gap: 4px;
    padding: 8px;
    min-height: 0;
  }
</style>
