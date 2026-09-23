# Sharpmas

## Headline

Rustmas ported to C#, feature for feature. A fixed design carried into another language to see where it bends.

## Category

Learning Project — Language Port

## What It Is

The same Advent of Code tooling as rustmas, written in C#: fetch input, run the
solution, check it against an independent solver, submit.

Counts depend on the branch. `main`, the tool alone, is ~2,600 lines. `scadoshi`, with
solutions, is ~3,600 lines and `dotnet test` reports 134 passing. That 134 is test
cases, not test methods: `[Theory]` with several `InlineData` rows counts once in the
source and many times in the run, which is why attribute counts and run counts
disagree here and not in rustmas.

The design was settled before the port started, which is the point. What the repo
records is where the two languages actually diverge, and where C# has no good answer.

## Branch Layout

Same as rustmas. `main` is the tool with no solutions and is the default branch;
`scadoshi` carries the solutions. Nothing puzzle-derived goes on `main`.

## What It Proves

- Porting a settled design without transliterating it: C# that compiles and reads like
  Rust is the failure mode, not the goal
- Comfort with the parts of C# that have no Rust equivalent, and honesty about the
  parts that have no C# equivalent

## Key Technical Highlights

### The closest C# gets to a trait

Rust's `fn new(input) -> Self` becomes a static abstract interface member, reachable
only through a type parameter since a static member has no receiver. The consequence is
structural: nothing can hold an `ISolution` and call `Parse` on it, so every caller down
to the registry knows the concrete type, exactly as the Rust side does through
monomorphized generics.

### Closing the set of cases

`Answer` is an abstract record with a private constructor and sealed nested cases, so
the hierarchy cannot be extended outside the file. The honest part is the last switch
arm: C# cannot prove the switch exhaustive, so the impossible case still has to be
written down and thrown. Rust's compiler proves it and the arm does not exist.

## What I Learned

- `Enumerable.Range` takes a count, not an end. Two bugs while porting the day filter,
  both caught by tests mirrored from rustmas before anything ran
- Matching Rust's phrasing in guard messages meant fighting the analyzer for a worse
  message than the framework already produces. The port is better where it stops
  imitating

## Status

Matches rustmas feature for feature. Two days solved so far (2015 day 1, 2016 day 1),
every answer confirmed by the solver and matching rustmas. What is left is solutions and
the shared helpers they will want.

## Repo

~/Developer/sharpmas, github.com/scadoshi/sharpmas
