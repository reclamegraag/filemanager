<script lang="ts">
  import { onMount } from 'svelte';
  
  const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
  
  let appWindow: any;

  onMount(async () => {
    if (isTauri) {
      const { getCurrentWindow } = await import('@tauri-apps/api/window');
      appWindow = getCurrentWindow();
    }
  });

  const minimize = () => {
    if (appWindow) appWindow.minimize();
  };

  const toggleMaximize = () => {
    if (appWindow) appWindow.toggleMaximize();
  };

  const close = () => {
    if (appWindow) appWindow.close();
  };
</script>

<div data-tauri-drag-region class="titlebar">
  <div class="title">FileManager</div>
  
  {#if isTauri}
    <div class="controls">
      <button onclick={minimize} aria-label="Minimize" title="Minimize">
        <svg width="10" height="1" viewBox="0 0 10 1"><rect fill="currentColor" width="10" height="1"/></svg>
      </button>
      <button onclick={toggleMaximize} aria-label="Maximize" title="Maximize">
        <svg width="10" height="10" viewBox="0 0 10 10"><rect fill="none" x="0.5" y="0.5" width="9" height="9" stroke="currentColor"/></svg>
      </button>
      <button onclick={close} class="close" aria-label="Close" title="Close">
        <svg width="10" height="10" viewBox="0 0 10 10"><path fill="none" d="M1 1L9 9M9 1L1 9" stroke="currentColor" stroke-width="1.2"/></svg>
      </button>
    </div>
  {/if}
</div>

<style>
  .titlebar {
    height: 32px;
    background: var(--bg);
    display: flex;
    justify-content: space-between;
    align-items: center;
    user-select: none;
    border-bottom: 1px solid var(--border-color);
    position: relative;
    z-index: 1000;
  }

  .title {
    padding-left: 12px;
    font-size: 12px;
    font-weight: 500;
    color: var(--muted-fg);
    pointer-events: none;
  }

  .controls {
    display: flex;
    height: 100%;
  }

  button {
    width: 46px;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    color: var(--fg);
    cursor: default;
    transition: background-color 0.1s;
  }

  button:hover {
    background: var(--hover-bg);
  }

  button.close:hover {
    background: #e81123;
    color: white;
  }
</style>
