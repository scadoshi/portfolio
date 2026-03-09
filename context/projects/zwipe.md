# Zwipe

## Headline

Full-stack MTG deck builder in Rust — Axum backend, Dioxus frontend, PostgreSQL, 35k+ cards.

## Category

Full-Stack Application

## What It Is

A mobile-first Magic: The Gathering deck builder with swipe-based navigation. Two Rust crates in a workspace: an Axum REST API (zerver) and a Dioxus cross-platform app (zwiper) targeting web, iOS, Android, and desktop from a single codebase.

## What It Proves

- Hexagonal architecture applied consistently across ~24,500 lines of Rust
- Domain-driven design with newtypes for type safety (UserId, DeckId, JwtSecret, Username — all validated at construction)
- JWT + rotating refresh tokens (max 5 per user, SHA-256 hashed, 14-day expiry)
- Argon2id password hashing with common password blocklist (170+ patterns, NIST guidelines)
- PostgreSQL with SQLx: 7 migrations, JSONB operators, window functions, composite constraints
- Advanced card search: CMC ranges, dual color identity modes, type/rarity/set filtering
- Background job binary (zervice) for Scryfall delta sync handling 35k+ cards in batches
- Option<Option<T>> for partial updates — distinguishing "not provided" from "set to null"
- Full-stack documentation pass with #![warn(missing_docs)] enabled (243 warnings resolved)
- Clippy configured with 26 enforced lints

## Key Technical Highlights

### Architecture
```
domain/        Pure business logic, no external deps
  models/      Per-operation request/response types
  ports.rs     Trait interfaces (repositories, services)
  services.rs  Business logic orchestration

inbound/       Entry points
  http/        Axum handlers, routes, JWT middleware

outbound/      External systems
  sqlx/        PostgreSQL repositories implementing port traits
```

### Database
- Normalized schema with composite keys and JSONB columns
- Window functions for token limit enforcement
- QueryBuilder with JSONB operators (@>, <@, ?|) for card search
- Batch upserts respecting PostgreSQL parameter limits (~327 cards/batch)

### Auth Flow
- Access tokens: 24-hour JWT (self-contained, no DB lookup)
- Refresh tokens: 14-day rotating, SHA-256 hashed in DB, max 5 per user
- Background cleanup job deletes expired tokens

## What I Learned

- How to structure a large Rust project across multiple crates with shared domain types
- Hexagonal architecture in practice — port traits make testing and swapping implementations possible
- The difference between access and refresh token strategies and why you hash refresh tokens
- PostgreSQL JSONB for semi-structured data (card legalities, prices, image URIs)
- Dioxus signals for reactive state management in a Rust UI framework

## Status

Active development. Auth, card database, deck management, and card search are complete. Working on deck card browser with full-screen swipeable card viewer.

## Repo

~/Work/zwipe
