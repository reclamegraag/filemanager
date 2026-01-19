const SIZE_UNITS = ['B', 'KB', 'MB', 'GB', 'TB'];

export function formatFileSize(bytes: number | null): string {
  if (bytes === null || bytes === undefined) return '';

  if (bytes === 0) return '0 B';

  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  const size = bytes / Math.pow(1024, i);

  return `${size.toFixed(i > 0 ? 1 : 0)} ${SIZE_UNITS[i]}`;
}

export function formatDate(timestamp: number | null): string {
  if (timestamp === null || timestamp === undefined) return '';

  const date = new Date(timestamp * 1000);
  const now = new Date();

  const isToday = date.toDateString() === now.toDateString();
  const isThisYear = date.getFullYear() === now.getFullYear();

  if (isToday) {
    return date.toLocaleTimeString(undefined, {
      hour: '2-digit',
      minute: '2-digit',
    });
  }

  if (isThisYear) {
    return date.toLocaleDateString(undefined, {
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  }

  return date.toLocaleDateString(undefined, {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  });
}

export function getFileIcon(entry: { is_dir: boolean; extension: string | null }): string {
  if (entry.is_dir) return '📁';

  const ext = entry.extension?.toLowerCase();

  const iconMap: Record<string, string> = {
    // Documents
    pdf: '📄',
    doc: '📝',
    docx: '📝',
    txt: '📄',
    md: '📄',
    rtf: '📄',

    // Spreadsheets
    xls: '📊',
    xlsx: '📊',
    csv: '📊',

    // Images
    jpg: '🖼️',
    jpeg: '🖼️',
    png: '🖼️',
    gif: '🖼️',
    svg: '🖼️',
    webp: '🖼️',
    ico: '🖼️',

    // Video
    mp4: '🎬',
    avi: '🎬',
    mkv: '🎬',
    mov: '🎬',
    webm: '🎬',

    // Audio
    mp3: '🎵',
    wav: '🎵',
    flac: '🎵',
    ogg: '🎵',

    // Archives
    zip: '📦',
    rar: '📦',
    '7z': '📦',
    tar: '📦',
    gz: '📦',

    // Code
    js: '⚡',
    ts: '⚡',
    jsx: '⚡',
    tsx: '⚡',
    py: '🐍',
    rs: '🦀',
    go: '🐹',
    java: '☕',
    html: '🌐',
    css: '🎨',
    scss: '🎨',
    json: '📋',
    xml: '📋',
    yaml: '📋',
    yml: '📋',
    toml: '📋',

    // Executables
    exe: '⚙️',
    msi: '⚙️',
    sh: '⚙️',
    bat: '⚙️',
    cmd: '⚙️',
  };

  return iconMap[ext || ''] || '📄';
}
