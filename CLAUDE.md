# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

SimPhy is a 2D physics simulation application built with Bevy 0.18.0, avian2d physics engine, and egui for UI. It simulates magnetic/electric fields and rigid body physics.

## Commands

```bash
# Build and run
cargo build
cargo build --release
cargo run
cargo run --features dev

# Code quality
cargo check
cargo fmt
cargo fmt -- --check
```

## Architecture

The codebase follows a plugin-based Bevy architecture:

- **`src/main.rs`** - Entry point, registers all plugins
- **`src/app/`** - App state and simulation control (start/pause/reset)
- **`src/simulation/`** - Physics simulation, field calculations (magnetic, electric, Coulomb)
- **`src/editor/`** - Editor state, entity selection, clipboard
- **`src/ui/`** - UI panels (toolbar, inspector, status bar) using egui
- **`src/camera/`** - Camera controls (pan, zoom)
- **`src/project/`** - Project file import/export (YAML/JSON)
- **`src/input/`** - Input action handling
- **`src/settings/`** - Editor preferences
- **`src/shared/`** - Shared types (EntityShape, theme)

## Code Style

- **No comments** unless explicitly requested
- **PascalCase** for types/components, **snake_case** for functions/variables
- **Modular**: use `pub mod` and `pub use` for exports
- **Plugin-based**: group functionality into Bevy plugins
- **Imports**: external crates first, then `super::` for siblings, then `crate::`
- **Error handling**: `Result<T, String>` with `map_err` for context

## Dependencies

- `bevy` 0.18.0 - Game engine
- `avian2d` 0.5 - 2D physics
- `bevy_egui` 0.39.1 - UI
- `bevy_enhanced_input` 0.24 - Input handling
- `serde`/`serde_yaml`/`ron` - Serialization

## Notes

- There are no test files currently
- The project uses OpenSpec workflow in `.opencode/` for change management
- Development tasks are tracked in `todo.md`
