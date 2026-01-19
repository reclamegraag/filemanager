<script lang="ts">
  interface Props {
    path: string;
    editing?: boolean;
    onNavigate: (path: string) => void;
    onEditEnd?: () => void;
  }

  let { path, editing = false, onNavigate, onEditEnd }: Props = $props();

  let inputValue = $state(path);
  let inputRef: HTMLInputElement;

  interface PathSegment {
    name: string;
    path: string;
  }

  let segments = $derived((): PathSegment[] => {
    if (!path) return [];

    const parts = path.split(/[/\\]/).filter(Boolean);
    const result: PathSegment[] = [];

    // Handle Windows drive letters
    if (path.match(/^[A-Z]:/i)) {
      let currentPath = parts[0] + '\\';
      result.push({ name: parts[0], path: currentPath });

      for (let i = 1; i < parts.length; i++) {
        currentPath += parts[i] + '\\';
        result.push({ name: parts[i], path: currentPath.slice(0, -1) });
      }
    } else {
      // Unix paths
      let currentPath = '/';
      result.push({ name: '/', path: '/' });

      for (const part of parts) {
        currentPath += part + '/';
        result.push({ name: part, path: currentPath.slice(0, -1) });
      }
    }

    return result;
  });

  $effect(() => {
    if (editing && inputRef) {
      inputValue = path;
      inputRef.focus();
      inputRef.select();
    }
  });

  function handleClick(segment: PathSegment) {
    onNavigate(segment.path);
  }

  function handleInputKeyDown(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      event.preventDefault();
      if (inputValue.trim()) {
        onNavigate(inputValue.trim());
      }
      onEditEnd?.();
    } else if (event.key === 'Escape') {
      event.preventDefault();
      inputValue = path;
      onEditEnd?.();
    }
  }

  function handleInputBlur() {
    onEditEnd?.();
  }
</script>

<nav class="path-bar">
  {#if editing}
    <input
      bind:this={inputRef}
      bind:value={inputValue}
      class="path-input"
      type="text"
      onkeydown={handleInputKeyDown}
      onblur={handleInputBlur}
      spellcheck="false"
    />
  {:else}
    {#each segments() as segment, i}
      {#if i > 0}
        <span class="separator">/</span>
      {/if}
      <button
        class="segment"
        class:current={i === segments().length - 1}
        onclick={() => handleClick(segment)}
      >
        {segment.name}
      </button>
    {/each}
  {/if}
</nav>

<style>
  .path-bar {
    display: flex;
    align-items: center;
    gap: 2px;
    padding: 8px 12px;
    background: var(--pathbar-bg);
    border-bottom: 1px solid var(--border-color);
    font-size: 13px;
    overflow-x: auto;
    white-space: nowrap;
  }

  .path-input {
    flex: 1;
    background: var(--input-bg);
    border: 1px solid var(--focus-border);
    border-radius: 4px;
    padding: 4px 8px;
    font: inherit;
    color: var(--fg);
    outline: none;
  }

  .separator {
    color: var(--muted-fg);
    user-select: none;
  }

  .segment {
    background: none;
    border: none;
    padding: 4px 6px;
    border-radius: 4px;
    font: inherit;
    color: var(--link-fg);
    cursor: pointer;
    transition: background-color 0.1s;
  }

  .segment:hover {
    background: var(--hover-bg);
  }

  .segment.current {
    color: var(--fg);
    font-weight: 500;
  }
</style>
