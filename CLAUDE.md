# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Session Startup

Bij het starten van een nieuwe sessie:
1. Activeer deze skills: `/king-mode`, `/frontend-design`, `/interview`
2. Bekijk de laatste 5 commits: `git log --oneline -5`

## Project Overview

Dual-pane Norton Commander-style file manager built with **SvelteKit 5** (frontend) and **Tauri 2** (Rust backend). The app runs as a desktop application on Windows with WSL integration support.

## Development Commands

```bash
bun install              # Install dependencies
bun run dev              # Frontend dev server only
bun run tauri dev        # Full desktop app with hot reload (recommended)
bun run tauri dev --debug # Debug mode with devtools
bun run check            # Type-check (svelte-check)
bun run tauri build      # Production build → src-tauri/target/release/
```

**No test framework configured** - gap identified for future setup (Vitest + Playwright recommended).

## Architecture

### Frontend (SvelteKit 5 + TypeScript)

```
src/
├── lib/
│   ├── components/     # Pane, FileList, FileRow, Sidebar, PathBar, StatusBar, etc.
│   ├── stores/         # panes.ts, selection.ts, config.ts, clipboard.ts, undo.ts
│   └── utils/
│       ├── ipc.ts      # Tauri command wrappers with web-compatible mocks
│       ├── keybindings.ts
│       └── formatters.ts
└── routes/+page.svelte # Main app component (~650 lines, orchestrates everything)
```

**Key patterns:**
- Uses Svelte 5 **runes**: `$state()`, `$props()`, `$derived()` (not old reactive syntax)
- Custom Svelte stores for state management (no external state library)
- `src/lib/utils/ipc.ts` abstracts Tauri commands with browser fallbacks for dev

### Backend (Rust + Tauri 2)

```
src-tauri/src/
├── commands/
│   ├── filesystem.rs   # read_directory, open_file, get_parent_directory
│   ├── operations.rs   # copy_files, move_files, delete_files, rename_file
│   ├── config.rs       # load_config, save_config
│   └── wsl.rs          # get_wsl_distros, wsl_copy
├── fs/                 # FileEntry, Config, AppError types
├── lib.rs              # Tauri app builder + command registration
└── main.rs             # Entry point
```

**Key patterns:**
- All filesystem ops are async (tokio)
- Error handling via `thiserror` with `AppError` enum
- Safe delete uses `trash` crate

### IPC Layer

Frontend calls Rust via `invoke()` from `@tauri-apps/api`. The `ipc.ts` wrapper provides:
- Type-safe command wrappers
- Mock implementations for browser development (`bun run dev`)
- Shared interfaces: `FileEntry`, `Config`, `AppError`, `UndoToken`

### State Flow

```
User Input → +page.svelte handlers → Store updates → Component re-renders
                    ↓
              Tauri invoke → Rust command → Filesystem → Response → Store update
```

## Code Conventions

### Svelte 5 Runes (IMPORTANT)

```svelte
<!-- Props -->
let { path, entries }: { path: string; entries: FileEntry[] } = $props();

<!-- State -->
let count = $state(0);
let items = $state<string[]>([]);

<!-- Derived -->
let doubled = $derived(count * 2);
```

### TypeScript

- Interfaces in `src/lib/utils/ipc.ts` for shared types
- Strict mode enabled
- camelCase for variables/functions, PascalCase for types

### Rust

- snake_case for functions, PascalCase for types
- All commands return `Result<T, AppError>`
- Async/await throughout

## Key Files

| File | Purpose |
|------|---------|
| `src/routes/+page.svelte` | Main app logic, keyboard handlers, file operations |
| `src/lib/utils/ipc.ts` | All Tauri command interfaces and browser mocks |
| `src/lib/stores/panes.ts` | Dual pane state (path, entries, sort, filter) |
| `src/lib/stores/selection.ts` | File selection state per pane |
| `src-tauri/src/commands/operations.rs` | Copy, move, delete, rename implementations |
| `src-tauri/capabilities/default.json` | Tauri security permissions |

## Styling

- Pure CSS (no framework) with CSS custom properties
- Dark theme: backgrounds `#0d0d0f` to `#1a1a1f`, text `#e8e6e3`
- Fonts: 'Outfit' (UI), 'JetBrains Mono' (monospace)
- Styles in `src/app.css`

## Features Reference

- **F5/F6**: Copy/Move to other pane
- **F7**: New folder
- **F8/Delete**: Delete (to trash)
- **F2**: Rename
- **Ctrl+P**: Command palette
- **Ctrl+D**: Add bookmark
- **Ctrl+1-9**: Jump to bookmark
- **Ctrl+Z**: Undo
- **Tab**: Switch panes
- **/**: Filter files
