<script lang="ts">
  import { onMount } from 'svelte';

  interface Command {
    id: string;
    label: string;
    shortcut?: string;
    action: () => void;
  }

  interface Props {
    open: boolean;
    commands: Command[];
    onClose: () => void;
  }

  let { open, commands, onClose }: Props = $props();

  let query = $state('');
  let selectedIndex = $state(0);
  let inputRef: HTMLInputElement;

  let filteredCommands = $derived(() => {
    if (!query) return commands;
    const lowerQuery = query.toLowerCase();
    return commands.filter(cmd => cmd.label.toLowerCase().includes(lowerQuery));
  });

  function handleKeyDown(event: KeyboardEvent) {
    const filtered = filteredCommands();

    switch (event.key) {
      case 'ArrowDown':
        event.preventDefault();
        selectedIndex = Math.min(selectedIndex + 1, filtered.length - 1);
        break;
      case 'ArrowUp':
        event.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, 0);
        break;
      case 'Enter':
        event.preventDefault();
        if (filtered[selectedIndex]) {
          filtered[selectedIndex].action();
          onClose();
        }
        break;
      case 'Escape':
        event.preventDefault();
        onClose();
        break;
    }
  }

  function selectCommand(cmd: Command) {
    cmd.action();
    onClose();
  }

  $effect(() => {
    if (open && inputRef) {
      query = '';
      selectedIndex = 0;
      inputRef.focus();
    }
  });
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_interactive_supports_focus -->
  <div class="overlay" onclick={onClose} role="dialog" tabindex="-1">
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="palette" onclick={(e) => e.stopPropagation()} role="document">
      <input
        bind:this={inputRef}
        bind:value={query}
        class="search"
        type="text"
        placeholder="Type a command..."
        onkeydown={handleKeyDown}
      />

      <ul class="commands">
        {#each filteredCommands() as cmd, i (cmd.id)}
          <li>
            <button
              class="command"
              class:selected={i === selectedIndex}
              onclick={() => selectCommand(cmd)}
            >
              <span class="label">{cmd.label}</span>
              {#if cmd.shortcut}
                <span class="shortcut">{cmd.shortcut}</span>
              {/if}
            </button>
          </li>
        {/each}

        {#if filteredCommands().length === 0}
          <li class="empty">No commands found</li>
        {/if}
      </ul>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: 100px;
    z-index: 1000;
  }

  .palette {
    width: 500px;
    max-width: 90vw;
    background: var(--palette-bg);
    border-radius: 8px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
    overflow: hidden;
  }

  .search {
    width: 100%;
    padding: 16px;
    font-size: 16px;
    border: none;
    background: transparent;
    color: var(--fg);
    outline: none;
  }

  .search::placeholder {
    color: var(--muted-fg);
  }

  .commands {
    list-style: none;
    padding: 0;
    margin: 0;
    max-height: 300px;
    overflow-y: auto;
    border-top: 1px solid var(--border-color);
  }

  .command {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    padding: 10px 16px;
    background: none;
    border: none;
    font: inherit;
    color: var(--fg);
    cursor: pointer;
    text-align: left;
  }

  .command:hover,
  .command.selected {
    background: var(--hover-bg);
  }

  .label {
    flex: 1;
  }

  .shortcut {
    font-size: 12px;
    color: var(--muted-fg);
    background: var(--badge-bg);
    padding: 2px 6px;
    border-radius: 4px;
    font-family: monospace;
  }

  .empty {
    padding: 16px;
    text-align: center;
    color: var(--muted-fg);
  }
</style>
