# Rustmas

## Headline

Advent of Code tooling in Rust. Fetches inputs, runs solutions, checks them against an independent solver, submits for stars.

## Category

Learning Project — CLI Tooling

## What It Is

A CLI that handles everything around an Advent of Code puzzle except the puzzle: it
downloads the input with your session cookie, runs your solution, verifies the answer
against a second independent solver before anything is submitted, and posts it. About
9,100 lines with 186 tests.

## Branch Layout

Two branches, deliberately:

- `main` is the tool with no solutions. This is the one to clone as a starting point,
  and it is the repo's default.
- `scadoshi` is the working branch with Scotty's solutions attached.

Anything that would leak puzzle text, inputs or answers belongs on `scadoshi`, never on
`main`. AoC asks people not to redistribute inputs, which is why the demo shows only a
single day.

## What It Proves

- A CLI shaped around a real workflow rather than a menu of commands
- Every line of input parses into a typed variant before the runner sees it, so bad
  input fails at the boundary
- The answer is checked twice by independent code paths before it costs a submission
  attempt

## Key Technical Highlights

### The day registry

One arm per day, returning the solver rather than calling it, so the dispatch table
stays a pure lookup and the runner owns timing and error handling.

### Merging the two verdicts

AoC's word supersedes the solver's, so a part that is already starred reads as correct
even when the local solver disagrees. That ordering matters: the site is the authority,
the solver is the pre-flight check.

## What I Learned

- Where a session cookie can and cannot appear. The SHA-256 shown in the demo is a
  one-way hash of a 128-hex-char value, which is not reversible and not worth a
  rainbow table
- Designing the tool and the solutions as separate branches from the start is what made
  the repo shareable at all

## Status

Done. `sharpmas` is the same tool in C#, and the two are meant to be read side by side.

## Repo

~/Developer/rustmas, github.com/scadoshi/rustmas
