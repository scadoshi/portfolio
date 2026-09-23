# Portfolio

scottyfermo.com. A Rust developer's portfolio written in Rust: Dioxus, prerendered
to static HTML, served by GitHub Pages.

## Where things live

Content is data, not markup. Every project is a `Project` const in `src/data.rs`, and
`featured_projects()` / `side_quests()` are the only two lists that decide what the
site shows. Adding a project means adding a const, putting it in one of those lists,
and adding its `<loc>` to `public/sitemap.xml`. The navbar dropdown and the cards
build themselves from the same data.

```
src/
  main.rs            Route enum, App, the head tags, static_routes() for SSG
  data.rs            every Project and Snippet; the two ordering functions
  theme_store.rs     localStorage theme persistence, no-ops on the server build
  components/        navbar, footer, project_card, gallery, code_block,
                     linked_text, page_meta
  pages/             home, detail (shared by projects and side quests),
                     side_quests, contribute, not_found
assets/              CSS, fonts, per-project media, the two ascii logos
public/              copied to the site root verbatim: sitemap, robots, og image,
                     redirect stubs for renamed slugs
context/             this, the commit rules, per-project notes, marketing
```

The shared UI (themes, `Panel`, `Banner`, `NavBar`, `ThemePicker`, `PageMeta`) comes
from `zwipe-components`, a git dependency on the zwipe repo. `Cargo.lock` pins the
exact commit, so pulling changes is a deliberate `cargo update -p zwipe-components`.
Its CSS is inlined as a string constant because a git dep cannot be reached by an
asset pipeline. That also means a fix to shared CSS has to land in zwipe first, and
pushing zwipe's `main` deploys zwipe's production.

`projects/` holds per-project background notes, one per entry in `data.rs`. They go
deeper than the site does and are where the numbers came from. `data.rs` is still the
source of truth for what the site shows; these are the working notes behind it.

## Build and deploy

```bash
dx build --release --ssg --force-sequential
```

`--force-sequential` is what makes `index.html` a real prerendered page rather than a
bare shell. Pushing to `main` runs the same build in Actions and publishes it. Tests
and clippy gate that deploy: red means the site does not update. See
`rules/commit_guidelines.md` for the exact commands, which are worth running before
you push rather than after.

Two things the build does that are easy to miss. `/404` is prerendered through the
catch-all route so the workflow can ship it as `404.html`, which is why unknown URLs
get a real title and a noindex instead of the home page's. And `asset!()`
content-hashes filenames, so anything referenced by a literal path has to live in
`public/`, not `assets/`.

## Writing content

The bar is in `rules/commit_guidelines.md` and in the humanizer skill: short enough
that a person reads it, specific enough to be checkable, and the code is the real
evidence. Long comprehensive prose reads as machine-written and gets skipped, which
is worse than saying less.

## History

`history/` holds the planning and progress docs from the original build. They record
what was intended in 2026, not what is true now. Read them for background, never as a
description of the current tree.
