# Portfolio Site Plan

## Stack
- Dioxus 0.7.1 + dioxus-router targeting WASM
- Static deploy (GitHub Pages / Cloudflare Pages)
- 6 color themes with light/dark variants
- Iosevka Nerd Font (CDN via fontsource)

## Themes

| Theme | Dark | Light |
|-------|------|-------|
| Gruvbox (default) | Yes | Yes |
| Everforest | Yes | Yes |
| Catppuccin | Yes (Mocha) | Yes (Latte) |
| Tokyo Night | Yes | Yes |
| Nord | Yes | Yes |
| Vantablack | Yes | No |

## Pages

### 1. Home (`/`)

**Hero**
- Name: Scotty Ray Fermo
- Title: Software Engineer | Full-Stack | Rust
- Mission: "I build production Rust systems that solve hard problems."
- Contact links: GitHub, LinkedIn, Email

**About** (2-3 sentences)
- Builder mentality. Nearly 4 years building production systems. Over a year of intensive Rust development. When I see inefficiency, I build tools to eliminate it.

**Big 3 Project Cards**
Each card: project name, category tag, summary, impact metric with hover tooltip, links to detail page and GitHub repo.

1. **Zwipe** - Full-stack MTG deck builder. 24,500 lines of production Rust.
2. **Halo Action Importer** - Production bulk import. Weeks of manual work, automated.
3. **Halo Custom Field Builder** - Shipped CLI tool. Manual UI clicks to one CSV import.

### 2. Zwipe (`/projects/zwipe`)
Full detail: objective, approach (hexagonal architecture, newtypes, JWT auth), implementation (3 code snippets), obstacles, progress and impact.

### 3. Halo Action Importer (`/projects/halo-action-importer`)
Full detail: objective, approach (resilience-first), implementation (resilience pattern, retry evolution), obstacles, progress and impact.

### 4. Halo Custom Field Builder (`/projects/halo-custom-field-builder`)
Full detail: objective, approach (domain modeling, two-layer serialization), implementation (field type system, serialization), obstacles, progress and impact.

### 5. Side Quests (`/side-quests`)
One page, four sections. Each: name, category, description, highlights, one code snippet, repo link.
- **Marvin** - AI Tooling
- **Nighthawk** - Database Internals
- **Upsee** - ML Inference
- **Capture** - Systems Programming

## Navbar

```
[SRF]  Home    Projects ▾    Side Quests    [Gruvbox ▾] [light]
                 Zwipe
                 Halo Action Importer
                 Halo Custom Field Builder
```

## GitHub Repos

- https://github.com/scadoshi/zwipe
- https://github.com/scadoshi/halo_action_importer
- https://github.com/scadoshi/halo_custom_field_builder
- https://github.com/scadoshi/marvin
- https://github.com/scadoshi/nighthawk
- https://github.com/scadoshi/upsee
- https://github.com/scadoshi/capture

## File Structure

```
src/
├── main.rs              # Entry, Router, App, NavbarLayout
├── data.rs              # All project content as static data
├── theme.rs             # ThemeConfig, THEMES list
├── components/
│   ├── navbar.rs        # Nav with Projects dropdown
│   ├── project_card.rs  # Card with impact tooltip
│   ├── code_block.rs    # Terminal-styled code display
│   ├── theme_switcher.rs # Theme picker + light/dark toggle
│   └── footer.rs        # Contact links
└── pages/
    ├── home.rs          # Hero + about + project grid
    ├── project_detail.rs # Dynamic :slug detail page
    └── side_quests.rs   # Grouped learning projects
```

## Remaining Phase 1 Work

- GIF/screenshot assets for project cards
- Deploy to GitHub Pages or Cloudflare Pages

## Phase 2 (Future)

- Syntax-highlighted code blocks
- Architecture diagrams (SVG)
- Nighthawk WASM REPL
- Code walkthroughs with expandable annotations

## Phase 3 (Future)

- Transitions and scroll animations
- SEO metadata
- Performance optimization
- Custom domain
