<script lang="ts">
  import { onMount } from 'svelte';
  import { contextMenu } from '$lib/stores/contextMenu';
  import { fly } from 'svelte/transition';

  let menuRef: HTMLDivElement;

  function handleClickOutside(event: MouseEvent) {
    if ($contextMenu.visible && menuRef && !menuRef.contains(event.target as Node)) {
      contextMenu.hide();
    }
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      contextMenu.hide();
    }
  }

  onMount(() => {
    window.addEventListener('click', handleClickOutside, true);
    window.addEventListener('contextmenu', handleClickOutside, true);
    window.addEventListener('keydown', handleKeyDown, true);
    return () => {
      window.removeEventListener('click', handleClickOutside, true);
      window.removeEventListener('contextmenu', handleClickOutside, true);
      window.removeEventListener('keydown', handleKeyDown, true);
    };
  });

  // Ensure menu stays within viewport
  let position = $derived(() => {
    if (!menuRef) return { x: $contextMenu.x, y: $contextMenu.y };
    
    const { innerWidth, innerHeight } = window;
    const { offsetWidth, offsetHeight } = menuRef;
    
    let x = $contextMenu.x;
    let y = $contextMenu.y;
    
    if (x + offsetWidth > innerWidth) x = innerWidth - offsetWidth - 5;
    if (y + offsetHeight > innerHeight) y = innerHeight - offsetHeight - 5;
    
    return { x, y };
  });

  function handleAction(action: (() => void) | undefined) {
    if (action) {
      action();
    }
    contextMenu.hide();
  }
</script>

{#if $contextMenu.visible}
  <div
    bind:this={menuRef}
    class="context-menu"
    style="left: {position().x}px; top: {position().y}px;"
    transition:fly={{ y: 5, duration: 100 }}
    role="menu"
    tabindex="-1"
  >
    {#each $contextMenu.items as item}
      {#if item.divider}
        <div class="divider"></div>
      {:else}
        <button
          class="menu-item"
          class:danger={item.danger}
          onclick={() => handleAction(item.action)}
          role="menuitem"
        >
          <span class="label">{item.label}</span>
          {#if item.shortcut}
            <span class="shortcut">{item.shortcut}</span>
          {/if}
        </button>
      {/if}
    {/each}
  </div>
{/if}

<style>
  .context-menu {
    position: fixed;
    z-index: 10000;
    min-width: 180px;
    background: var(--palette-bg);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
    padding: 4px;
    display: flex;
    flex-direction: column;
    outline: none;
  }

  .menu-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 8px 12px;
    border: none;
    background: none;
    color: var(--fg);
    font-size: 13px;
    text-align: left;
    cursor: pointer;
    border-radius: 4px;
    transition: background 0.1s;
  }

  .menu-item:hover {
    background: var(--hover-bg);
  }

  .menu-item.danger {
    color: var(--error-fg);
  }

  .menu-item.danger:hover {
    background: rgba(255, 68, 68, 0.1);
  }

  .label {
    flex: 1;
  }

  .shortcut {
    font-size: 11px;
    color: var(--muted-fg);
    font-family: var(--font-mono);
    opacity: 0.8;
  }

  .divider {
    height: 1px;
    background: var(--border-color);
    margin: 4px;
  }
</style>
