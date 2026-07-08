# Request to zwipe-components: adopt the zite/portfolio shared UI

**From:** the portfolio site (scottyfermo.com, this repo)
**To:** the `zwipe-components` maintainers (Scotty + AI, other hat)
**Date:** 2026-07-08

Zwipe's roadmap (`context/plans/zwipe_components.md`) says a component earns a
move into the crate when a real second consumer appears, and it names a future
"portfolio" pull as the end goal. This is that consumer showing up. The
portfolio and zite already render several elements that look exactly the same
because they *are* the same code, copied by hand, and the copies have started
to drift. We would like the crate to become the single home for them.

## What is duplicated today (evidence)

1. **Theme palettes.** Portfolio `assets/main.css` embeds the same 14 themes
   (rustbox, gruvbox, dracula, everforest, catppuccin, tokyo-night, nord,
   rose-pine, monokai, one-dark, solarized, plus the three color-blind themes)
   as zwipe's `shared/themes.css`, dark and light variants, byte-for-byte
   (spot-checked `.theme-gruvbox-dark`: identical). Roughly 650 lines of
   variable blocks that silently strand the portfolio the day zwipe tweaks a
   palette.

2. **Theme picker.** zite's `ThemePicker` (`zite/src/main.rs`) and the
   portfolio's `ThemeSwitcher` (`src/components/theme_switcher.rs`) render
   identical DOM with identical class names: `theme-switcher`, `theme-select`,
   `theme-select-trigger`, `theme-select-content`, `theme-option`,
   `theme-select-label`, `theme-backdrop`, `mode-toggle`. Same color-blind
   bottom section, same `▾` trigger, same backdrop-to-close. Drift has already
   begun: the portfolio grew a `has_light_mode` guard and a vantablack special
   case zite lacks; zite title-cases slugs at runtime while the portfolio
   carries a static label table.

3. **`ThemeConfig`.** Portfolio `src/theme.rs` is a hand-copy of
   `zwipe_core::domain::user::models::theme::ThemeConfig` (name, `is_dark`,
   `css_class()`), diverged in how the class is applied: zite sets
   `body.className`, the portfolio wraps the app in a `theme-wrapper` div.

4. **Navbar hamburger.** Same structure on both sides: `nav-brand`,
   `nav-toggle` with three `nav-toggle-bar` spans, `nav-panel`,
   `nav-panel-inner`, `nav-links`, theme picker as the panel's trailing item,
   shared 60rem breakpoint, and the scroll-to-top + logo animation-reset
   `eval` script on the brand link copied verbatim. The link *contents* differ
   (portfolio has a Projects dropdown; zite has store links), so this is a
   shell + CSS share, not a whole-component share.

5. **`PageMeta`.** Structurally identical head-meta component in both repos;
   the diff is only constants: base URL, `og:site_name`, title-suffix rule,
   og image presence, twitter card type.

6. **Footer.** Same visual shape (centered muted fine print over an accent
   top border) but different markup and classes. Weakest candidate; listed for
   completeness, fine to skip.

## What we are asking for

1. **Make the crate consumable outside the workspace.** The portfolio is a
   separate repo, so it needs `zwipe-components` as a git dependency at
   minimum (public repo), or crates.io if you want to go all the way. Note the
   path dep on `zwipe-core` blocks a crates.io publish unless core publishes
   too; a public git dep sidesteps that and is enough for v1 of this.

   **Amended 2026-07-08 (owner ruling): crates.io, not a git dep.** The
   portfolio must stay maintainable without the zwipe repo present on the
   machine, and the owner wants the real registry experience. The portfolio
   holds its migration until `cargo add zwipe-components` works.

   **Publish checklist (verified against the registry and the manifests as
   of `8473761a`, 2026-07-08):**
   - Neither `zwipe-core` nor `zwipe-components` exists on crates.io yet;
     both names appear free.
   - `zwipe-core` publishes first — crates.io rejects path-only deps, so
     `zwipe-components` must depend on it as
     `zwipe-core = { version = "..." }` (a `path` can stay alongside for
     workspace dev, cargo strips it on publish).
   - Both crates need their own semver (suggest `0.1.0`), decoupled from
     `version.workspace = true` (currently 1.4.0, the app release number —
     published versions are permanent and shouldn't bump with app ships).
   - Both `[package]` sections need `description` and `license` (PolyForm
     Noncommercial's SPDX id is accepted); `repository` recommended.
   - `include`/`exclude` check: `zwipe-components` must ship its `assets/`
     (components.css, themes.css) in the package or the `include_str!`
     consts fail to build from the registry tarball.

2. **Keep the zwipe-core surface optional.** Only `KeywordChips` touches
   `zwipe_core` today (`keyword_reminder`). A `domain` feature (default-on so
   zwiper/zite notice nothing) would let the portfolio pull Button, Chip,
   ActionBar, and the components below without MTG domain code riding along.

3. **Lift the theme system into the crate.**
   - Ship `themes.css` as a crate asset alongside `components.css`, so
     consumers copy both at build time and the portfolio deletes its fork.
   - Move `ThemeConfig` + the theme list out of zwipe-core's *user domain*
     into `zwipe-components` (a theme is a UI concern; zwipe-core can
     re-export for compatibility). Without this, `ThemePicker` can't move.
   - Promote `ThemePicker` as authored in zite. Portfolio-side asks: keep the
     color-blind grouping, and either adopt or drop the light-mode guard, we
     don't mind which, we just want one copy.

4. **Promote `PageMeta`**, parameterized by a small site config
   (`base_url`, `site_name`, title-suffix rule, optional og image, twitter
   card type) instead of the current per-repo constants.

5. **Promote a nav shell** (`NavBar` / hamburger toggle / panel) with slots:
   brand child, links children, trailing child (the theme picker). The
   hamburger + panel + 60rem breakpoint CSS moves to `components.css`; each
   site keeps its own link list. The brand link's scroll/logo-reset script can
   ride along as the default brand behavior.

## Suggested order (each its own small pass, per your roadmap)

1. Public visibility + `domain` feature gate. Unblocks everything else.
2. `themes.css` as crate asset + `ThemeConfig`/theme-list move. Portfolio
   deletes `src/theme.rs` and ~650 lines of `main.css`.
3. `ThemePicker`. Portfolio deletes `src/components/theme_switcher.rs`.
4. `PageMeta`.
5. Nav shell (most API design, do it last).

## Round 2 (2026-07-08, post-migration): nav shell selector fixes

**Status: GRANTED IN FULL, shipped upstream as `a76ccc62` the same day.**
All three fixes landed as asked (2.2 used `> li > *` instead of a new
`.nav-item` class — better call, no new API). The portfolio updated its pin
to `04dd9276`, deleted all four counter-rule blocks (the three below plus
the `a:visited` restatement, which the child combinator's higher
specificity made redundant), and re-verified as second consumer. End state
achieved: the portfolio's nav footprint is `--nav-max-width: 760px` plus
brand/logo rules.

**Consumption mechanism, final ruling (owner, 2026-07-08):** the crates.io
amendment above is REVERSED — the git dependency is the settled mechanism.
Registry publication is off the table until a consumer outside the owner's
repos appears; the publish checklist above is archived for that day. The
portfolio keeps `git = "https://github.com/scadoshi/zwipe"` with
Cargo.lock pinning, and picks up releases deliberately via
`cargo update -p zwipe-components`.

The original asks, kept for the record:

Status update first: the portfolio has fully migrated (git dep pinned at
`7848dbd7`). Themes, ThemePicker, PageMeta, NavBar, and NavDropdown are all
consumed from the crate; ~760 lines of duplication deleted. Round 1 is
closed. These three asks are what migration surfaced — each one exists
because the shell's selectors assume zite's flat markup, and each currently
costs the portfolio a counter-rule block in `assets/main.css` that must be
deleted when the fix lands.

### 2.1 Scope nav pill styling to direct link items

- **Today:** `.nav-links a, .nav-links button.nav-link` (base, hover,
  active) and the in-panel `.nav-panel .nav-links a, .nav-panel .nav-links
  button.nav-link` sizing match *all descendants* of the links slot. Anchors
  nested inside a composed widget inherit pill borders, padding, background,
  and shadow — the portfolio's `NavDropdown` menu items rendered as a stack
  of pills inside the menu box.
- **Ask:** child combinators — `.nav-links > li > a`, `.nav-links > li >
  button.nav-link`, and the same for the in-panel variants.
- **zite impact:** none; its links are all direct `li > a`.
- **Portfolio deletes:** the `.nav-links .nav-dropdown-content a` reset
  block (~25 lines) plus its hover rule.

### 2.2 Stretch panel items generically, not by tag enumeration

- **Today:** the in-panel full-width/centering rules enumerate `a` /
  `button.nav-link`. A `NavDropdown` placed in the links slot (a wrapper
  `div` + `button.nav-dropdown-trigger`) falls through both selectors and
  renders content-width — the crate's own component doesn't fit the crate's
  own shell. The portfolio's first patch attempt (`.nav-panel .nav-dropdown
  { width: 100% }`) then leaked into the theme picker's dropdown in the
  trailing slot and broke *that* row, which shows how fragile the
  enumeration is for consumers.
- **Ask:** stretch every direct panel link item — `.nav-panel .nav-links >
  li > * { width: 100% }` (or introduce a documented `.nav-item` class the
  shell owns) — and move `.nav-panel .nav-links .nav-dropdown-trigger {
  width: 100%; justify-content: center; }` into the crate next to its
  existing in-panel trigger font/padding rules. Keeping it scoped under
  `.nav-links` preserves the trailing-slot picker's compact centered row.
- **zite impact:** none today; free if it ever puts a dropdown in its nav.
- **Portfolio deletes:** both `.nav-panel .nav-links .nav-dropdown*` width
  rules.

### 2.3 First-class anchor items in NavDropdown

- **Today:** `.nav-dropdown-item` styling assumes a `<button>`: it sets no
  `display`, no `text-decoration`, and has no `:visited` handling. Menu
  items that are real links (router `Link`s — needed for hrefs, SEO, and
  middle-click) render inline, ignore the `width: 100%`, keep underlines,
  and take the host's global `a:visited` color (which outranks the bare
  class selector).
- **Ask:** add an anchor variant to the existing rules:
  `a.nav-dropdown-item { display: block; text-decoration: none; }` and
  include `a.nav-dropdown-item:visited` in the color declaration. The `a.`
  prefix wins the specificity fight against any host's `a:visited` global,
  so consumers need zero overrides.
- **zite impact:** none (picker items are buttons); benefits any future
  link-based menu.
- **Portfolio deletes:** the rest of the anchor reset, including the
  in-panel item compaction rule.

**End state when all three land:** the portfolio's entire nav footprint in
`main.css` is `--nav-max-width: 760px` plus its brand/logo rules. The
portfolio re-verifies each fix as the second consumer, same as Round 1.

## What the portfolio commits to

- Track the dioxus `0.7.9` pin (already matches).
- Add the `build.rs` copy pipeline for `components.css` + `themes.css`,
  mirroring zite.
- Migrate to each component as it lands and report any render regressions,
  so every move gets its second-consumer verification immediately.
