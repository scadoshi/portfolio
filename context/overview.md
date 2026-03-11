# Portfolio

Personal portfolio site presenting seven Rust projects. Built with Dioxus targeting WASM for web deployment. The site itself is a portfolio piece: a Rust developer's portfolio written in Rust.

## Goal

Present projects in a way that highlights what was learned, what was built, and why it matters. Not a resume. A technical showcase with interactive elements where possible.

## Tech Stack

| Crate | Purpose |
|-------|---------|
| `dioxus` 0.7.1 | Cross-platform UI framework (targeting web/WASM) |
| `dioxus-router` | Client-side routing between project pages |
| `gloo-timers` 0.3 | Async delays for highlight.js integration in WASM |

### Later

| Crate | Purpose |
|-------|---------|
| `xterm-js-rs` | Embedded terminal emulator for interactive demos |
| `wasm-bindgen` | JS interop for terminal integration |

## Architecture

Dioxus WASM SPA. Flat module structure:

```
src/
├── main.rs                 # Entry point, Router enum, App, NavbarLayout
├── data.rs                 # All project content as static data (Project struct, ProjectType enum)
├── theme.rs                # ThemeConfig, 8 themes, light/dark toggle logic
├── components/
│   ├── navbar.rs           # [SRF] brand, nav links, Projects dropdown
│   ├── project_card.rs     # Home page card with impact tooltip
│   ├── code_block.rs       # Syntax-highlighted code via highlight.js + unique IDs
│   ├── linked_text.rs      # Auto-detects URLs in text and renders as <a> tags
│   ├── theme_switcher.rs   # Theme dropdown + light/dark toggle
│   └── footer.rs           # GitHub, LinkedIn, Email
└── pages/
    ├── home.rs             # Hero + about + project cards grid
    ├── project_detail.rs   # Dynamic :slug lookup, full detail page
    ├── side_quests.rs      # Index page with cards linking to detail pages
    └── side_quest_detail.rs # Dynamic :slug lookup for side quests
```

## Design Direction

- Terminal aesthetic with Iosevka Nerd Font, monospace everywhere
- 8 color themes: Rustbox (default), Gruvbox, Dracula, Everforest, Catppuccin, Tokyo Night, Nord, Vantablack
- Each theme has dark/light variants (Vantablack dark-only)
- Theme switcher in navbar
- Each project (featured and side quest) gets its own page with objective, approach, implementation, obstacles, progress
- Syntax highlighting via highlight.js CDN with CSS overrides mapping tokens to theme variables
- Mobile responsive

## Hosting

- GitHub Pages with GitHub Actions CI/CD
- Custom domain: scottyfermo.com (Namecheap)
- DNS: 4 A records to GitHub Pages IPs + CNAME www -> scadoshi.github.io
- Free SSL via GitHub Pages (Let's Encrypt)
- SPA routing handled via 404.html copy of index.html

## Current State

Live at https://scottyfermo.com. Phase 1 complete. All pages, routing, 8 themes, syntax highlighting, and content evaluation done. GitHub Actions deploys automatically on push to master.
