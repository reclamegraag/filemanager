<script lang="ts">
  import { onMount } from 'svelte';

  const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

  let {
    theme = 'dark',
    onToggleTheme = () => {},
    onToggleFullscreen = () => {}
  }: {
    theme?: 'light' | 'dark';
    onToggleTheme?: () => void;
    onToggleFullscreen?: () => void;
  } = $props();

  let appWindow: any;
  let isFullscreen = $state(false);

  onMount(async () => {
    if (isTauri) {
      const { getCurrentWindow } = await import('@tauri-apps/api/window');
      appWindow = getCurrentWindow();
      isFullscreen = await appWindow.isFullscreen();
    }
  });

  const minimize = () => {
    if (appWindow) appWindow.minimize();
  };

  const close = () => {
    if (appWindow) appWindow.close();
  };

  const toggleFullscreen = async () => {
    if (appWindow) {
      const currentFullscreen = await appWindow.isFullscreen();

      if (!currentFullscreen) {
        // Going to fullscreen: first unmaximize if maximized (Windows fix)
        const isMaximized = await appWindow.isMaximized();
        if (isMaximized) {
          await appWindow.unmaximize();
          // Small delay to let Windows process the unmaximize
          await new Promise(resolve => setTimeout(resolve, 50));
        }
        await appWindow.setFullscreen(true);
        isFullscreen = true;
      } else {
        // Exiting fullscreen: go back to maximized state
        await appWindow.setFullscreen(false);
        await appWindow.maximize();
        isFullscreen = false;
      }
    }
    onToggleFullscreen();
  };
</script>

<div data-tauri-drag-region class="titlebar">
  <div class="title">FileManager</div>

  <div class="controls">
    <!-- Theme toggle -->
    <button onclick={onToggleTheme} aria-label="Toggle theme" title="Toggle theme (Light/Dark)">
      {#if theme === 'dark'}
        <!-- Sun icon for switching to light -->
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="5"/>
          <path d="M12 1v2M12 21v2M4.22 4.22l1.42 1.42M18.36 18.36l1.42 1.42M1 12h2M21 12h2M4.22 19.78l1.42-1.42M18.36 5.64l1.42-1.42"/>
        </svg>
      {:else}
        <!-- Moon icon for switching to dark -->
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
        </svg>
      {/if}
    </button>

    {#if isTauri}
      <!-- Fullscreen toggle -->
      <button onclick={toggleFullscreen} aria-label="Toggle fullscreen" title="Toggle fullscreen (F11)">
        {#if isFullscreen}
          <!-- Exit fullscreen icon -->
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M8 3v3a2 2 0 0 1-2 2H3M21 8h-3a2 2 0 0 1-2-2V3M3 16h3a2 2 0 0 1 2 2v3M16 21v-3a2 2 0 0 1 2-2h3"/>
          </svg>
        {:else}
          <!-- Enter fullscreen icon -->
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M8 3H5a2 2 0 0 0-2 2v3M21 8V5a2 2 0 0 0-2-2h-3M3 16v3a2 2 0 0 0 2 2h3M16 21h3a2 2 0 0 0 2-2v-3"/>
          </svg>
        {/if}
      </button>

      <button onclick={minimize} aria-label="Minimize" title="Minimize">
        <svg width="10" height="1" viewBox="0 0 10 1"><rect fill="currentColor" width="10" height="1"/></svg>
      </button>
      <button onclick={close} class="close" aria-label="Close" title="Close">
        <svg width="10" height="10" viewBox="0 0 10 10"><path fill="none" d="M1 1L9 9M9 1L1 9" stroke="currentColor" stroke-width="1.2"/></svg>
      </button>
    {/if}
  </div>
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
