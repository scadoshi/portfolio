# Halo Action Importer

## Headline

Production bulk import tool — millions of records, resilient retry, incremental caching.

## Category

Production Data Tooling

## What It Is

A CLI tool for bulk importing actions into Halo ITSM from CSV and Excel files. Built for real migrations involving millions of records against a production API with real failure modes. Supports batched processing, parallel execution, and automatic recovery.

## What It Proves

- Production-grade error recovery: infinite retry on network/timeout failures, automatic token refresh on 401s
- Ticket-grouped retry logic: when a batch fails due to missing tickets, splits by ticket_id and retries each group independently — maximizes successful imports while minimizing API calls
- Incremental caching with file locking (fs2): tracks fetched report IDs and imported action IDs, survives process restarts
- Structured output per run: log/YYYY-MM-DD_HH-MM-SS/ directory with full.log, retry.csv (failed actions for re-import), and summary.json
- Parallel execution: split input files across directories and run multiple instances
- CLI with practical flags: --batch-size, --only-parse (validation mode), --only-cache, --input-path

## Key Technical Highlights

### Retry Strategy Evolution
Started with basic retry, hit batch failures where some tickets didn't exist. First implemented binary search to find the bad ticket. Realized binary search was inefficient — replaced with ticket-grouped retry that groups actions by ticket_id, retries each group, and marks missing tickets as permanently failed. Commit history shows this progression.

### Resilience Pattern
```
401 Unauthorized  → refresh token, retry immediately
504 Gateway Timeout → retry immediately (no delay)
Network error     → retry immediately
Missing ticket    → mark ticket as missing, skip future actions for it
Deserialization   → skip row, continue processing
```

### Cache Architecture
- cache/existing.json — tracks already-fetched report IDs with resource metadata
- cache/imported/ — append-only tracking of successfully imported action IDs
- File locking via fs2 prevents corruption from parallel instances

## What I Learned

- How to build software that runs unattended for hours against unreliable APIs
- Why binary search retry was the wrong abstraction for batch failures (ticket-grouped is O(unique_tickets) vs O(log(batch_size) * failures))
- File locking for concurrent cache writes — hit the bug, fixed it properly
- Structured logging for production debugging: timestamps, per-action stats, ETAs

## Status

Production. Actively used for real data migrations.

## Repo

~/Work/halo_action_importer
