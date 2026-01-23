# File Manager

A fast, modern dual-pane file manager built with Tauri, SvelteKit and TypeScript.

---

## Installation

### Windows

Download the latest release from the [Releases](../../releases) page:
- **MSI Installer:** `FileManager_x.x.x_x64_en-US.msi` — Recommended for most users
- **Portable:** `filemanager.exe` — No installation required

### Build from Source

```bash
# Requirements: Node.js, Rust, Bun
bun install
bun run tauri build
```

Build output is located at:
- `src-tauri/target/release/filemanager.exe`
- `src-tauri/target/release/bundle/msi/FileManager_x.x.x_x64_en-US.msi`

---

## User Guide

### Overview

File Manager is a powerful dual-pane file manager inspired by classic tools like Norton Commander and Total Commander. Navigate efficiently through your files using keyboard or mouse.

### Interface

```
┌─────────────────────────────────────────────────────────────┐
│  Sidebar            │  Left Pane         │  Right Pane      │
│  ────────────────── │  ──────────────────│  ────────────────│
│  📁 Bookmarks       │  📂 Documents      │  📂 Downloads    │
│  📁 WSL Distros     │  📄 report.pdf     │  🖼️ photo.jpg    │
│  🕐 Recent          │  📁 project        │  📦 backup.zip   │
│                     │                    │                  │
└─────────────────────────────────────────────────────────────┘
│  Status bar: /home/user/Documents  •  3 items selected      │
└─────────────────────────────────────────────────────────────┘
```

### Keyboard Shortcuts

#### Navigation

| Key | Action |
|-----|--------|
| `↑` `↓` | Move selection up/down |
| `Enter` | Open folder or execute file |
| `Backspace` | Go to parent folder |
| `Tab` | Switch between panes |
| `Home` / `End` | Jump to first/last item |
| `Ctrl+1` to `Ctrl+9` | Jump to bookmark |

#### Selection

| Key | Action |
|-----|--------|
| `Space` | Select/deselect item |
| `Shift+↑/↓` | Extend selection |
| `Ctrl+A` | Select all |
| Click | Select single item |
| `Ctrl+Click` | Add item to selection |
| `Shift+Click` | Select range |

#### File Operations

| Key | Action |
|-----|--------|
| `F2` | Rename |
| `F5` | Copy to other pane |
| `F6` | Move to other pane |
| `F7` | Create new folder |
| `F8` or `Delete` | Delete |
| `Ctrl+Z` | Undo |

#### Interface

| Key | Action |
|-----|--------|
| `Ctrl+P` | Open command palette |
| `Ctrl+L` | Enter path |
| `/` | Search/filter |
| `Escape` | Clear filter |
| `F1` | Show help |

### Features

#### Dual-Pane View
Work with two folders simultaneously. Ideal for copying or moving files between locations.

#### Bookmarks
Save frequently used folders as bookmarks. Accessible via the sidebar or with `Ctrl+1` to `Ctrl+9`.

#### WSL Integration
Direct access to your Windows Subsystem for Linux distributions from the sidebar.

#### Batch Rename
Select multiple files and rename them using patterns:
- `{name}` — original filename
- `{ext}` — extension
- `{n}` — sequence number
- `{date}` — current date

*Example:* `photo_{n}.{ext}` renames to `photo_1.jpg`, `photo_2.jpg`, etc.

#### Sorting
Click on column headers to sort by:
- Name
- Extension
- Size
- Modified

#### Command Palette
Press `Ctrl+P` for quick access to all commands. Type to search.

### Tips

- **Quick navigation:** Type `/` and start typing to filter files by name instantly.
- **Efficient copying:** Navigate to the source in one pane, to the destination in the other, select files and press `F5`.
- **Hidden files:** Toggle visibility via the command palette.

---

## Development

### Requirements

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://rustup.rs/)
- [Bun](https://bun.sh/) (or npm/pnpm)

### Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

### Commands

```bash
bun install          # Install dependencies
bun run dev          # Start development server
bun run tauri dev    # Start Tauri in development mode
bun run build        # Build frontend
bun run tauri build  # Build production release
bun run check        # Type-check
```

### Tech Stack

- **Frontend:** SvelteKit + TypeScript
- **Backend:** Tauri (Rust)
- **Styling:** Custom CSS
