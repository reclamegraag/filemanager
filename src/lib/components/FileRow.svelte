<script lang="ts">
  import type { FileEntry } from '$lib/utils/ipc';
  import FileIcon from './FileIcon.svelte';
  import { formatFileSize, formatDate, getFileIconType } from '$lib/utils/formatters';

  interface Props {
    entry: FileEntry;
    selected: boolean;
    focused: boolean;
    onSelect: (entry: FileEntry, event: MouseEvent) => void;
    onOpen: (entry: FileEntry) => void;
    onContextMenu: (entry: FileEntry, event: MouseEvent) => void;
  }

  let { entry, selected, focused, onSelect, onOpen, onContextMenu }: Props = $props();

  let rowRef: HTMLDivElement;

  $effect(() => {
    if (focused && rowRef) {
      rowRef.scrollIntoView({ block: 'nearest' });
    }
  });

  function handleClick(event: MouseEvent) {
    onSelect(entry, event);
  }

  function handleDoubleClick() {
    onOpen(entry);
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      onOpen(entry);
    }
  }
</script>

<div
  bind:this={rowRef}
  class="file-row"
  class:selected
  class:focused
  class:hidden={entry.is_hidden}
  class:directory={entry.is_dir}
  role="row"
  tabindex={focused ? 0 : -1}
  onclick={handleClick}
  ondblclick={handleDoubleClick}
  onkeydown={handleKeyDown}
  oncontextmenu={(e) => onContextMenu(entry, e)}
>
  <div class="icon"><FileIcon type={getFileIconType(entry)} /></div>
  <span class="name">{entry.name}</span>
  <span class="extension">{entry.extension || ''}</span>
  <span class="size">{entry.is_dir ? '' : formatFileSize(entry.size)}</span>
  <span class="modified">{formatDate(entry.modified)}</span>
</div>

<style>
  .file-row {
    display: grid;
    grid-template-columns: 28px 1fr 64px 88px 130px;
    gap: 8px;
    padding: 6px 10px;
    align-items: center;
    cursor: pointer;
    border-radius: 4px;
    font-size: 13px;
    transition: background-color 0.1s;
  }

  .file-row:hover:not(.selected) {
    background: var(--hover-bg);
    opacity: 0.7;
  }

  .file-row.selected {
    background: var(--selection-bg);
    color: var(--selection-fg);
  }

  .file-row.selected .extension,
  .file-row.selected .size,
  .file-row.selected .modified {
    color: var(--selection-muted);
  }

  .file-row.selected:hover {
    background: var(--selection-bg);
  }

  .file-row.focused {
    outline: 1px solid var(--focus-border);
    outline-offset: -1px;
  }

  .file-row.hidden {
    opacity: 0.6;
  }

  .file-row.directory .name {
    font-weight: 500;
  }

  .icon {
    font-size: 16px;
    text-align: center;
  }

  .name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .extension {
    color: var(--muted-fg);
    text-transform: uppercase;
    font-size: 11px;
  }

  .size {
    text-align: right;
    color: var(--muted-fg);
    font-variant-numeric: tabular-nums;
  }

  .modified {
    text-align: right;
    color: var(--muted-fg);
    font-size: 12px;
  }
</style>
