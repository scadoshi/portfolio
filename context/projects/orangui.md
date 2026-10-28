# Orangui

## Headline

The Rust UI kit under Zwipe, Cairn and this site, with 50 color themes.

## Category

UI Kit

## What It Is

The generic half of Zwipe's `zwipe-components`, split into its own crate on 2026-10-12: themes, panels, nav, chips, the entrance effects, the heat grid and the chart helpers. MIT, at github.com/scadoshi/orangui. Its hosts are Zwipe (the app and zwipe.net), this site and Cairn, each pinning a commit in its lockfile.

Before the split, this site and Cairn took the components from Zwipe's repo, so a shared style fix meant pushing Zwipe, which deploys Zwipe, and then re-pinning. One corner radius took five rounds of that. Orangui has no deploy of its own.

The theme model (`ThemeConfig`, `ALLOWED_THEMES`) moved in with it, so nothing in the crate knows Zwipe. Zwipe's server still validates a saved theme against its own copy of the list, and a test on Zwipe's side keeps the two equal.

The name is orangutan, a call sign of mine, plus GUI. The old orangui repository, the Tampermonkey scripts that give Teams and Outlook Zwipe's look, is `tamper-orangui` now.

How this site inlines the CSS, and why a shared fix lands in Orangui first, is under "Where things live" in `../overview.md`.

## Where the detail is

Orangui keeps its own notes: `~/Developer/orangui/context/`, starting at `README.md`. The color rules are in `development/styling.md`, and the host switch in `plans/host_migration.md`. Nothing here repeats them.

## What the entry on the site leaves out, on purpose

The cascade order, the CSS variable names, OKLab and the color-vision themes, the theme wipe's mechanics, and the `status_uses` scanner. They are real and they are in the repo. The entry keeps one snippet, the theme model with its stylesheet test, since it shows the idea in the fewest lines. The page runs the kit itself (`src/components/orangui_demo.rs`) rather than showing pictures of it, so it takes the visitor's theme.

## Status

Doing. CI green. Split out on 2026-10-12; this site, Zwipe and Cairn all run on it as of 2026-10-13. On 2026-10-12 three pages of this site matched pixel for pixel before and after, apart from a playing video.

## Repo

~/Developer/orangui, github.com/scadoshi/orangui
