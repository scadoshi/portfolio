# Review backlog

What four outside reviews (Claude, Gemini, Grok, ChatGPT, October 2026) said about the site and has not been done yet. The count is how many raised it. The storage overclaims (scope notes, "key-value stores", heron as "Live Service") are done and not listed.

## Home page

- No ask above the fold: target role, years, "open to", resume link (4/4)
- Commit, repo and star counters lead the hero; they read as volume, not quality. Commits 4,172 and contributions 5,328 also disagree on one page (4/4)
- The heron banner and "every number is measured" put the site's own plumbing first (4/4)
- Card copy is written for Rust insiders: "the cache is a port", Arc<dyn …> (4/4)
- Zwipe has no traction number on the site. The GitHub profile README says ~1,000 users; use it if it holds (3/4)
- "How I Use AI" on the home page raises "did he write this" before the work answers it. Move lower or shrink (2/4)
- Contribute panel (Stripe, Buy Me a Coffee, Sponsors) on the home page is off-message for hiring; footer link instead (2/4)

## Structure

- Side quests dilute the pitch; Gotcha, Upsee, Marvin and the Advent of Code ports named as noise (3/4). See the category proposal
- No team or collaboration evidence anywhere; everything public is solo (3/4)

## Per page

- Marvin: "One upstream bug found and fixed". Both Rig PRs (#1371, #2507) closed unmerged. Say found and reported, with a proposed fix
- Zwipe: "Security audit complete" names no auditor, scope or date
- Zwipe: demo videos sit below the code; show it working first
- Zwipe: an architecture diagram would beat the six-crate sentence
- Zwipe impact line ("six crates, unwrap banned by CI") is an engineering detail, not an outcome
- `context/marketing/og_default.html` and the rendered OG image still say "two storage engines"
