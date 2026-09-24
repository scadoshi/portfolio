# Cairn

## Headline

Lifetime rep counter for iOS. Local SQLite, no server, no account.

## Category

Personal App — Mobile

## What It Is

A counter for things you do every day. Tap a button and it keeps the lifetime total plus the rates that make a total mean something: this year, per day, where you stand against a goal, what today still owes. Rust and Dioxus 0.7, one crate, on my phone since 22 September 2026 with nine months of imported training history.

A cairn is a waymarker built one stone at a time by whoever walks the path, and here the walker is you: the same path, the same small act, every day. The pile is the record of having done it.

## What It Proves

- Hexagonal architecture that held: 2,600 lines of domain that never touch Dioxus, rusqlite or the clock. `today` is an argument, not a call, which is what makes the stats testable and what will let them run on a watch
- Events as the source of truth, with daily totals derived, so changing when a day starts rebuilds history rather than losing it
- Six schema versions migrated on a database holding real data, with the upgrade path tested at every one
- Shipping to a physical device end to end: signing, provisioning, install, and a backup that runs before every deploy

## Key Technical Highlights

Which quote shows is a pure function of the clock hour. Nothing is stored, so nothing can drift, and the stride through the list is coprime with its length so all 72 appear before any repeat.

```rust
pub fn at_hour(hours: i64) -> Option<&'static Quote> {
    let len = QUOTES.len();
    // The empty check is load-bearing, not defensive: rem_euclid(0) is a
    // divide by zero, which took the whole screen down while this list was
    // still being filled in.
    if len == 0 {
        return None;
    }
    let slot = usize::try_from(hours.rem_euclid(i64::try_from(len).ok()?)).ok()?;
    QUOTES.get(slot.wrapping_mul(STEP) % len)
}
```

Goals come in three units and all reduce to what today has to clear, by integer ceiling division. A float there would make "is the day done" depend on the rounding mode, and a year of days each falling a fraction short misses the goal.

## What I Learned

The audit taught more than the build. Mutation testing found three assertions that could not fail, the worst being the migration ladder: changing `if version < 5` to `if version < 4` passed all 64 tests while breaking every install that already had data. Every store test started from an empty database, so no version guard was ever exercised. The one install with real data was the one path with no coverage.

Two bugs reached my phone. A hook read inside a loop meant the first render ran zero hooks and the second ran three, which Dioxus treats as fatal. And `-delta.min(n)` negates the comparison rather than `delta`, turning every subtraction into an addition.

Quote attribution is worse than expected. "You don't stop running because you get old" is Jack Kirk, not McDougall. Tyson never said "punched in the mouth". About a third of the candidates were rejected.

## Roadmap

TestFlight and App Store review. Editing a past day, CSV import, a yearly heat strip. An Apple Watch face eventually, which is why the domain stays pure.

## Status

Doing. In daily use: 225 days, 9,914 taps, 98,300 reps. 6,700 lines, 79 tests.

## Repo

https://github.com/scadoshi/cairn
