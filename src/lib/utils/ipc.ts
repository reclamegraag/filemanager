// Web-compatible IPC implementation
// Tauri v2: check for __TAURI_INTERNALS__ instead of __TAURI__
const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

// Mock data for web version
const mockFiles: FileEntry[] = [
  { name: 'Documents', path: '/home/user/Documents', extension: null, size: null, modified: Date.now(), is_dir: true, is_hidden: false, is_symlink: false },
  { name: 'Downloads', path: '/home/user/Downloads', extension: null, size: null, modified: Date.now(), is_dir: true, is_hidden: false, is_symlink: false },
  { name: 'Pictures', path: '/home/user/Pictures', extension: null, size: null, modified: Date.now(), is_dir: true, is_hidden: false, is_symlink: false },
  { name: 'README.md', path: '/home/user/README.md', extension: 'md', size: 1024, modified: Date.now(), is_dir: false, is_hidden: false, is_symlink: false },
  { name: 'script.js', path: '/home/user/script.js', extension: 'js', size: 2048, modified: Date.now(), is_dir: false, is_hidden: false, is_symlink: false },
];

const mockConfig: Config = {
  bookmarks: [
    { name: 'Home', path: '/home/user', shortcut: 1 },
    { name: 'Documents', path: '/home/user/Documents', shortcut: 2 },
  ],
  left_pane: { path: '/home/user', sort_column: 'name', sort_ascending: true },
  right_pane: { path: '/home/user', sort_column: 'name', sort_ascending: true },
  window: { x: null, y: null, width: 1200, height: 800, maximized: false },
  show_hidden: false,
  recent_paths: ['/home/user', '/home/user/Documents'],
};

async function invoke<T>(cmd: string, args?: any): Promise<T> {
  if (isTauri) {
    const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');
    return tauriInvoke<T>(cmd, args);
  }

  // Web fallback implementations
  switch (cmd) {
    case 'read_directory':
      return new Promise(resolve => {
        setTimeout(() => resolve(mockFiles as T), 100);
      });

    case 'get_home_directory':
      return new Promise(resolve => {
        setTimeout(() => resolve('/home/user' as T), 100);
      });

    case 'get_wsl_distros':
      return new Promise(resolve => {
        setTimeout(() => resolve([] as T), 100);
      });

    case 'load_config':
      return new Promise(resolve => {
        setTimeout(() => resolve(mockConfig as T), 100);
      });

    case 'save_config':
      return new Promise(resolve => {
        setTimeout(() => resolve(undefined as T), 100);
      });

    case 'copy_files':
    case 'move_files':
    case 'delete_files':
    case 'create_directory':
    case 'rename_file':
      console.log(`Mock operation: ${cmd}`, args);
      return new Promise(resolve => {
        setTimeout(() => resolve({} as T), 100);
      });

    case 'search_files':
      return new Promise(resolve => {
        const query = (args?.query || '').toLowerCase();
        const results = mockFiles.filter(f => f.name.toLowerCase().includes(query));
        setTimeout(() => resolve(results as T), 200);
      });

    case 'get_available_drives':
      return new Promise(resolve => {
        const drives = [
          { name: 'C: Drive', path: 'C:\\' },
          { name: 'D: Drive', path: 'D:\\' },
        ];
        setTimeout(() => resolve(drives as T), 100);
      });

    case 'start_indexing':
    case 'stop_indexing':
    case 'clear_index_cache':
      console.log(`Mock index operation: ${cmd}`, args);
      return new Promise(resolve => {
        setTimeout(() => resolve(undefined as T), 100);
      });

    case 'search_index':
      return new Promise(resolve => {
        const query = (args?.query || '').toLowerCase();
        const results = mockFiles.filter(f => f.name.toLowerCase().includes(query));
        setTimeout(() => resolve(results as T), 100);
      });

    case 'get_index_status':
      return new Promise(resolve => {
        const status = {
          status: 'idle',
          indexed_count: mockFiles.length,
          current_path: null
        };
        setTimeout(() => resolve(status as T), 100);
      });

    default:
      console.log(`Unknown command: ${cmd}`, args);
      return new Promise(resolve => {
        setTimeout(() => resolve({} as T), 100);
      });
  }
}

export interface FileEntry {
  name: string;
  path: string;
  extension: string | null;
  size: number | null;
  modified: number | null;
  is_dir: boolean;
  is_hidden: boolean;
  is_symlink: boolean;
}

export interface WslDistro {
  name: string;
  path: string;
  is_default: boolean;
}

export interface AppError {
  NotFound?: string;
  NotADirectory?: string;
  Io?: string;
  PermissionDenied?: string;
  Cancelled?: boolean;
  InvalidOperation?: string;
}

export interface UndoToken {
  id: string;
  operation: string;
  paths: string[];
  backup_paths: string[];
}

export interface DriveInfo {
  name: string;
  path: string;
}

export interface Config {
  bookmarks: Array<{
    name: string;
    path: string;
    shortcut: number | null;
  }>;
  left_pane: {
    path: string;
    sort_column: string;
    sort_ascending: boolean;
  };
  right_pane: {
    path: string;
    sort_column: string;
    sort_ascending: boolean;
  };
  window: {
    x: number | null;
    y: number | null;
    width: number;
    height: number;
    maximized: boolean;
  };
  show_hidden: boolean;
  recent_paths: string[];
}

// Filesystem commands
export async function readDirectory(path: string): Promise<FileEntry[]> {
  const normalizedPath = normalizeWslPath(path);
  return invoke<FileEntry[]>('read_directory', { path: normalizedPath });
}

export async function getParentDirectory(path: string): Promise<string | null> {
  return invoke<string | null>('get_parent_directory', { path });
}

export async function getHomeDirectory(): Promise<string | null> {
  return invoke<string | null>('get_home_directory');
}

export async function openFile(path: string): Promise<void> {
  return invoke<void>('open_file', { path });
}

export async function getFileInfo(path: string): Promise<FileEntry> {
  return invoke<FileEntry>('get_file_info', { path });
}

// File operations
export async function copyFiles(sources: string[], dest: string): Promise<void> {
  return invoke<void>('copy_files', { sources, dest });
}

export async function moveFiles(sources: string[], dest: string): Promise<void> {
  return invoke<void>('move_files', { sources, dest });
}

export async function deleteFiles(paths: string[]): Promise<UndoToken> {
  return invoke<UndoToken>('delete_files', { paths });
}

export async function createDirectory(parentPath: string, name: string): Promise<string> {
  return invoke<string>('create_directory', { parentPath, name });
}

export async function renameFile(path: string, newName: string): Promise<string> {
  return invoke<string>('rename_file', { path, newName });
}

// WSL commands
export async function getWslDistros(): Promise<WslDistro[]> {
  return invoke<WslDistro[]>('get_wsl_distros');
}

export async function wslCopy(
  source: string,
  dest: string,
  useWslNative: boolean
): Promise<void> {
  return invoke<void>('wsl_copy', {
    source,
    dest,
    useWslNative,
  });
}

// Config commands
export async function loadConfig(): Promise<Config> {
  return invoke<Config>('load_config');
}

export async function saveConfig(config: Config): Promise<void> {
  return invoke<void>('save_config', { config });
}

// WSL path normalization - converts /wsl$/... to \\wsl$\...
export function normalizeWslPath(path: string): string {
  // Converteer /wsl$/distro/... naar \\wsl$\distro\...
  if (path.startsWith('/wsl$/') || path.startsWith('/wsl$\\')) {
    return '\\\\wsl$\\' + path.slice(6).replace(/\//g, '\\');
  }
  // Converteer //wsl$/... naar \\wsl$\...
  if (path.startsWith('//wsl$/')) {
    return '\\\\wsl$\\' + path.slice(7).replace(/\//g, '\\');
  }
  return path;
}

// Search commands
export async function searchFiles(
  query: string,
  rootPaths: string[],
  limit?: number
): Promise<FileEntry[]> {
  return invoke<FileEntry[]>('search_files', {
    query,
    rootPaths,
    limit,
  });
}

export async function getAvailableDrives(): Promise<DriveInfo[]> {
  return invoke<DriveInfo[]>('get_available_drives');
}

// Indexer commands
export async function startIndexing(roots: string[]): Promise<void> {
  return invoke<void>('start_indexing', { roots });
}

export async function stopIndexing(): Promise<void> {
  return invoke<void>('stop_indexing');
}

export async function clearIndexCache(): Promise<void> {
  return invoke<void>('clear_index_cache');
}

export async function searchIndex(query: string, limit?: number): Promise<FileEntry[]> {
  return invoke<FileEntry[]>('search_index', { query, limit });
}

// Error parsing helper - converts AppError objects to readable strings
export function parseError(e: unknown): string {
  if (typeof e === 'string') return e;
  if (e && typeof e === 'object') {
    // AppError variants from Rust backend
    const err = e as Record<string, unknown>;
    if ('NotFound' in err) return `Path not found: ${err.NotFound}`;
    if ('NotADirectory' in err) return `Not a directory: ${err.NotADirectory}`;
    if ('Io' in err) return `IO error: ${err.Io}`;
    if ('PermissionDenied' in err) return `Permission denied: ${err.PermissionDenied}`;
    if ('Cancelled' in err) return 'Operation cancelled';
    if ('InvalidOperation' in err) return `Invalid operation: ${err.InvalidOperation}`;
    if ('message' in err) return String(err.message);
  }
  return 'Unknown error';
}
