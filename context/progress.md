# Progress

## Phase 0 — Planning and Content (COMPLETE)

- [x] Create project directory and context structure
- [x] Write project descriptions for all 7 repos
- [x] Define presentation strategy and page structure
- [x] Define architecture direction (Dioxus WASM)
- [x] Curate key code snippets per project
- [x] Plan page layouts and component structure

## Phase 1 — Static Site (COMPLETE)

- [x] Initialize Dioxus project (dx 0.7.3, dioxus 0.7.1)
- [x] Dioxus project scaffold with routing (/, /projects/:slug, /side-quests, /side-quests/:slug)
- [x] Home page with hero, about blurb, 3 project cards
- [x] Individual project detail pages (Zwipe, Halo Action Importer, Halo Custom Field Builder)
- [x] Side quest index page with cards linking to individual detail pages
- [x] Individual side quest detail pages (Marvin, Nighthawk, Upsee, Capture)
- [x] Navbar with [SRF] brand, Projects dropdown (full names), Side Quests link
- [x] Projects dropdown hover fix (bridge element for stable hover)
- [x] All GitHub repo links verified and accurate (underscore naming)
- [x] 8 themes: Rustbox (default), Gruvbox, Dracula, Everforest, Catppuccin, Tokyo Night, Nord, Vantablack
- [x] Light/dark toggle per theme (Vantablack dark-only)
- [x] Theme switcher in navbar with dropdown + [light]/[dark] button
- [x] Impact metric tooltips on project cards (hover modal with yellow border)
- [x] Mobile responsive layout
- [x] Footer with GitHub, LinkedIn, Email links
- [x] LinkedIn URL corrected to real profile
- [x] Syntax-highlighted code blocks via highlight.js CDN with theme-aware CSS overrides
- [x] LinkedText component for auto-linking URLs in text
- [x] Component key fix for stale snippet rendering on navigation
- [ ] GIFs/screenshots for project cards (placeholder for now)

## Content Evaluation (COMPLETE)

All 7 project descriptions reviewed against actual source code and updated:

- [x] Marvin — open source contribution, architecture evolution, LOC
- [x] Nighthawk — Bitcask paper link, compaction details, LOC
- [x] Upsee — MoveNet/Hugging Face links, fall detection algorithm
- [x] Capture — raw pointer FFI, V4L2 ioctls, memory-mapped buffers
- [x] Halo Custom Field Builder — corrected LOC, layered architecture, CI/CD
- [x] Halo Action Importer — cache evolution story, data normalization, two-tier caching
- [x] Zwipe — 100k+ cards, 3 binaries, error chain, strict linting, shared types

## Hosting (IN PROGRESS)

- [x] GitHub Actions workflow for build and deploy to GitHub Pages
- [x] Domain purchased: scottyfermo.com (Namecheap)
- [x] DNS A records and CNAME configured pointing to GitHub Pages
- [ ] DNS propagation and GitHub Pages verification
- [ ] Enforce HTTPS
- [ ] Site live at https://scottyfermo.com

## Phase 2 — Interactive Elements (FUTURE)

- [ ] Architecture diagrams (SVG or canvas)
- [ ] Nighthawk WASM REPL (embed terminal, run storage engine in-browser)

## Phase 3 — Polish (FUTURE)

- [ ] GIF/screenshot assets for project cards
- [ ] Transitions and scroll animations
- [ ] SEO metadata
- [ ] Performance optimization (lazy loading, code splitting if applicable)
