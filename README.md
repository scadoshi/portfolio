# scottyfermo.com

Personal portfolio site showcasing Rust projects. Built in Rust with Dioxus, compiled to WASM, deployed to GitHub Pages.

## Featured Projects

- **Zwipe** — Full-stack MTG deck builder (Axum + Dioxus + PostgreSQL, ~45,200 LOC)
- **Halo Action Importer** — Production bulk import tool for Halo Software
- **Halo Custom Field Builder** — CLI for bulk-creating custom fields via API

## Side Quests

- **Nighthawk** — LSM-tree KV store (WAL, memtable, SSTables, bloom filters, k-way compaction)
- **Diprotodon** — Redis-compatible in-memory KV server with hand-written RESP wire protocol
- **Marvin** — CLI chatbot built on the Rig AI framework
- **Capture** — V4L2 camera capture via raw Linux syscalls
- **Upsee** — Real-time pullup counter with MoveNet ML inference

## Stack

- [Dioxus](https://dioxuslabs.com/) 0.7.1 (Rust → WASM SPA, client-side routing)
- 14 color themes with dark/light variants (3 colorblind-friendly)
- Per-project media galleries (mp4 demos with controls)
- Syntax highlighting via highlight.js
- Auto-dismissing toast banners with progress line + hover-pause
- JetBrains Mono throughout
- Custom domain `scottyfermo.com` on GitHub Pages with GitHub Actions CI/CD

## Build

```
cargo install dioxus-cli --locked
dx serve            # dev server
dx build --release  # production build
```
