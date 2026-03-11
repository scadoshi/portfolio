# Portfolio Site Plan

## Stack
- Dioxus 0.7.1 + dioxus-router targeting WASM
- GitHub Pages via GitHub Actions CI/CD
- Custom domain: scottyfermo.com
- 8 color themes with light/dark variants
- Iosevka Nerd Font (CDN via fontsource)
- highlight.js for syntax highlighting (CDN)

## Themes

| Theme | Dark | Light |
|-------|------|-------|
| Rustbox (default) | Yes | Yes |
| Gruvbox | Yes | Yes |
| Dracula | Yes | Yes |
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

1. **Zwipe** - Full-stack MTG deck builder. ~24,500 lines of production Rust.
2. **Halo Action Importer** - Production bulk import. Weeks of manual work, automated.
3. **Halo Custom Field Builder** - Shipped CLI tool. Manual UI clicks to one CSV import.

### 2-4. Featured Project Detail Pages (`/projects/:slug`)

Full detail: objective, approach, implementation (syntax-highlighted code snippets with component keys for correct rendering on navigation), obstacles, progress and impact.

### 5. Side Quests Index (`/side-quests`)

Card-based index linking to individual detail pages. Subtitle explains these are proofs of concept and learning projects.

### 6. Side Quest Detail Pages (`/side-quests/:slug`)

Same full detail structure as featured projects. Each side quest gets its own page:
- **Marvin** - AI Tooling
- **Nighthawk** - Database Internals
- **Upsee** - ML Inference
- **Capture** - Systems Programming

## Navbar

```
[SRF]  Home    Projects v    Side Quests    [Rustbox v] [light]
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
├── main.rs              # Entry, Router, App, NavbarLayout, highlight.js CDN
├── data.rs              # All project content (Project struct, ProjectType enum)
├── theme.rs             # ThemeConfig, 8 themes
├── components/
│   ├── navbar.rs        # Nav with Projects dropdown
│   ├── project_card.rs  # Card with impact tooltip
│   ├── code_block.rs    # highlight.js integration with unique IDs
│   ├── linked_text.rs   # Auto-linking URLs in text
│   ├── theme_switcher.rs # Theme picker + light/dark toggle
│   └── footer.rs        # Contact links
└── pages/
    ├── home.rs          # Hero + about + project grid
    ├── project_detail.rs # Dynamic :slug detail page (featured)
    ├── side_quests.rs   # Card index linking to detail pages
    └── side_quest_detail.rs # Dynamic :slug detail page (side quests)
```

## Hosting

- GitHub Actions workflow builds on push to master
- Installs Rust + wasm32-unknown-unknown target
- Runs `dx build --release`
- Adds CNAME file for custom domain
- Copies index.html to 404.html for SPA routing
- Deploys via upload-pages-artifact + deploy-pages

## Remaining Work

- GIF/screenshot assets for project cards
- Architecture diagrams (SVG)
- Nighthawk WASM REPL
- Transitions and scroll animations
- SEO metadata
