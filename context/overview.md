# Portfolio

Personal portfolio site presenting seven Rust projects. Built with Dioxus targeting WASM for web deployment. The site itself is a portfolio piece: a Rust developer's portfolio written in Rust.

## Goal

Present projects in a way that highlights what was learned, what was built, and why it matters. Not a resume. A technical showcase with interactive elements where possible.

## Tech Stack

| Crate | Purpose |
|-------|---------|
| `dioxus` 0.7.1 | Cross-platform UI framework (targeting web/WASM) |
| `dioxus-router` | Client-side routing between project pages |

### Later

| Crate | Purpose |
|-------|---------|
| `xterm-js-rs` | Embedded terminal emulator for interactive demos |
| `syntect` | Syntax highlighting for code snippets |
| `wasm-bindgen` | JS interop for terminal integration |

## Architecture

Dioxus WASM app. Flat module structure for simplicity:

```
src/
├── main.rs                 # Entry point, Router enum, App, NavbarLayout
├── data.rs                 # All project content as static data
├── theme.rs                # ThemeConfig, theme list, light/dark toggle logic
├── components/
│   ├── navbar.rs           # [SRF] brand, nav links, Projects dropdown
│   ├── project_card.rs     # Home page card with impact tooltip
│   ├── code_block.rs       # Terminal-styled code display
│   ├── theme_switcher.rs   # Theme dropdown + light/dark toggle
│   └── footer.rs           # GitHub, LinkedIn, Email
└── pages/
    ├── home.rs             # Hero + about + project cards grid
    ├── project_detail.rs   # Dynamic :slug lookup, full detail page
    └── side_quests.rs      # Marvin, Nighthawk, Upsee, Capture grouped
```

## Design Direction

- Terminal aesthetic with Iosevka Nerd Font, monospace everywhere
- 6 color themes: Gruvbox (default), Everforest, Catppuccin, Tokyo Night, Nord, Vantablack
- Each theme has dark/light variants (Vantablack dark-only)
- Theme switcher in navbar
- Each project gets its own page with objective, approach, implementation, obstacles, progress
- Mobile responsive

## Deployment

Static WASM bundle. Host on GitHub Pages, Cloudflare Pages, or Vercel. No backend required.

## Current State

Phase 1 in progress. Core site is built and compiles. All pages, routing, themes, and content are functional. Remaining: GIF/screenshot assets and deployment.
