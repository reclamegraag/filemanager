<script lang="ts">
  import type { FileEntry } from '$lib/utils/ipc';
  import { formatFileSize, formatDate, getFileIcon } from '$lib/utils/formatters';

  interface Props {
    entry: FileEntry;
    selected: boolean;
    focused: boolean;
    onSelect: (entry: FileEntry, event: MouseEvent) => void;
    onOpen: (entry: FileEntry) => void;
  }

  let { entry, selected, focused, onSelect, onOpen }: Props = $props();

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
>
  <span class="icon">{getFileIcon(entry)}</span>
  <span class="name">{entry.name}</span>
  <span class="extension">{entry.extension || ''}</span>
  <span class="size">{entry.is_dir ? '' : formatFileSize(entry.size)}</span>
  <span class="modified">{formatDate(entry.modified)}</span>
</div>

<style>
  .file-row {
    display: grid;
    grid-template-columns: 24px 1fr 60px 80px 120px;
    gap: 8px;
    padding: 4px 8px;
    align-items: center;
    cursor: pointer;
    border-radius: 4px;
    font-size: 13px;
    transition: background-color 0.1s;
  }

  .file-row:hover {
    background: var(--hover-bg);
  }

  .file-row.selected {
    background: var(--selection-bg);
    color: var(--selection-fg);
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
