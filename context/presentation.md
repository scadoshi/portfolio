# Presentation Strategy

## Project Ordering

Lead with strongest signal, end with breadth.

### Tier 1 — "I build real things"
1. **Zwipe** — largest project, shows full-stack architecture at scale
2. **Halo Action Importer** — production tool, shows resilience engineering
3. **Halo Custom Field Builder** — shipped tool, shows end-to-end delivery

### Tier 2 — "I understand the ecosystem"
4. **Marvin** — AI agent tooling in Rust (tool use, streaming, context management)

### Tier 3 — "I go deep on things I don't know"
5. **Nighthawk** — database internals from scratch
6. **Upsee** — ML inference pipeline on-device
7. **Capture** — cross-platform systems programming

## Per-Project Page Structure

Each project page should have:

1. **Hero section** — one-sentence description + category tag + tech icons
2. **What it does** — 2-3 sentences max, no jargon
3. **Architecture diagram** — visual overview of how it's wired (ASCII or simple SVG)
4. **Key code** — 1-2 curated code snippets showing the most interesting technical decisions (not just "hello world" code — show the retry logic, the corruption recovery, the hysteresis state machine)
5. **What I learned** — honest, specific takeaways (not "I learned Rust" — more like "I learned why binary search was the wrong retry strategy for batch failures")
6. **Link to repo**

## Home Page

- Brief intro: "Rust developer. I build things to learn things."
- Project grid with cards — image/icon, title, one-liner, category tag
- No "about me" essay — let the projects speak

## Tone

- Direct. No corporate speak.
- Honest about what's a learning project vs what's production code
- Technical but accessible — a hiring manager should understand the one-liners, an engineer should find depth on the detail pages
- Show the progression: learning projects → production tools → full-stack application

## Interactive Elements (Phase 2+)

- **Nighthawk REPL** — best candidate for WASM terminal. The entire storage engine is pure Rust with file I/O that could be stubbed for in-memory. Let visitors run set/get/delete commands.
- **Code walkthroughs** — expandable annotations on key code snippets. Click a line, see why it's interesting.
- **Architecture diagrams** — hover/click for details on each layer

## What NOT to Include

- No "skills" section with progress bars
- No star ratings on technologies
- No timeline/resume format — this is a project showcase, not a CV
- No blog posts unless they add real value (a Nighthawk walkthrough could work)
