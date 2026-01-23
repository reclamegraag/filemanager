<script lang="ts">
  import Fa from 'svelte-fa';
  import { faTimes } from '@fortawesome/free-solid-svg-icons';

  interface Props {
    open: boolean;
    onClose: () => void;
  }

  let { open, onClose }: Props = $props();

  function handleKeyDown(event: KeyboardEvent) {
    if (open && event.key === 'Escape') {
      event.preventDefault();
      onClose();
    }
  }


  const shortcuts = [
    { category: 'Navigation', items: [
      { key: '↑/↓', action: 'Move selection up/down' },
      { key: 'Enter', action: 'Open directory / file' },
      { key: 'Backspace', action: 'Go to parent directory' },
      { key: 'Tab', action: 'Switch between panes' },
      { key: 'Home/End', action: 'Jump to first/last item' },
    ]},
    { category: 'Selection', items: [
      { key: 'Space', action: 'Toggle selection' },
      { key: 'Shift+↑/↓', action: 'Extend selection' },
      { key: 'Ctrl+A', action: 'Select all' },
    ]},
    { category: 'File Operations', items: [
      { key: 'F5', action: 'Copy to other pane' },
      { key: 'F6', action: 'Move to other pane' },
      { key: 'F7', action: 'Create directory' },
      { key: 'F8 / Delete', action: 'Delete (to trash)' },
      { key: 'F2', action: 'Rename' },
      { key: 'Ctrl+Z', action: 'Undo' },
    ]},
    { category: 'UI', items: [
      { key: 'Ctrl+P', action: 'Command palette' },
      { key: 'F3', action: 'Global search' },
      { key: 'Ctrl+L', action: 'Edit path' },
      { key: '/', action: 'Start filter' },
      { key: 'Escape', action: 'Clear filter' },
      { key: 'Ctrl+1-9', action: 'Jump to bookmark' },
      { key: 'F1', action: 'Show this help' },
    ]},
  ];
</script>

<svelte:window onkeydown={handleKeyDown} />

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_interactive_supports_focus -->
  <div class="overlay" onclick={onClose} role="dialog" tabindex="-1">

    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="help" onclick={(e) => e.stopPropagation()} role="document">
      <header class="header">
        <h2>Keyboard Shortcuts</h2>
        <button class="close" onclick={onClose}><Fa icon={faTimes} /></button>
      </header>

      <div class="content">
        {#each shortcuts as section}
          <section class="section">
            <h3>{section.category}</h3>
            <ul>
              {#each section.items as item}
                <li>
                  <span class="key">{item.key}</span>
                  <span class="action">{item.action}</span>
                </li>
              {/each}
            </ul>
          </section>
        {/each}
      </div>

      <footer class="footer">
        Press <span class="key">Esc</span> or <span class="key">F1</span> to close
      </footer>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.7);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .help {
    width: 600px;
    max-width: 90vw;
    max-height: 80vh;
    background: var(--palette-bg);
    border-radius: 12px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border-color);
  }

  .header h2 {
    margin: 0;
    font-size: 16px;
    font-weight: 600;
  }

  .close {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: none;
    font-size: 20px;
    color: var(--muted-fg);
    cursor: pointer;
    border-radius: 4px;
  }

  .close:hover {
    background: var(--hover-bg);
    color: var(--fg);
  }

  .content {
    flex: 1;
    overflow-y: auto;
    padding: 16px 20px;
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 20px;
  }

  .section h3 {
    margin: 0 0 8px;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--accent-fg);
  }

  .section ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .section li {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 6px 0;
  }

  .key {
    min-width: 80px;
    padding: 3px 6px;
    background: var(--badge-bg);
    border-radius: 4px;
    font-size: 11px;
    font-family: var(--font-mono);
    color: var(--fg);
    text-align: center;
  }

  .action {
    font-size: 13px;
    color: var(--muted-fg);
  }

  .footer {
    padding: 12px 20px;
    border-top: 1px solid var(--border-color);
    text-align: center;
    font-size: 12px;
    color: var(--muted-fg);
  }

  .footer .key {
    min-width: auto;
    display: inline-block;
  }
</style>
