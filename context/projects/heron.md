# Heron

## Headline

My personal server. It serves the commit counts on this site, cached in steller.

## Category

Production Service

## What It Is

An Axum server that reads GitHub's API for an allowlist of repositories and serves commit counts, stars, languages and last-push dates as JSON. Live at `https://api.scadoshi.dev` since 2026-09-29, on a Hetzner box of its own behind a Cloudflare Tunnel, with steller beside it as the cache.

This site reads `GET /stats` when it builds. See "GitHub numbers" in `../overview.md`.

Named for the great blue heron. It was scotland-server for its first day.

## Where the detail is

heron keeps its own notes, and they are the source of truth: `~/Developer/heron/context/`, starting at `README.md`. The decisions, the steller bug and how it was found, and the deploy steps are all there. Nothing here repeats them.

## What the entry on the site leaves out, on purpose

The entry is the shortest on the site. It has no test count and no line count, since both drift and the repo has them. It has no demo, since the API is live and linked.

## Status

Live. Open work is in `~/Developer/heron/context/progress/todo.md`.

## Repo

~/Developer/heron, github.com/scadoshi/heron
