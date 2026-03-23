# scottyfermo.com

Personal portfolio site showcasing Rust projects. Built with Dioxus (Rust WASM framework).

## Projects

- **Zwipe** - Full-stack MTG deck builder (Axum + Dioxus + PostgreSQL)
- **Halo Action Importer** - Production bulk import tool for Halo Software
- **Halo Custom Field Builder** - CLI for bulk-creating custom fields via API
- **Marvin** - CLI chatbot built on the Rig AI framework
- **Nighthawk** - LSM-tree key-value database with TCP server and concurrency
- **Upsee** - Real-time pullup counter with MoveNet ML inference
- **Capture** - V4L2 camera capture via raw Linux syscalls

## Stack

- [Dioxus](https://dioxuslabs.com/) 0.7.1 (WASM)
- 8 color themes with light/dark variants
- Syntax highlighting via highlight.js
- Deployed to GitHub Pages

## Build

```
cargo install dioxus-cli
dx serve    # dev server
dx build --release  # production build
```
