<script lang="ts">
  import { onMount } from 'svelte';

  interface Props {
    open: boolean;
    title: string;
    label: string;
    value: string;
    onClose: () => void;
    onConfirm: (value: string) => void;
  }

  let { open, title, label, value = '', onClose, onConfirm }: Props = $props();

  let inputValue = $state(value);
  let inputElement = $state<HTMLInputElement>();

  $effect(() => {
    if (open) {
      inputValue = value;
      // Focus input when dialog opens
      setTimeout(() => {
        inputElement?.focus();
        inputElement?.select();
      }, 50);
    }
  });

  function handleConfirm() {
    if (inputValue.trim()) {
      onConfirm(inputValue);
      onClose();
    }
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      onClose();
    } else if (event.key === 'Enter') {
      handleConfirm();
    }
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_interactive_supports_focus -->
  <div class="overlay" onclick={onClose} onkeydown={handleKeyDown} role="dialog" tabindex="-1">
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="dialog" onclick={(e) => e.stopPropagation()} role="document">
      <h2 class="title">{title}</h2>

      <div class="form-group">
        <label for="input">{label}</label>
        <input
          id="input"
          type="text"
          bind:this={inputElement}
          bind:value={inputValue}
          placeholder={label}
          autocomplete="off"
          onkeydown={handleKeyDown}
        />
      </div>

      <div class="actions">
        <button class="cancel" onclick={onClose}>Cancel</button>
        <button class="confirm" onclick={handleConfirm} disabled={!inputValue.trim()}>OK</button>
      </div>
    </div>
  </div>
{/if}

<style>
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1100; /* Higher than other overlays if needed */
  }

  .dialog {
    width: 400px;
    max-width: 90vw;
    background: var(--palette-bg);
    border-radius: 8px;
    padding: 24px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    border: 1px solid var(--border-color);
  }

  .title {
    margin: 0 0 16px;
    font-size: 16px;
    font-weight: 600;
    color: var(--fg);
  }

  .form-group {
    margin-bottom: 24px;
  }

  .form-group label {
    display: block;
    margin-bottom: 8px;
    font-size: 13px;
    color: var(--muted-fg);
  }

  .form-group input {
    width: 100%;
    padding: 10px 12px;
    background: var(--bg);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--fg);
    font-family: inherit;
    font-size: 14px;
    outline: none;
  }

  .form-group input:focus {
    border-color: var(--accent-bg);
    box-shadow: 0 0 0 2px rgba(var(--accent-rgb), 0.2);
  }

  .actions {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
  }

  .actions button {
    padding: 8px 20px;
    border: none;
    border-radius: 4px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .cancel {
    background: var(--badge-bg);
    color: var(--fg);
  }

  .confirm {
    background: var(--accent-bg);
    color: var(--accent-fg);
  }

  .confirm:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .cancel:hover {
    background: var(--hover-bg);
  }

  .confirm:not(:disabled):hover {
    filter: brightness(1.1);
  }
</style>
