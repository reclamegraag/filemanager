<script lang="ts">
  import { renameFile } from '$lib/utils/ipc';

  interface Props {
    open: boolean;
    files: { path: string; name: string }[];
    onClose: () => void;
    onComplete: () => void;
  }

  let { open, files, onClose, onComplete }: Props = $props();

  let pattern = $state('{name}');
  let preview = $state<{ original: string; renamed: string }[]>([]);

  $effect(() => {
    if (open && files.length > 0) {
      generatePreview();
    }
  });

  function generatePreview() {
    preview = files.map((file, index) => {
      const ext = file.name.includes('.') ? file.name.split('.').pop() || '' : '';
      const nameWithoutExt = file.name.replace(/\.[^.]+$/, '');
      const date = new Date().toISOString().split('T')[0];

      let renamed = pattern
        .replace('{name}', nameWithoutExt)
        .replace('{ext}', ext)
        .replace('{n}', String(index + 1).padStart(2, '0'))
        .replace('{date}', date);

      if (ext && !renamed.includes('.') && pattern.includes('{name}')) {
        renamed += '.' + ext;
      }

      return { original: file.name, renamed };
    });
  }

  async function handleRename() {
    try {
      for (let i = 0; i < files.length; i++) {
        const file = files[i];
        const newName = preview[i].renamed;
        if (newName !== file.name) {
          await renameFile(file.path, newName);
        }
      }
      onComplete();
      onClose();
    } catch (e) {
      console.error('Batch rename error:', e);
    }
  }

  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      onClose();
    } else if (event.key === 'Enter' && event.ctrlKey) {
      handleRename();
    }
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_interactive_supports_focus -->
  <div class="overlay" onclick={onClose} onkeydown={handleKeyDown} role="dialog" tabindex="-1">
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="dialog" onclick={(e) => e.stopPropagation()} role="document">
      <h2 class="title">Batch Rename</h2>

      <div class="form-group">
        <label for="pattern">Pattern:</label>
        <input
          id="pattern"
          type="text"
          bind:value={pattern}
          oninput={generatePreview}
          placeholder={"{name}_{n}"}
        />
        <div class="hint">
          Available: {'{name}'}, {'{ext}'}, {'{n}'} (number), {'{date}'}
        </div>
      </div>

      <div class="preview">
        <h3>Preview ({files.length} files)</h3>
        <ul class="preview-list">
          {#each preview.slice(0, 10) as item}
            <li>
              <span class="original">{item.original}</span>
              <span class="arrow">→</span>
              <span class="renamed">{item.renamed}</span>
            </li>
          {/each}
          {#if files.length > 10}
            <li class="more">... and {files.length - 10} more</li>
          {/if}
        </ul>
      </div>

      <div class="actions">
        <button class="cancel" onclick={onClose}>Cancel</button>
        <button class="confirm" onclick={handleRename}>Rename All</button>
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
    z-index: 1000;
  }

  .dialog {
    width: 500px;
    max-width: 90vw;
    max-height: 80vh;
    background: var(--palette-bg);
    border-radius: 8px;
    padding: 24px;
    overflow-y: auto;
  }

  .title {
    margin: 0 0 16px;
    font-size: 18px;
    font-weight: 600;
  }

  .form-group {
    margin-bottom: 16px;
  }

  .form-group label {
    display: block;
    margin-bottom: 4px;
    font-size: 13px;
    color: var(--muted-fg);
  }

  .form-group input {
    width: 100%;
    padding: 8px 12px;
    background: var(--bg);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    color: var(--fg);
    font-family: var(--font-mono);
  }

  .hint {
    margin-top: 4px;
    font-size: 11px;
    color: var(--muted-fg);
  }

  .preview {
    margin-bottom: 16px;
  }

  .preview h3 {
    margin: 0 0 8px;
    font-size: 13px;
    color: var(--muted-fg);
  }

  .preview-list {
    list-style: none;
    padding: 0;
    margin: 0;
    max-height: 200px;
    overflow-y: auto;
    background: var(--bg);
    border-radius: 4px;
    padding: 8px;
  }

  .preview-list li {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 4px 0;
    font-size: 12px;
    font-family: var(--font-mono);
  }

  .original {
    color: var(--muted-fg);
  }

  .arrow {
    color: var(--accent-fg);
  }

  .renamed {
    color: var(--success-fg);
  }

  .more {
    color: var(--muted-fg);
    font-style: italic;
  }

  .actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  .actions button {
    padding: 8px 16px;
    border: none;
    border-radius: 4px;
    font-size: 13px;
    cursor: pointer;
  }

  .cancel {
    background: var(--badge-bg);
    color: var(--fg);
  }

  .confirm {
    background: var(--accent-bg);
    color: var(--accent-fg);
  }

  .cancel:hover {
    background: var(--hover-bg);
  }

  .confirm:hover {
    background: var(--selection-bg);
    color: var(--selection-fg);
  }
</style>
