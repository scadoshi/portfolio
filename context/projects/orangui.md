# orangui

## Headline

The Dioxus UI kit under this site, zwipe and cairn. 31 themes, one panel shape, and a stylesheet held to two rules by its tests.

## Category

UI Kit

## What It Is

The generic part of zwipe's `zwipe-components`, lifted into its own crate on 2026-10-12 at `~/Developer/orangui`, github.com/scadoshi/orangui, MIT. One flat crate: 27 modules under `src/`, each one component or one hook family, with `lib.rs` as the whole public surface. The stylesheets ship as string constants (`THEMES_CSS`, `COMPONENTS_CSS`, `APP_CSS`, `SITE_CSS`, `TOAST_CSS`) and the three scripts the same way, because a git dependency cannot be reached by an asset pipeline.

Three kits sit on a shared base. For every host: `ActionBar`, `Banner`, `Button`, `Chip`, `NavBar`, `NavDropdown`, `PageMeta`, `Panel`, `PasswordInput`, `ThemePicker`, the diagram pieces, the entrance figures (`CountUp`, `Decode`, `Figure`, `Replay`), `HeatGrid` and the chart helpers. The app kit: `BottomSheet`, `PageHeader`, `ThemeSheet`, the hints, the OS back stack and, behind the `toast` feature, `ToastStack`. The site kit: `use_persisted_theme`, `StatsStrip`, `GalleryFrame`, `GalleryFooter`, the theme wipe and the reveal, nav glide and scroll fade scripts.

The theme model (`ThemeConfig`, `ALLOWED_THEMES`, `COLORBLIND_THEMES`, `display_theme_name`) moved in from `zwipe-core` so nothing in the crate depends on zwipe. 31 themes in `assets/themes.css`, a dark and a light block each, 62 blocks; gruvbox dark repeated in `:root` as the default.

The name is orangutan, a call sign of mine, plus GUI. The old orangui repository, the Tampermonkey scripts that give Teams and Outlook zwipe's look, is `tamper-orangui` now.

This site is on it: see "Where things live" in `../overview.md` for how the CSS is inlined and why a shared fix lands in orangui first and gets pinned here second.

## What It Proves

- One crate under three hosts that knows none of them. No product name, no domain type, no route. A Magic card row or a deck chart lives in the host, built from these pieces
- A color contract with a test behind each half. Every theme block carries six palette slots and `--color-money`, each at 3:1 or better on the block's background (`tests/themes_palette.rs`). Status colors appear only under selectors that mark an outcome (`tests/status_colors.rs`), and `status_uses` is public so a host holds its own sheet to the same rule
- The theme list and the stylesheet cannot drift: a test reads `themes.css` at compile time and fails when a listed theme lacks a block, and its twin fails when the stylesheet has a theme the list does not
- The migration cost a host a dependency line and an import path. Class names, component names and the storage key are unchanged, and the before and after screenshots of this site were pixel-identical bar a playing video

## Key Technical Highlights

The theme wipe. A pick writes a pending theme rather than the real one. A view transition snapshots the page, the theme class is swapped on the themed element inside it, and the real theme is committed only once the swap is in, so the re-render lands on a page that already shows it. Light to dark sweeps right to left; everything else left to right. Browsers without view transitions, and readers who ask for reduced motion, get the instant swap. Two hooks drive it: `use_theme_wipe` for a site's picker, which also replays the entrance, and `use_theme_follow` for a host whose theme is set from many places, as the app's is.

Slot order is fixed across themes: 1 is the theme's blue, 2 its orange, 3 green, 4 violet, 5 yellow, 6 cyan. A row of chips alternates warm and cool in every theme, and the same element has the same temperature everywhere. Values come from published palettes with the source above each block; the four color-vision themes are built from the deficiency outward, and their six slots must stay 8 apart in OKLab under the deficiency they are built for.

The cascade is the contract. A host inlines `THEMES_CSS`, then `COMPONENTS_CSS`, then `APP_CSS` or `SITE_CSS`, then its own sheet. A component rule names a theme variable and never a color literal, so a host overrides at equal specificity. Class names are as public as the Rust names: a host styles against them, so renaming one is a breaking change.

## What I Learned

The theme model had to be in two places. zwipe's server validates a saved theme against the list, and its core crate cannot depend on a UI crate, so orangui carries its own `ThemeConfig` and zwipe-components converts between the two, with a test on the zwipe side pinning the lists equal.

crates.io is off the table for now, decided 2026-10-12. The toast stack pins `dioxus-primitives` to a git commit, crates.io has only its 0.0.0, and a git dependency is refused even behind a feature. The name was free that day. If it changes, the shape is a workspace with `orangui` published and `orangui-toast` git only.

## Roadmap

The production switch, one host at a time, at a device: merge the branch, deploy, check live. zite and this site are public pages; the zwipe app is a store cut; cairn needs its simulator build first. Then the re-pin scripts move from `zwipe-components` to `orangui`. Open: whether the theme-sync test ships as a helper a host calls with its own list.

## Status

Doing. The crate exists with CI green; zwipe, this site and cairn each have a branch on it, built and screenshotted; no host is on it in production yet. Measured 2026-10-12: 3,162 lines of Rust in `src/`, 4,178 of CSS and script in `assets/`, 25 tests (23 unit, the two stylesheet gates), 3 commits.

## Repo

~/Developer/orangui, github.com/scadoshi/orangui
