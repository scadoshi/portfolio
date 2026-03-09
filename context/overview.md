# Portfolio

Personal portfolio site presenting six Rust projects. Built with Dioxus targeting WASM for web deployment. The site itself is a portfolio piece — a Rust developer's portfolio written in Rust.

## Goal

Present projects in a way that highlights what was learned, what was built, and why it matters. Not a resume — a technical showcase with interactive elements where possible.

## Tech Stack

| Crate | Purpose |
|-------|---------|
| `dioxus` | Cross-platform UI framework (targeting web/WASM) |
| `dioxus-router` | Client-side routing between project pages |
| `serde` | Serialization for project data |

### Later

| Crate | Purpose |
|-------|---------|
| `xterm-js-rs` | Embedded terminal emulator for interactive demos |
| `syntect` | Syntax highlighting for code snippets |
| `wasm-bindgen` | JS interop for terminal integration |

## Architecture

Dioxus WASM app. Same hexagonal-ish structure used in zwipe/zwiper:

```
src/
├── bin/main.rs              # Entry point, launches Dioxus app
└── lib/
    ├── lib.rs
    ├── domain/              # Project data, categories, metadata
    │   └── projects/        # Per-project content and descriptions
    ├── inbound/             # UI layer
    │   ├── router.rs        # Route definitions
    │   ├── screens/         # Page-level components (home, project detail, about)
    │   └── components/      # Reusable UI (project card, code block, nav, terminal)
    └── outbound/            # (minimal — static content, no backend needed)
```

## Design Direction

- Dark theme, monospace-forward — looks like it was built by someone who lives in the terminal
- Each project gets its own page with architecture overview, key code, and what was learned
- Interactive terminal demos where feasible (Nighthawk REPL is the easiest candidate for WASM)
- Mobile responsive — recruiters look at portfolios on phones

## Deployment

Static WASM bundle. Host on GitHub Pages, Cloudflare Pages, or Vercel. No backend required.

## Current State

Phase 0 — planning and content organization. No code yet.
