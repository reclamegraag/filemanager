<script lang="ts">
  interface Props {
    path: string;
    onNavigate: (path: string) => void;
  }

  let { path, onNavigate }: Props = $props();

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

  function handleClick(segment: PathSegment) {
    onNavigate(segment.path);
  }
</script>

<nav class="path-bar">
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
