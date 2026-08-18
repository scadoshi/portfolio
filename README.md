# scottyfermo.com

My portfolio site. What it says about the projects lives on the site itself, so
this file stays out of that business (and out of the business of going stale).

## Stack

- [Dioxus](https://dioxuslabs.com/) 0.7.9, Rust compiled to WASM, prerendered to
  static HTML so crawlers and link unfurlers get real markup
- Shared UI (panels, nav, themes, banners) from
  [zwipe-components](https://github.com/scadoshi/zwipe), a git dependency pinned
  in `Cargo.lock`
- GitHub Pages on a custom domain, deployed by GitHub Actions on push to
  `master`

## Build

```
cargo install dioxus-cli --locked
dx serve                                       # dev server
dx build --release --ssg --force-sequential    # what CI runs
```

`--force-sequential` is required: without it the parallel client build finishes
last and overwrites the prerendered `index.html` with a bare shell.

## Layout

- `src/data.rs` is the content; every project and side quest is a const in there
- `public/` is copied verbatim to the site root, so the OG card, `sitemap.xml`,
  and `robots.txt` keep un-hashed URLs
- `index.html` is a custom dx shell, kept only to set `lang="en"`
- `context/rules/` holds the commit and CI conventions
