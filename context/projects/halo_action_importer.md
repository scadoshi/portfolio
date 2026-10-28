# Halo Action Importer

## Headline

Production bulk import tool: millions of records, resilient retry, incremental caching.

## Category

Production Data Tooling

## What It Is

A CLI tool for bulk importing actions into Halo ITSM from CSV and Excel files. Built for real migrations involving millions of records against a production API with real failure modes. Supports batched processing, parallel execution, and automatic recovery.

## What It Proves

- Error recovery per failure mode: one token refresh on a 401, three report tries a minute apart, posts never resent (their batch goes to retry.csv)
- Ticket-grouped retry logic: when a batch fails due to missing tickets, splits by ticket_id and retries each group independently, which maximizes successful imports while minimizing API calls
- Incremental caching with file locking (fs2): tracks fetched report IDs and imported action IDs, survives process restarts
- Structured output per run: log/YYYY-MM-DD_HH-MM-SS/ directory with full.log, retry.csv (failed actions for re-import), and summary.json
- Parallel execution: split input files across directories and run multiple instances
- CLI with practical flags: --batch-size, --only-parse (validation mode), --sync-from-halo, --fresh, --input-path

## Key Technical Highlights

### Retry Strategy Evolution
Started with basic retry, hit batch failures where some tickets didn't exist. First implemented binary search to find the bad ticket. Realized binary search was inefficient and replaced it with ticket-grouped retry that groups actions by ticket_id, retries each group, and marks missing tickets as permanently failed. Commit history shows this progression.

### Resilience Pattern
```
401 Unauthorized       → refresh the token once, try again
504 or drop, report    → try again a minute later, three tries
504 or drop, post      → never resent; the batch goes to retry.csv
Missing ticket (404)   → skip its actions for the rest of the run
Row that will not parse → count it failed, write it to unparsed.csv
```

### Cache Architecture
- cache/existing.json: tracks already-fetched report IDs with resource metadata
- cache/imported/: append-only tracking of successfully imported action IDs
- File locking via fs2 prevents corruption from parallel instances

## What I Learned

- How to build software that runs unattended for hours against unreliable APIs
- Why binary search retry was the wrong abstraction for batch failures (ticket-grouped is O(unique_tickets) vs O(log(batch_size) * failures))
- File locking for concurrent cache writes: hit the bug, fixed it properly
- Structured logging for production debugging: timestamps, per-action stats, ETAs

## Status

Production. Actively used for real data migrations.

## Repo

~/Developer/halo_action_importer
