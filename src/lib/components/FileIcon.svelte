<script lang="ts">
  import Fa from 'svelte-fa';
  import {
    faFolder,
    faCode,
    faImage,
    faVideo,
    faMusic,
    faFileArchive,
    faTerminal,
    faDatabase,
    faFile
  } from '@fortawesome/free-solid-svg-icons';

  interface Props {
    type: string;
    className?: string;
  }

  let { type, className = '' }: Props = $props();

  const iconMap: Record<string, typeof faFolder> = {
    folder: faFolder,
    code: faCode,
    image: faImage,
    video: faVideo,
    audio: faMusic,
    archive: faFileArchive,
    executable: faTerminal,
    data: faDatabase,
    document: faFile
  };

  let icon = $derived(iconMap[type] || faFile);
</script>

<span class="file-icon {type} {className}">
  <Fa {icon} />
</span>

<style>
  .file-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    color: var(--icon-default);
    font-size: 16px;
  }

  /* Specific colors */
  .folder { color: var(--icon-folder); }
  .code { color: var(--icon-code); }
  .image { color: var(--icon-image); }
  .video { color: var(--icon-video); }
  .audio { color: var(--icon-audio); }
  .archive { color: var(--icon-archive); }
  .executable { color: var(--icon-executable); }
  .data { color: var(--icon-data); }
  .document { color: var(--icon-document); }
</style>
