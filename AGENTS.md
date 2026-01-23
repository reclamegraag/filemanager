# AGENTS.md - File Manager Development Guide

## Project Overview
Dual-pane file manager built with **SvelteKit 5** + **Tauri 2**. Frontend uses modern Svelte 5 runes syntax, backend uses Rust with async filesystem operations.

## Development Commands

### Frontend (SvelteKit)
```bash
bun run dev          # Start development server
bun run build        # Build for production  
bun run preview      # Preview production build
bun run check        # Type check with svelte-check
bun run check:watch  # Type check in watch mode
```

### Tauri (Desktop App)
```bash
bun run tauri dev    # Start development with hot reload
bun run tauri build  # Build production desktop app
bun run tauri dev --debug  # Debug mode
```

### Testing
**No test framework currently configured** - this is a gap that should be addressed. Recommended setup:
- Frontend: `bun add -D vitest @testing-library/svelte`
- Backend: Use Rust's built-in `#[cfg(test)]` modules
- E2E: `bun add -D @playwright/test`

## Code Style Guidelines

### TypeScript/Svelte Conventions

#### Modern Svelte 5 Syntax
- Use **runes** (`$props()`, `$state()`, `$derived()`) instead of old Svelte syntax
- Explicit props typing: `let { name, count }: { name: string; count: number } = $props()`
- State management: `let value = $state(initial)` for reactive state

#### Interface-First Development
- Define comprehensive TypeScript interfaces for all data structures
- Use `export interface` for shared types in `src/lib/utils/ipc.ts`
- Example: `FileEntry`, `Config`, `AppError`, `UndoToken`

#### Import Organization
```typescript
// External dependencies
import { onMount } from 'svelte';
import { writable, derived } from 'svelte/store';

// Internal imports (grouped by type)
import Pane from '$lib/components/Pane.svelte';
import { leftPane, rightPane } from '$lib/stores/panes';
import { readDirectory, type FileEntry } from '$lib/utils/ipc';
```

#### Component Patterns
- Functional components with explicit props
- Store-based state management using custom store creators
- Web-compatible fallbacks for Tauri APIs (see `src/lib/utils/ipc.ts`)

#### Naming Conventions
- **Variables/Functions**: `camelCase`
- **Types/Interfaces**: `PascalCase` 
- **Files**: `kebab-case` (mostly)
- **Stores**: `camelCase` with descriptive names

### Rust Conventions

#### Module Organization
```
src-tauri/src/
├── commands/     # Tauri command handlers
├── fs/          # Filesystem operations  
├── lib.rs       # Main Tauri app setup
```

#### Error Handling
- Use `thiserror` for custom error types
- Return `Result<T, E>` for all operations
- Async/await throughout for filesystem operations

#### Naming
- **Functions/Variables**: `snake_case`
- **Types**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`

## Architecture Patterns

### IPC Abstraction Layer
- `src/lib/utils/ipc.ts` provides web-compatible fallbacks
- Mock implementations for development in browser
- Unified interface for Tauri commands

### Store-Based State Management
- Custom Svelte stores for app state (`panes.ts`, `selection.ts`, `config.ts`)
- Derived stores for computed values
- Store creators for reusable patterns

### Component Composition
- Modular Svelte components with explicit props
- Clear separation of concerns
- Reusable utility functions

## File Structure
```
src/
├── lib/
│   ├── components/     # Svelte components
│   ├── stores/         # Svelte stores for state management
│   └── utils/          # Utility functions and IPC layer
└── routes/             # SvelteKit routes

src-tauri/src/
├── commands/           # Tauri command handlers
├── fs/                 # Filesystem operations
└── lib.rs              # Main Tauri app setup
```

## Development Workflow

### Environment Setup
- **VS Code** with Svelte, Tauri, and Rust Analyzer extensions
- **Bun** as package manager
- **Hot reload** during development via Vite + Tauri

### Code Quality
- **Type checking** via `svelte-check` with TypeScript integration
- **Strict TypeScript** enabled
- **SPA mode** with static adapter (no SSR)

## Key Dependencies

### Frontend
- `@tauri-apps/api` - Tauri frontend API
- `@sveltejs/kit` - SvelteKit framework
- `svelte` - Svelte 5 with runes

### Backend  
- `tauri` - Main Tauri framework
- `tokio` - Async runtime
- `serde` - JSON serialization
- `walkdir`, `trash`, `dirs` - Filesystem operations

## Special Considerations

### Web Compatibility
- All Tauri operations have web fallbacks
- Mock data for development in browser
- Progressive enhancement approach

### Performance
- Async operations throughout
- Efficient state management with Svelte stores
- Minimal re-renders through proper reactive patterns

### Security
- Tauri capabilities in `src-tauri/capabilities/default.json`
- Proper file operation permissions
- Safe IPC communication patterns

## Global Skills Integration

This project automatically loads the **King Mode** skill from `~/.config/opencode/skills/king-mode.md` which provides:
- Dutch language conversation
- Senior Frontend Architect expertise
- "ULTRATHINK" protocol for deep analysis
- Intentional minimalism design philosophy
- Modern frontend coding standards

When working on this codebase, follow the King Mode protocols for optimal code quality and design decisions.

## Skill Auto-Loading Configuration

The King Mode skill is automatically loaded at the start of each new chat session. This ensures consistent behavior and adherence to the established coding standards and design philosophy throughout all development work on this project.