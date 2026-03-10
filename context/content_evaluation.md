# Content Evaluation Initiative

## Background

The portfolio site structure is built and compiling. Routing, themes, navbar, all pages, and CSS are functional. But the content on the site was written during the initial build pass without cross-referencing the actual project source code and context files. This initiative is about going back through every project and making sure the descriptions do justice to what was actually built, learned, and solved.

## Goal

Each project description should clearly show:
1. **Problem** - What issue or gap existed
2. **Objective** - What I set out to build and why
3. **Approach** - Architecture and design decisions
4. **Implementation** - Key technical details and interesting code
5. **Obstacles/Solutions** - Real roadblocks and how they were resolved (shows critical thinking)
6. **Progress** - Current state of the project
7. **Impact** - What it demonstrates about my capabilities

The descriptions should show that I notice problems, think critically about solutions, and follow through on execution. Not feature lists. Stories about problem-solving.

## Structural Change: Side Quests Get Their Own Pages

During evaluation of Marvin, we realized the side quest card format (one shared page with bullet lists) is too limiting. Each side quest now has its own route (`/side-quests/:slug`) with the full project page structure. The `/side-quests` page is an index with cards linking to detail pages. Side quests use the same `Project` struct with a `ProjectType::SideQuest` discriminator.

**Status: COMPLETE**

## Evaluation Progress

### All Projects Evaluated

- **Marvin** - COMPLETE. Added open source contribution story (Rig issue #1370, PR across 17 files), architecture evolution (220-line monolith to command pattern), tool system detail (Arc<TavilyClient>, schemars JSON Schema generation), LOC count (~1,750).

- **Nighthawk** - COMPLETE. Added Bitcask paper link, corrected technical details about append-only log compaction, added LOC count, linked to Bitcask paper PDF.

- **Upsee** - COMPLETE. Added MoveNet model details, Hugging Face model source link, fall detection algorithm specifics (acceleration magnitude thresholds), LOC count.

- **Capture** - COMPLETE. Added technical details about raw pointer FFI patterns, V4L2 ioctl wrappers, memory-mapped buffer management, the journey from v4l crate to raw syscalls.

- **Halo Custom Field Builder** - COMPLETE. Corrected LOC from ~727 to ~1,370. Added layered architecture detail, bin/lib crate split, type-safe domain modeling (Name/Label/FieldType newtypes), OAuth 2.0 token caching pattern, CSV header-position detection, interactive debug TUI, rate limiting, GitHub Actions CI/CD matrix build.

- **Halo Action Importer** - COMPLETE. Added LOC (~3,230 across 20 files), layered architecture mention. Expanded approach from 5 to 8 bullets: data normalization layer (11+ serde aliases, 6+ datetime formats, Excel serial-to-datetime, Arizona timezone), two-tier caching architecture, report-based deduplication, real-time progress with ETA. New "Data Normalization" code snippet. Added data format chaos obstacle.

- **Zwipe** - COMPLETE. Fixed "35k+" to "100k+" cards, "26 lints" to "33 lints", "two crates" to "three binaries". Added structured error chain (SQLx → constraint detection → domain error → HTTP status), production-strict linting (unwrap/expect/panic/todo/dbg/print denied), expanded password validation policy, shared domain types between frontend and backend.

## Additional Enhancements Made During Evaluation

- **Rustbox theme** (default) and **Dracula theme** added
- **Syntax highlighting** via highlight.js CDN with CSS overrides mapping hljs tokens to theme variables
- **LinkedText component** for auto-detecting URLs in text and rendering as clickable links
- **8 themes total**: Rustbox, Gruvbox, Dracula, Everforest, Catppuccin, Tokyo Night, Nord, Vantablack

## Source of Truth

Each project has its own context directory at `~/Work/<project>/context/` with purpose files, progress logs, and technical notes. These are the source of truth for what was actually built and learned. The portfolio descriptions in `src/data.rs` should accurately reflect those files.

## Tone Reminders (from presentation.md)

- No em dashes
- No mention of employer/role
- Refer to "Halo Software" or "Halo Software products" (not just ITSM, no parenthetical product lists)
- Direct, honest, technical but accessible
- Honest about what's a learning project vs production code
