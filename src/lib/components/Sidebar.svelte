<script lang="ts">
  import type { Bookmark } from '$lib/stores/config';
  import type { WslDistro } from '$lib/utils/ipc';

  interface Props {
    bookmarks: Bookmark[];
    wslDistros: WslDistro[];
    recentPaths: string[];
    onNavigate: (path: string) => void;
  }

  let { bookmarks, wslDistros, recentPaths, onNavigate }: Props = $props();
</script>

<aside class="sidebar">
  <section class="section">
    <h3 class="section-title">Bookmarks</h3>
    <ul class="list">
      {#each bookmarks as bookmark}
        <li>
          <button class="item" onclick={() => onNavigate(bookmark.path)}>
            <span class="icon">⭐</span>
            <span class="label">{bookmark.name}</span>
            {#if bookmark.shortcut}
              <span class="shortcut">Ctrl+{bookmark.shortcut}</span>
            {/if}
          </button>
        </li>
      {/each}
    </ul>
  </section>

  {#if wslDistros.length > 0}
    <section class="section">
      <h3 class="section-title">WSL</h3>
      <ul class="list">
        {#each wslDistros as distro}
          <li>
            <button class="item" onclick={() => onNavigate(distro.path)}>
              <span class="icon">🐧</span>
              <span class="label">{distro.name}</span>
              {#if distro.is_default}
                <span class="badge">Default</span>
              {/if}
            </button>
          </li>
        {/each}
      </ul>
    </section>
  {/if}

  {#if recentPaths.length > 0}
    <section class="section">
      <h3 class="section-title">Recent</h3>
      <ul class="list">
        {#each recentPaths.slice(0, 5) as path}
          <li>
            <button class="item" onclick={() => onNavigate(path)}>
              <span class="icon">📁</span>
              <span class="label">{path.split(/[/\\]/).pop()}</span>
            </button>
          </li>
        {/each}
      </ul>
    </section>
  {/if}
</aside>

<style>
  .sidebar {
    width: 200px;
    background: var(--sidebar-bg);
    border-right: 1px solid var(--border-color);
    padding: 12px 0;
    overflow-y: auto;
  }

  .section {
    margin-bottom: 16px;
  }

  .section-title {
    padding: 0 12px;
    margin: 0 0 8px;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: var(--muted-fg);
  }

  .list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 6px 12px;
    background: none;
    border: none;
    font: inherit;
    font-size: 13px;
    color: var(--fg);
    cursor: pointer;
    text-align: left;
  }

  .item:hover {
    background: var(--hover-bg);
  }

  .icon {
    flex-shrink: 0;
    font-size: 14px;
  }

  .label {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .shortcut {
    font-size: 10px;
    color: var(--muted-fg);
    background: var(--badge-bg);
    padding: 2px 4px;
    border-radius: 3px;
  }

  .badge {
    font-size: 9px;
    color: var(--accent-fg);
    background: var(--accent-bg);
    padding: 2px 4px;
    border-radius: 3px;
    text-transform: uppercase;
  }
</style>
