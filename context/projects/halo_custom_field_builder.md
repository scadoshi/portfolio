# Halo Custom Field Builder

## Headline

Shipped CLI tool for bulk-creating custom fields in Halo ITSM — cross-platform binaries via GitHub Actions.

## Category

Production Data Tooling

## What It Is

A CLI tool that reads custom field definitions from CSV files and creates them in Halo via the API. Supports all 8 Halo field types with input type sub-variants, OAuth 2.0 authentication, rate limiting, and interactive debug mode. Distributed as cross-platform binaries (Windows, macOS Intel/ARM, Linux).

## What It Proves

- Domain modeling with validation: Name (alphanumeric + underscore, max 64), Label (visible chars, max 256), FieldType enum with 8 variants and input type sub-enums — all validated at construction via TryFrom
- Two-layer serialization: CustomField (domain) maps to HttpCustomField (API representation)
- OAuth 2.0 client credentials flow with automatic token refresh and 30-second expiry buffer
- Rate limiting (500ms between requests) to stay under Halo's 700/5min API limit
- Interactive debug mode: field-by-field review with skip/process/quit
- GitHub Actions CI/CD: cross-platform builds, distribution packaging, tagged v1.0.0 release
- Same layered architecture (domain/inbound/outbound) applied at smaller scale (~727 LOC)

## Key Technical Highlights

### Field Type System
```rust
FieldType::Text(TextInputType)           // 7 input variants
FieldType::SingleSelect(SingleSelectInputType)  // 3 input variants
FieldType::Date(DateInputType)           // 2 input variants
FieldType::Memo | MultiSelect | Time | Checkbox | Rich  // no input types
```

### CSV Validation
- Header validation (exact column names required)
- Row-by-row parsing with error context (row numbers, specific field issues)
- Optional field handling (input_type_id, selection_options)
- Selection options comma normalization

## What I Learned

- How to ship Rust binaries to non-technical users via GitHub Actions
- OAuth 2.0 client credentials flow in practice
- Rate limiting as a design consideration, not an afterthought
- The value of debug/dry-run modes for tools that write to production APIs

## Status

Shipped. Tagged v1.0.0 with cross-platform releases.

## Repo

~/Work/halo_custom_field_builder
