<script lang="ts">
  interface Props {
    leftPath: string;
    rightPath: string;
    selectedCount: number;
    totalCount: number;
    activePane: 'left' | 'right';
  }

  let { leftPath, rightPath, selectedCount, totalCount, activePane }: Props = $props();
</script>

<footer class="status-bar">
  <div class="section left">
    <span class="pane-indicator" class:active={activePane === 'left'}>L</span>
    <span class="path">{leftPath}</span>
  </div>

  <div class="section center">
    {#if selectedCount > 0}
      <span class="selection">{selectedCount} selected</span>
    {:else}
      <span class="count">{totalCount} items</span>
    {/if}
  </div>

  <div class="section right">
    <span class="path">{rightPath}</span>
    <span class="pane-indicator" class:active={activePane === 'right'}>R</span>
  </div>
</footer>

<style>
  .status-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 12px;
    background: var(--statusbar-bg);
    border-top: 1px solid var(--border-color);
    font-size: 12px;
    color: var(--muted-fg);
  }

  .section {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .section.left,
  .section.right {
    flex: 1;
    min-width: 0;
  }

  .section.right {
    justify-content: flex-end;
  }

  .section.center {
    flex-shrink: 0;
  }

  .pane-indicator {
    width: 18px;
    height: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 3px;
    background: var(--badge-bg);
    font-size: 10px;
    font-weight: 600;
  }

  .pane-indicator.active {
    background: var(--accent-bg);
    color: var(--accent-fg);
  }

  .path {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .selection {
    color: var(--accent-fg);
  }
</style>
