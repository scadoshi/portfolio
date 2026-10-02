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
  stats.rs           every number from heron: baked stats.json, live fetch after load
  stats.json         heron's answer, replaced before each build
  theme_store.rs     localStorage theme persistence, no-ops on the server build
  components/        navbar, footer, project_card, gallery, code_block,
                     linked_text, page_meta
  pages/             home, detail (shared by projects and side quests),
                     side_quests, contribute, not_found
assets/              CSS, fonts, per-project media, the two ascii logos
public/              copied to the site root verbatim: sitemap, robots, og image
context/             this, the commit rules, per-project notes, marketing
```

The shared UI (themes, `Panel`, `Banner`, `NavBar`, `ThemePicker`, `PageMeta`) comes
from `zwipe-components`, a git dependency on the zwipe repo. `Cargo.lock` pins the
exact commit, so pulling changes is a deliberate `cargo update -p zwipe-components`.
Its CSS is inlined as a string constant because a git dep cannot be reached by an
asset pipeline. That also means a fix to shared CSS has to land in zwipe first, and
pushing zwipe's `main` deploys zwipe's production.

Iterating on shared CSS through that loop costs a zwipe deploy and a pin commit per attempt; one afternoon produced five pins for one radius. The cheaper loop is a local override while iterating: put `[patch."https://github.com/scadoshi/zwipe"] zwipe-components = { path = "../zwipe/zwipe-components" }` in `.cargo/config.toml` (not `Cargo.toml`, so it never gets committed), build and screenshot until it looks right, delete the file, restore `Cargo.lock`, push zwipe once, and pin once.

`projects/` holds per-project background notes, one per entry in `data.rs`. They go
deeper than the site does and are where the numbers came from. `data.rs` is still the
source of truth for what the site shows; these are the working notes behind it.

## Moving a page

GitHub Pages cannot answer with a redirect. When a project is renamed or changes lists, add its old address to `MOVED` in `src/data.rs`. SSG then prerenders the old address as a page carrying a refresh and a canonical to the new one. Tests check that every entry leads to a live page and that none shadows one.

## GitHub numbers

The commit counts and last-push dates on the cards come from heron (`~/Developer/heron`, live at `https://api.scadoshi.dev`), not from `data.rs`. `src/stats.json` is the body of heron's `GET /stats` and `src/stats.rs` reads it.

The numbers are baked in when the site builds. The deploy workflow asks heron for a new answer before each build and keeps the committed `src/stats.json` when heron is down or could not resolve every repository, so a build never waits on heron. The workflow also runs every morning.

After a page loads, the browser asks heron once more (`stats::fetch_live`, wasm only) and swaps the live answer in. The hero's source line says ", live" when that worked and ", as of <day>" when it did not, where the day is the baked snapshot's `generated_at`. The prerendered HTML always carries the baked numbers, so hydration matches. Cloudflare caches `GET /stats` for five minutes, so a traffic spike on the site does not reach heron.

A project added to `data.rs` has to be added to `GITHUB_REPOS` on heron's box as well. Until it is, `every_project_on_the_site_is_in_the_snapshot` fails and names it.

## Charts

Every chart is an inline SVG the site draws itself from the `GET /stats` answer, so it takes the theme's colors and needs no chart library. The home page has the year of contributions (heron's `calendar`, from GitHub's GraphQL) as a heatmap with a contributions-by-month line drawn on the same columns; `heatmap.rs` lays both out, and the year's outlier days get a lit edge (`PEAK_RATIO` times the median busy day, `MAX_PEAKS` at most). The heron page has the flow diagram (shared `Diagram*` pieces from zwipe-components), the fleet bar chart of lines, tests and lints, and commits by month summed from `weekly_commits`, which only covers the repositories heron measures and says so in its caption. The steller page has the benchmark against Redis, the one hand-typed number set, with its link pinned to the commit of `BENCHMARKS.md` the rows came from. Hover chips are shared in `chart.rs`; the monotone curve in `curve.rs`. On a phone a chart keeps a readable width inside a `.chart-scroll` box and the time charts open on the newest months (`scroll.rs`).

## The entrance

The hero animates in: the S resolves from noise, the numbers count up, the line draws, the grid sweeps, all left to right. `CountUp`, `Decode` and the `Replay` counter come from zwipe-components (`entrance.rs` there); the chart's part is CSS keyframes with per-cell delays. Clicking the nav S bumps `Replay`, which remounts the keyed pieces and runs everything again.

The prerendered page carries the settled values, so without script it reads right. With script, the end-of-body block in `index.html` (this repo's dx shell) runs before first paint: it puts the wrapper on the theme stored in localStorage and marks the document `js`, and the stylesheet then hides the logo and numbers and pauses the chart until the app marks `hydrated` on mount. That is what keeps the page from showing the default theme and the final numbers for the few hundred milliseconds the wasm takes to load.

## Source counts

Lines, test functions and configured clippy lints per repository come in heron's `counts` field in the same `GET /stats` answer as the commits, measured by heron from a tarball of each repository's default branch after every push, and rendered under the commit line on every card. The same two sources apply as for commits: the live answer after the page loads, else the `stats.json` baked at build, which the deploy workflow refreshes from heron and only replaces when every repository in the answer is measured. For rustmas and sharpmas that is `main`, the template branch, not the solution branches.

Because the numbers come from heron, the copy in `data.rs` never types one. `no_summary_field_types_a_count_by_hand` fails the build if a headline, summary, metric, progress or card bullet says "N tests" or "N lines". The narrative fields are free to.

## Build and deploy

```bash
dx build --release --ssg --force-sequential
```

`--force-sequential` is what makes `index.html` a real prerendered page rather than a
bare shell. After a build, check `target/dx/portfolio/release/web/public/index.html`
is tens of kilobytes: a stale server holding port 9999, or macOS killing the fresh
server binary on a cached signature, both leave a shell with a green log. Deleting
`target/dx/portfolio/release/web/server` before the build clears the second. Pushing to `main` runs the same build in Actions and publishes it. Tests
and clippy gate that deploy: red means the site does not update. See
`rules/commit_guidelines.md` for the exact commands, which are worth running before
you push rather than after.

Two things the build does that are easy to miss. `/404` is prerendered through the
catch-all route so the workflow can ship it as `404.html`, which is why unknown URLs
get a real title and a noindex instead of the home page's. And `asset!()`
content-hashes filenames, so anything referenced by a literal path has to live in
`public/`, not `assets/`.

## Writing content

The bar is in `rules/commit_guidelines.md`, `rules/comment_guidelines.md` and the humanizer skill: short enough
that a person reads it, specific enough to be checkable, and the code is the real
evidence. Long comprehensive prose reads as machine-written and gets skipped, which
is worse than saying less.

## History

`history/` holds the planning and progress docs from the original build. They record
what was intended in 2026, not what is true now. Read them for background, never as a
description of the current tree.
