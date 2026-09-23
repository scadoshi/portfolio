# Steller

## Headline

Redis-compatible key-value server, hand-written from the wire protocol up. RESP over TCP, snapshot plus append-only log, pub/sub.

## Category

Learning Project — Network Protocols and Durability

## What It Is

An in-memory key-value server that real `redis-cli` clients connect to without knowing
the difference. It speaks RESP over TCP and handles ping, get, set, delete, exists,
relative and absolute TTLs, TTL queries, persist, and channel subscription. Data
survives a restart on a snapshot baseline plus an append-only command log. About 5,900
lines with 238 tests.

Named for the Steller's jay. It was diprotodon until 2026-09-13; anything still saying
diprotodon is stale.

## What It Proves

- Hexagonal architecture held honestly: `CacheRepository` and `CacheService` are traits
  in the domain, and the domain has no TCP, no files and no clock
- Parser as framer, where `Incomplete` is a load-bearing error variant rather than a
  failure: the same function decides whether a frame is complete and what it means
- The AOF is the wire protocol, so replay reuses the inbound parse path instead of a
  second format nobody tests
- Benchmarked rather than asserted, including the part where the design loses

## Key Technical Highlights

### Time-invariant replay

Relative TTLs are unrepresentable in `WriteCommand`. They are normalized to an absolute
deadline at parse time, so replaying a log an hour later produces the same state rather
than extending every expiry. `domain/time.rs` has `Seconds` and `Milliseconds` newtypes
so a unit mismatch is a compile error.

The bug this design was supposed to prevent still happened once: `ExpireAt` held
milliseconds but encoded as `EXPIREAT`, whose parse arm multiplies by 1000. Every
restart pushed deadlines 1000x further out, silently. Caught before shipping by a test
that asserts a set deadline survives replay exactly.

### Benchmarks

`redis-benchmark`, SET throughput, median of three runs, against real Redis 8 on the
same laptop:

| clients | steller | Redis 8 |
| ---: | ---: | ---: |
| 1 | 52,743 | 15,380 |
| 16 | 153,374 | 88,496 |
| 256 | 151,976 | 176,678 |

Under ~32 clients a thread blocked in `read()` wakes faster than an event-loop
iteration. Past that, a thread per connection around one mutex flattens while Redis
keeps climbing, and near 3,000 clients the process hits macOS's thread cap and dies at
two threads per client. That is the tradeoff of the design, measured.

## What I Learned

- RESP is small enough to hand-write and unforgiving enough to teach framing properly
- Making an invalid state unrepresentable moves the bug rather than removing it: the
  1000x TTL bug lived in the encoder, one layer below where the newtypes reach
- A shutdown path deadlocks when a reader's `Sender` outlives the join: the writer parks
  in `recv()`, which only returns once every sender is dropped

## Status

Done. Set/expiry work merged and the smoke tests are green.

## Repo

~/Developer/steller, github.com/scadoshi/steller
