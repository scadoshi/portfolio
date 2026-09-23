# Cairn

## Headline

Lifetime rep counter for iOS. Local SQLite, no server, no account.

## Category

Personal App — Mobile

## What It Is

A counter for things you do every day. Name the thing, tap a button, and it keeps the running total plus the rates that make a total mean something: this year, per day, where you stand against a goal, what today still owes. Rust and Dioxus 0.7, one crate, running on my phone since 22 September 2026 with nine months of imported training history.

A cairn is the pile of stones on a mountain path where everyone who passes adds one. The act is identical every time and the pile is the whole point, which is what a lifetime counter is.

## What It Proves

- Hexagonal architecture held under pressure: 2,400 lines of domain that never touch Dioxus, rusqlite or the clock. `today` is an argument, not a call, which is what makes the stats testable and what will let them run on a watch
- Events as the source of truth, with per-day totals derived. Changing when a day starts rebuilds history rather than losing it
- SQLite migrations across five schema versions on a database holding real data
- Shipping to a physical device end to end: development signing, provisioning, install, and a backup script that runs before every deploy
- Mutation testing a suite that looked healthy and finding three assertions that could not fail

## Key Technical Highlights

### The quote with no state

Which quote shows is a pure function of the clock hour. Nothing is stored, so nothing can drift, and every launch inside the same hour agrees. The stride through the list is coprime with its length, so all 41 appear before any repeat.

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

### Goals in three units, one daily target

A goal is per day, per week or per year, and all three reduce to what today has to clear. Integer ceiling division, because a float there would make "is the day done" depend on the rounding mode, and a year of days each falling a fraction short misses the goal.

```rust
pub fn daily_target(self, days_in_year: u32) -> u32 {
    let ceil_div = |n: u32, d: u32| {
        let d = d.max(1);
        n / d + u32::from(!n.is_multiple_of(d))
    };
    match self {
        Self::PerDay(n) => n,
        Self::PerWeek(n) => ceil_div(n, 7),
        Self::PerYear(n) => ceil_div(n, days_in_year),
    }
}
```

### A streak that spans counters

The home screen's streak is not the best of the per-counter streaks. Every counter's history merges by day first, so it counts days anything at all was logged. Two counters that reach 2 and 1 alone make 3 together, which is the run that is actually hard to break.

## What I Learned

The audit taught more than the build. Mutation testing found three assertions that could not fail, and the worst was the migration ladder: changing `if version < 5` to `if version < 4` passed all 64 tests while breaking every install that already had data. Every store test started from an empty in-memory database, so no version guard was ever exercised. The one install with real data in it was the one path with no coverage.

Two bugs reached my phone. A hook read inside a loop over counters meant the first render ran zero hooks and the second ran three, which Dioxus treats as fatal, and the home screen died on launch. `dx check` catches that class, but it had been failing on an unrelated call for weeks, so nobody could run it; one word fixed it and it is a CI job now. The other was `-delta.min(n)`, which negates the comparison rather than `delta`, turning every subtraction into an addition.

Quote attribution is worse than I expected. "You don't stop running because you get old" is Jack Kirk, not McDougall. "You have power over your mind" appears in no published translation of Meditations. Tyson never said "punched in the mouth"; he said "everybody has plans until they get hit for the first time". About a third of the candidates were rejected.

## Roadmap

TestFlight and App Store review. Editing a past day, CSV import, and a yearly heat strip per counter. CloudKit sync if it ever needs to be on two devices. An Apple Watch face eventually, which is the reason the domain stays pure.

## Status

Doing. In daily use: 224 days, 9,829 taps, 97,600 reps across pushups, pullups and squats. 6,250 lines, 73 tests.

## Repo

https://github.com/scadoshi/cairn
