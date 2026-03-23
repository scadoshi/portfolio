pub struct Project {
    pub name: &'static str,
    pub slug: &'static str,
    pub headline: &'static str,
    pub category: &'static str,
    pub repo_url: &'static str,
    pub summary: &'static str,
    pub impact_metric: &'static str,
    pub impact_detail: &'static str,
    pub objective: &'static str,
    pub approach: &'static [&'static str],
    pub snippets: &'static [Snippet],
    pub obstacles: &'static [&'static str],
    pub progress: &'static str,
    pub impact: &'static str,
}

pub struct Snippet {
    pub title: &'static str,
    pub code: &'static str,
    pub description: &'static str,
}

pub fn featured_projects() -> &'static [Project] {
    &[ZWIPE, HALO_ACTION_IMPORTER, HALO_CUSTOM_FIELD_BUILDER]
}

pub fn side_quests() -> &'static [Project] {
    &[NIGHTHAWK, MARVIN, CAPTURE, UPSEE]
}

pub fn find_project(slug: &str) -> Option<&'static Project> {
    featured_projects().iter().find(|p| p.slug == slug)
}

pub fn find_side_quest(slug: &str) -> Option<&'static Project> {
    side_quests().iter().find(|p| p.slug == slug)
}

const ZWIPE: Project = Project {
    name: "Zwipe",
    slug: "zwipe",
    headline: "Full-stack MTG deck builder. Axum backend, Dioxus frontend, PostgreSQL, 100k+ cards.",
    category: "Full-Stack Application",
    repo_url: "https://github.com/scadoshi/zwipe",
    summary: "A mobile-first Magic: The Gathering deck builder with swipe-based navigation. Three binaries in a Cargo workspace: zerver (Axum REST API), zwiper (Dioxus cross-platform app), and zervice (background task runner). The frontend imports the backend as a library dependency for shared domain types.",
    impact_metric: "~25,800 lines of production Rust",
    impact_detail: "Hexagonal architecture with strict type safety throughout. Every domain boundary enforced at the type level. Production-strict linting: .unwrap(), .expect(), panic!, todo!, dbg!, and print! are all denied at compile time. 33 enforced Clippy rules.",
    objective: "Build a full-stack MTG deck builder with swipe-based navigation, targeting web, iOS, Android, and desktop from a single Rust codebase. Three workspace binaries: zerver (Axum REST API), zwiper (Dioxus frontend), and zervice (background service for card sync and session cleanup).",
    approach: &[
        "Hexagonal architecture applied consistently across ~25,800 lines of Rust. Port traits define what operations are needed (AuthRepository, CardRepository, DeckRepository). Adapters implement those ports for specific technologies. Domain logic has zero external dependencies",
        "Domain-driven design with validated newtypes: Username (3-20 chars, profanity filter), Password (8-128 chars, uppercase/lowercase/digit/symbol required, max 3 consecutive repeats, checked against common password dictionary), EmailAddress, UserId, DeckId, JwtSecret. Invalid data is unrepresentable",
        "CardFilter builder (30+ fluent setters) drives both SQL and in-memory execution. FilterCards and GroupCards are extension traits on Vec<Card>: the frontend can filter and partition a local deck collection without a server round-trip using the exact same criteria as the SQL adapter. Grouping by card type, mana cost, or color with enum-dispatched classification",
        "Structured error chain: SQLx errors → PostgreSQL constraint violation detection (unique=23505, check=23514) → domain-specific error enums (RegisterUserError::Duplicate) → HTTP status codes (409 Conflict). Internal details logged but never exposed to clients",
        "JWT access tokens (HS256, 24-hour expiry) + rotating refresh tokens (max 5 per user, SHA-256 hashed, 14-day expiry). Old refresh token deleted on use, preventing replay attacks. Session limits auto-enforced by background service",
        "Argon2id password hashing with OS-random salts (resistant to GPU/ASIC attacks). Common password blocklist with 170+ patterns following NIST guidelines. Password type consumed after hashing so plaintext can never be reused",
        "PostgreSQL with compile-time verified SQLx queries: 7 migrations, JSONB operators (@> contains, <@ contained by, ?| has any key), dynamic query building, bulk upsert with automatic card-by-card fallback on batch failure, PartialEq-based delta detection to skip unchanged records during Scryfall sync",
        "Background service binary (zervice): hourly Scryfall delta sync handling 100k+ cards in batches of 327 (respecting PostgreSQL's 65k parameter limit), expired refresh token cleanup, max session enforcement",
        "Custom swipe gesture engine across 10 files: OnSwipe core trait with OnTouch (mobile) and OnMouse (desktop) adapters. Velocity-based and distance-based dual-threshold detection, axis locking to prevent cross-axis drift, dynamic return animation scaled to swipe distance. Built from scratch, not a library",
        "Dioxus signal architecture: Upkeep trait extends Signal<Option<Session>> with background auto-refresh (60s interval, refreshes access token before expiry, clears session on failure). Central context provider initializes session, HTTP client, card filter state, and search results for consumption across all screens",
        "Production-strict linting: .unwrap(), .expect(), panic!, todo!, dbg!, and print! all denied at compile time. 33 enforced Clippy rules. Full documentation pass with #![warn(missing_docs)]",
    ],
    snippets: &[
        Snippet {
            title: "Hexagonal Architecture",
            code: r#"domain/        Pure business logic, no external deps
  models/      Per-operation request/response types
  ports.rs     Trait interfaces (repositories, services)
  services.rs  Business logic orchestration

inbound/       Entry points
  http/        Axum handlers, routes, JWT middleware

outbound/      External systems
  sqlx/        PostgreSQL repositories implementing port traits"#,
            description: "Clean separation of concerns. Domain logic has zero external dependencies. Port traits make testing and swapping implementations straightforward.",
        },
        Snippet {
            title: "Trait-Based Card Filtering & Grouping",
            code: r#"// Extension traits on Vec<Card> — no wrapper types, just import the trait
pub trait FilterCards {
    fn filter_by(self, filter: &CardFilter) -> Vec<Card>;
}
pub trait GroupCards {
    fn group_by(self, option: GroupByOption) -> Vec<CardGroup>;
}

impl FilterCards for Vec<Card> {
    fn filter_by(self, filter: &CardFilter) -> Vec<Card> {
        let mut cards: Vec<Card> = self.into_iter()
            .filter(|card| { /* 20+ criteria: text, color, CMC, power,
                               rarity, type, set, artist, legality... */ })
            .collect();
        // Sort by enum dispatch (name, CMC, rarity, random, etc.)
        // Paginate: .skip(offset).take(limit)
        cards
    }
}

impl GroupCards for Vec<Card> {
    fn group_by(self, option: GroupByOption) -> Vec<CardGroup> {
        let labels = match option {
            GroupByOption::CardType => vec!["lands", "creatures", ...],
            GroupByOption::Cmc      => vec!["0", "1", "2", ... "6+"],
            GroupByOption::Color    => vec!["white", "blue", ... "colorless"],
        };
        let mut buckets = vec![Vec::new(); labels.len()];
        for card in self { buckets[classify(&card, option)].push(card); }
        // zip labels + buckets, drop empties
    }
}

// Usage: deck_cards.filter_by(&filter).group_by(GroupByOption::CardType)"#,
            description: "Same CardFilter drives both the SQL adapter (server-side) and these in-memory traits (client-side). The frontend can filter a local deck without a round-trip using the exact same criteria. Extension traits mean Vec<Card> gains these methods just by importing the trait.",
        },
        Snippet {
            title: "CardFilter Builder Pipeline",
            code: r#"// Builder with 30+ fluent setters, validates on build()
let filter = CardFilterBuilder::default()
    .set_color_identity_within(colors)  // -> &mut Self
    .set_cmc_range((2.0, 5.0))
    .set_type_line_contains("Creature")
    .set_rarity_equals_any(rarities)
    .set_is_valid_commander(true)
    .set_order_by(OrderByOption::Cmc)
    .set_limit(50)
    .build()?;  // -> Result<CardFilter, InvalidCardFilter>

// Same filter works server-side (SQL) and client-side (in-memory)
let results = card_service.search(&filter).await?;  // SQL adapter
let local   = deck_cards.filter_by(&filter);         // Vec<Card> trait
let groups  = local.group_by(GroupByOption::CardType); // partition"#,
            description: "One filter type, two execution paths. The builder validates that at least one search criterion is set (not just pagination). 30+ setters cover every MTG search dimension: colors, mana cost, power/toughness ranges, text search, rarity, set, artist, legality, commander validity.",
        },
        Snippet {
            title: "Swipe Gesture Engine",
            code: r#"// Core trait: platform-agnostic swipe logic
trait OnSwipe {
    fn onswipestart(&mut self, point: ClientPoint);
    fn onswipemove(&mut self, point: ClientPoint);
    fn onswipeend(&mut self, point: ClientPoint, config: &SwipeConfig);
}

// Platform adapters: same core, different event types
impl OnTouch for Signal<SwipeState> {
    fn ontouchstart(&mut self, e: Event<TouchData>) {
        self.with_mut(|ss| ss.onswipestart(e.client_coordinates()));
    }
}
impl OnMouse for Signal<SwipeState> {
    fn onmousedown(&mut self, e: Event<MouseData>) {
        self.with_mut(|ss| ss.onswipestart(e.client_coordinates()));
    }
}

// Detection: dual threshold (distance OR velocity) + axis locking
fn set_latest_swipe(&mut self, config: &SwipeConfig) {
    if distance > config.distance_threshold
        || (distance > 10.0 && speed > config.speed_threshold)
    {
        match self.traversing_axis {
            Some(Axis::X) if delta.x < 0.0 => self.latest_swipe = Some(Dir::Left),
            Some(Axis::X) if delta.x > 0.0 => self.latest_swipe = Some(Dir::Right),
            Some(Axis::Y) if delta.y < 0.0 => self.latest_swipe = Some(Dir::Up),
            Some(Axis::Y) if delta.y > 0.0 => self.latest_swipe = Some(Dir::Down),
            _ => {}
        }
    }
}

// Swipeable component: reactive transform follows finger/cursor
rsx! { div {
    style: "transform: translate({xpx}px, {ypx}px);
            transition: transform {return_seconds}s;",
    ontouchstart, ontouchmove, ontouchend,
    onmousedown, onmousemove, onmouseup,
    { children }
} }"#,
            description: "10 files, zero library dependencies. OnSwipe defines the gesture logic once; OnTouch and OnMouse adapt it to platform events. Axis locks on first movement so diagonal drags don't fire both directions. Velocity tracking (pixels/ms between consecutive points) lets quick flicks register even below the distance threshold. The Swipeable component renders a reactive CSS transform that follows the user's finger in real time.",
        },
        Snippet {
            title: "88-Column Upsert Automation",
            code: r#"// Single source of truth: 88 field names, line-separated
const SCRYFALL_DATA_FIELDS: &str = "
    arena_id  id  lang  mtgo_id  oracle_id  cmc
    color_identity  colors  power  toughness  type_line
    ...  (88 fields total)
";

// Derived helpers — all read from the same constant
fn scryfall_data_fields() -> String { /* comma-join for INSERT */ }
fn bulk_upsert_conflict_fields() -> String {
    // "ON CONFLICT (id) DO UPDATE SET arena_id = EXCLUDED.arena_id, ..."
}

// Trait-based binding: QueryBuilder gains card methods
trait BindScryfallDataFields {
    fn bind_scryfall_fields(&mut self, card: &ScryfallData) -> &mut Self;
}
trait BindCards {
    fn bind_cards(&mut self, data: &[ScryfallData]) -> &mut Self;
}

// Result: one fluent chain builds the entire 88-column upsert
QueryBuilder::new("INSERT INTO scryfall_data (")
    .push(scryfall_data_fields())
    .push(") VALUES ")
    .bind_cards(scryfall_data)
    .push(bulk_upsert_conflict_fields())
    .push(" RETURNING *;")

// Upsert strategy chain — each layer adds a capability:
// BatchDeltaUpsertWithTx (chunk + skip unchanged)
//   → BulkDeltaUpsertWithTx (PartialEq diff against DB)
//     → BulkUpsertWithTx (single SQL statement)
//       → SingleUpsertWithTx (card-by-card fallback)
// One bad card never blocks the rest of the batch"#,
            description: "Strict hex-arch would demand a separate DatabaseScryfallData DTO, but maintaining 88 fields on two types is untenable solo. Compromise: feature-gated #[cfg_attr(feature = \"zerver\", derive(sqlx::FromRow))] on the domain type. A constant feeds all SQL generation. Trait-based binding keeps the calling code clean. Five upsert strategies compose via traits — delta detection skips unchanged cards, batching respects PostgreSQL's 65k parameter limit, and automatic fallback to card-by-card ensures one bad record never blocks 100k others.",
        },
    ],
    obstacles: &[
        "ScryfallData has 88 fields. Strict hexagonal architecture would demand a separate DatabaseScryfallData DTO, but maintaining 88 fields across two types plus mapping between them is untenable solo. Pragmatic compromise: feature-gated derive on the domain type, a single constant feeding all SQL generation, and trait-based binding automation. Bend the rule once, automate everything around it",
        "PostgreSQL's 65,535 parameter limit meets 88 fields per card: max ~327 cards per batch. Five upsert strategies compose via traits — delta detection skips unchanged cards, batching chunks within the parameter limit, and automatic card-by-card fallback ensures one bad record never blocks 100k others",
        "Swipe gesture detection required solving axis locking, velocity vs distance thresholds, and cross-platform input (touch vs mouse). Built from scratch across 10 files with a trait hierarchy rather than pulling in a gesture library",
    ],
    progress: "Auth, card database, deck management, and card search complete. Working on deck card browser with full-screen swipeable card viewer.",
    impact: "Demonstrates complete full-stack capability in Rust: database migrations, JWT auth with refresh token rotation, reactive frontend, background services, all in one language with shared domain types between frontend and backend.",
};

const HALO_ACTION_IMPORTER: Project = Project {
    name: "Halo Action Importer",
    slug: "halo-action-importer",
    headline: "Production bulk import tool. Millions of records, resilient retry, incremental caching.",
    category: "Production Data Tooling",
    repo_url: "https://github.com/scadoshi/halo_action_importer",
    summary: "CLI tool for bulk importing actions into the Halo Software suite from CSV and Excel files. Layered architecture (inbound/domain/outbound) with bin/lib crate split. Built for real migrations involving millions of records against production APIs with real failure modes: messy data formats, unreliable networks, and hours-long unattended runtimes. ~3,230 LOC across 20 files.",
    impact_metric: "Weeks of manual work, automated",
    impact_detail: "Halo's built-in browser import runs one entry at a time, loses progress on page refresh, and offers minimal error handling. For millions of entries that's literal weeks of runtime coordinating spreadsheets manually. This tool: dump everything into one file, run it, and forget it. Split files across directories for parallel execution when you need faster runtimes.",
    objective: "Build a CLI tool for bulk importing actions into Halo Software products from CSV and Excel files. Must handle millions of records against a production API with real failure modes: network errors, token expiry, missing tickets, partial batch failures, and wildly inconsistent data formats across client exports.",
    approach: &[
        "Production-grade error recovery: infinite retry on network/timeout failures, automatic token refresh on 401s, permanent skip on missing tickets via HashSet<u32> tracked across the entire run",
        "Ticket-grouped retry logic: when a batch fails with a 'not found' error, group actions by ticket_id and retry each group independently to maximize successful imports and identify exactly which tickets don't exist",
        "Deduplication cache that evolved through three stages as the dataset grew: (1) single report endpoint fetching existing IDs from Halo before each run, (2) split across multiple report resources when the single endpoint started timing out under load, (3) fully local cache built from a direct database query of ~8 million action IDs when even the split reports couldn't serve that volume. Each stage required rethinking how the tool remembers its own work",
        "Two-tier incremental caching with fs2 file locking: JSON cache tracks existing IDs per Halo report resource, text file tracks IDs imported during the current run (append-only for speed). Both survive process restarts and support concurrent writes. --only-cache flag skips report fetching entirely when using a manual cache",
        "Structured output per run: log/YYYY-MM-DD_HH-MM-SS/ directory with full.log, retry.csv (re-importable), and summary.json with performance metrics, error breakdown by type, and affected ticket IDs",
        "Real-time progress: ETA based on rolling average batch times, entries/minute throughput, per-sheet timing. You can see exactly where a multi-hour run stands",
        "CLI with practical flags: --batch-size, --only-parse (validate everything without API calls), --only-cache (skip report fetching), --input-path",
    ],
    snippets: &[
        Snippet {
            title: "Resilience Pattern",
            code: r#"// Every failure mode has a specific recovery strategy
401 Unauthorized    → refresh token, retry immediately
504 Gateway Timeout → retry immediately (no delay)
Network error       → retry immediately
Missing ticket      → mark ticket as missing, skip future actions
Deserialization     → skip row, continue processing"#,
            description: "No blanket retry-with-backoff. Each failure mode gets the recovery strategy that actually makes sense for it.",
        },
        Snippet {
            title: "Retry Strategy Evolution",
            code: r#"// v1: Binary search to find bad ticket in failed batch
//     O(log(batch_size) * failures) — too many API calls
//
// v2: Ticket-grouped retry (current)
//     Group actions by ticket_id, retry each group
//     O(unique_tickets) — maximizes successful imports
//     Missing tickets marked permanently failed
fn retry_by_ticket_group(batch: Vec<Action>) -> Result<Stats> {
    let groups = batch.group_by(|a| a.ticket_id);
    for (ticket_id, actions) in groups {
        match import_group(&actions).await {
            Ok(_) => stats.success += actions.len(),
            Err(e) if e.is_missing_ticket() => {
                cache.mark_missing(ticket_id);
                stats.skipped += actions.len();
            }
            Err(e) => return Err(e),
        }
    }
}"#,
            description: "The commit history shows this progression. Binary search was clever but wrong. Ticket-grouped retry is simpler and more efficient.",
        },
        Snippet {
            title: "Cache Evolution",
            code: r#"// v1: Single report endpoint
//     Fetch all existing IDs from Halo before each run
//     Worked fine at ~100k IDs. Timed out at ~1M+
//
// v2: Split across multiple report resources
//     Each resource serves a subset of IDs
//     Cache per-resource to avoid refetching
//     Still hit timeouts as dataset grew to millions
//
// v3: Manual cache from direct database query (current)
//     Query Halo DB for all ~8M existing action IDs
//     Store locally, merge with per-run imported IDs
//     --only-cache flag skips report fetching entirely
fn read_cached_ids() -> CacheData {
    let mut action_ids = HashSet::new();
    // JSON cache: resource-grouped existing IDs
    // Text cache: imported IDs (append-only per run)
    // Both locked with fs2 for concurrent access
}"#,
            description: "The biggest obstacle was remembering work already done. Each stage worked until the dataset outgrew it. I was the only one importing so a local cache was safe as the source of truth.",
        },
    ],
    obstacles: &[
        "The biggest challenge was deduplication at scale. Started with a single Halo report endpoint serving existing action IDs before each run. That worked at ~100k IDs but started timing out as the dataset grew. Split across multiple report resources with per-resource caching. That bought time but eventually even the split reports couldn't serve millions of IDs without timing out. Final solution: query the Halo database directly for all ~8 million existing action IDs, store them locally, and trust the local cache as the source of truth. Safe because I was the only one importing, which I could assure in my instance",
        "Binary search retry was the wrong abstraction for batch failures. Replaced with ticket-grouped retry that's both simpler and more efficient",
        "File locking for concurrent cache writes: hit corruption bugs when running parallel instances against the same cache directory. Fixed properly with fs2 exclusive locks on both cache files",
        "Building software that runs unattended for hours against unreliable APIs required thinking through every failure mode. A crash at hour 3 of a 4-hour run would mean starting over without the caching layer",
    ],
    progress: "Production. Actively used for real data migrations.",
    impact: "Reduced data migration timelines from weeks to days. Runs unattended for hours processing millions of records with automatic recovery from any transient failure.",
};

const HALO_CUSTOM_FIELD_BUILDER: Project = Project {
    name: "Halo Custom Field Builder",
    slug: "halo-custom-field-builder",
    headline: "Shipped CLI tool. Bulk-creating custom fields across Halo Software products with cross-platform binaries.",
    category: "Production Data Tooling",
    repo_url: "https://github.com/scadoshi/halo_custom_field_builder",
    summary: "CLI tool that reads custom field definitions from CSV and creates them across Halo Software products via API. Layered architecture (inbound/domain/outbound) with bin/lib crate split. Type-safe domain modeling, OAuth 2.0 with token caching, interactive debug TUI, import results tracking, and log management. Cross-platform binaries via GitHub Actions CI/CD. ~1,370 LOC.",
    impact_metric: "Manual UI clicks to one CSV import",
    impact_detail: "Instead of building configuration manually through Halo's UI one field at a time, prepare a CSV and import. ~1000 fields in about 15 minutes on a single thread. Great workflow: gather client requirements, use AI to generate a CSV to spec, then import. Building forms becomes trivially fast as long as the fields fall within the tool's scope.",
    objective: "Build a CLI tool that reads custom field definitions from CSV files and creates them in Halo Software products via the API. Must support all 8 field types, handle authentication, respect rate limits, and distribute as cross-platform binaries.",
    approach: &[
        "Layered architecture: inbound (CSV parsing, interactive TUI), domain (models, validation, import results), outbound (OAuth auth client, field API client, HTTP type mapping). Same pattern used at larger scale in Zwipe",
        "bin/lib crate split: binary crate for orchestration, library crate for all logic. Enables testing domain logic independently of the CLI",
        "Type-safe domain modeling: Name (max 64, alphanumeric + underscore), Label (max 256), FieldType (8 variants with sub-type enums). All validated at construction, invalid data rejected before any API call",
        "OAuth 2.0 client credentials flow with Arc<Mutex<Option<AuthToken>>> for token caching. 30-second expiry buffer prevents edge-case 401s. Only fetches a new token when the cached one expires",
        "CSV parsing with header-position detection: finds columns by name rather than assuming fixed positions. Row-level error messages include row number and specific field issue for debugging",
        "Interactive debug TUI with colored terminal output: import mode (all fields) or debug mode (field-by-field with process/skip/quit). Import results tracked with timestamps per field",
        "Rate limiting (500ms between requests) to stay under Halo's 700/5min API limit",
        "GitHub Actions CI/CD: matrix build for Windows, macOS (Intel + ARM), and Linux. Cargo caching, distribution packaging with README and sample CSV, artifact uploads",
    ],
    snippets: &[
        Snippet {
            title: "Layered Architecture",
            code: r#"// bin/main.rs — orchestration only
// lib/ — all logic lives here
//
// inbound/
//   readers.rs     CSV parsing, header-position detection
//   screens.rs     Interactive TUI (import mode, debug mode)
//
// domain/
//   models/        CustomField, Name, Label, FieldType (8 variants)
//   import_result  Per-field success/failure tracking with timestamps
//   logging        Dual output (terminal + file), auto-cleanup
//
// outbound/
//   auth/client    OAuth 2.0 with token caching (Arc<Mutex<Option<AuthToken>>>)
//   auth/token     Expiry check with 30-second buffer
//   field_client   API calls with rate limiting
//   http_custom_field  Domain-to-API type mapping via From impl"#,
            description: "Same inbound/domain/outbound pattern used in Zwipe. Each layer has a clear responsibility. Domain types know nothing about CSV or HTTP. The bin crate just wires the layers together.",
        },
        Snippet {
            title: "Domain Validation",
            code: r#"// Newtypes with validation at construction
struct Name(String);  // max 64, alphanumeric + underscore only
struct Label(String); // max 256 characters

// 8 field types, some with sub-type enums
enum FieldType {
    Text { input_type: TextInputType },        // 7 input variants
    SingleSelect { input_type: SingleSelectInputType, selection_options: Vec<String> },
    Date { input_type: DateInputType },         // 2 input variants
    Memo, MultiSelect { selection_options: Vec<String> },
    Time, Checkbox, Rich,
}

// Domain → API mapping via From trait
impl From<&CustomField> for HttpCustomField {
    fn from(value: &CustomField) -> Self {
        // Validated domain type maps to Halo's expected JSON shape
        // type_id, input_type_id, selection_options all derived from FieldType
    }
}"#,
            description: "Invalid data is rejected at parse time with specific error messages (row number + field name). By the time a CustomField reaches the API client, it is guaranteed valid.",
        },
    ],
    obstacles: &[
        "Cross-platform binary distribution required a GitHub Actions CI/CD matrix: 4 targets (Windows, macOS Intel, macOS ARM, Linux), each with cargo caching, release builds, and distribution packaging",
        "OAuth token management: caching the token in Arc<Mutex<Option<AuthToken>>> with a 30-second expiry buffer. Without the buffer, tokens could expire between the check and the API call",
        "CSV header-position parsing instead of fixed column indices. Real CSVs from clients don't always have columns in the expected order. Row-level error messages made debugging bad input straightforward",
        "Selection options for SingleSelect/MultiSelect fields contain commas, which conflict with the Halo API's comma-separated format. Built selection_options_string() to strip commas from individual options before joining",
        "Log management: auto-cleanup of old log files (max 100 files, 7-day retention). Without this, repeated runs in production would accumulate unbounded log files",
    ],
    progress: "Shipped. Tagged v1.0.0 with cross-platform releases via GitHub Actions. Actively used in production for client implementations.",
    impact: "Reduced enterprise configuration time from hours to minutes. Deployed across Fortune 500 client implementations. ~1,370 lines of Rust demonstrating layered architecture, production-grade auth, and operational tooling (logging, results tracking, CI/CD).",
};

const MARVIN: Project = Project {
    name: "Marvin",
    slug: "marvin",
    headline: "CLI chatbot on Rig framework. Streaming, tool use, web search, context management.",
    category: "AI Tooling",
    repo_url: "https://github.com/scadoshi/marvin",
    summary: "Interactive CLI chatbot built on the Rig AI framework with Claude as the backend. Started as a learning project to understand AI agent plumbing in Rust. Streaming responses, 4 Tavily web tools with Arc-shared client, math tools, chat persistence with session IDs, token tracking, context compaction, and dynamic model discovery from Anthropic's API. Found and fixed deprecated model constants in Rig itself, submitting a PR across 17 files. ~1,750 LOC.",
    impact_metric: "~1,750 lines of Rust",
    impact_detail: "Built to learn the Rig AI framework by building something real, not just reading docs. Each feature taught something new about Rust async patterns, AI agent plumbing, or open source contribution.",
    objective: "Learn the Rig AI framework by building a real CLI chatbot. Each feature should teach something new about Rig or Rust, prioritizing learning over shipping.",
    approach: &[
        "Incremental feature development: start with basic chat loop, add streaming, tools, persistence, context management",
        "Command pattern architecture: each slash command is a trait impl routed via ChatInput enum",
        "4 Tavily web tools (search, extract, crawl, sitemap) sharing an Arc<TavilyClient> for efficient client reuse",
        "schemars for automatic JSON Schema generation from Rust types, eliminating manual schema maintenance",
        "Dynamic model discovery from Anthropic's /v1/models API instead of hardcoded constants",
        "Chat persistence with session IDs: /save writes to JSON, /import loads previous sessions",
    ],
    snippets: &[
        Snippet {
            title: "Tool Architecture",
            code: r#"// Each tool uses schemars for automatic JSON Schema generation
#[derive(JsonSchema, Deserialize)]
struct SearchArgs {
    query: String,
    topic: Option<String>,
    search_depth: Option<String>,
    max_results: Option<u32>,
}

// Tools share HTTP client via Arc
impl Tool for SearchWeb {
    async fn call(&self, args: SearchArgs) -> Result<String, ToolError> {
        self.client.search(args).await  // Arc<TavilyClient>
    }
}"#,
            description: "schemars derives JSON Schema from Rust types at compile time. No manual schema writing, no drift between types and definitions. Arc sharing keeps a single HTTP client across all 4 web tools.",
        },
        Snippet {
            title: "Architecture Evolution",
            code: r#"// Before: monolithic main.rs (~220 lines, all logic inline)
// After: command pattern with 11 user commands in separate modules
//
// main.rs         13 lines, bootstraps Runner::run()
// chat/mod.rs     Chat struct: history, input, agent, model
// chat/input.rs   ChatInput enum for command routing
// commands/       Individual modules per command
// runner.rs       Main loop with pattern matching on ChatInput
// anthropic/      Dynamic model discovery from /v1/models API
// agent_tools/    Math tools + 4 Tavily web tools"#,
            description: "Deliberate refactoring from monolith to clean module boundaries as complexity grew. Each command is independently testable.",
        },
    ],
    obstacles: &[
        "Discovered deprecated Anthropic model constants in Rig causing 404 errors. Filed issue #1370 (https://github.com/0xPlaygrounds/rig/issues/1370), fixed 17 files across the Rig repo, and submitted a PR following their contributing guidelines",
        "Stdout buffering: print!() without newline requires manual flush() for immediate display during streaming",
        "Tavily API rejects null values for optional fields. Fixed with #[serde(skip_serializing_if = \"Option::is_none\")]",
        "Architecture evolved from 220-line monolith to command pattern as features outgrew the original structure",
    ],
    progress: "Active. Core chatbot with streaming, tools, persistence, and context management all working. Roadmap includes RAG with local files, persistent memory, and MCP server integration.",
    impact: "Demonstrates ability to learn a new framework by building with it. Contributed back to the ecosystem when a bug was found. Shows progression from simple prototype to well-structured application.",
};

const NIGHTHAWK: Project = Project {
    name: "Nighthawk",
    slug: "nighthawk",
    headline: "LSM-tree key-value database from scratch. TCP server, concurrent connections, WAL, SSTables, bloom filters, k-way compaction.",
    category: "Database Internals",
    repo_url: "https://github.com/scadoshi/nighthawk",
    summary: "Complete LSM-tree key-value database built phase by phase from the Bitcask paper (https://riak.com/assets/bitcask-intro.pdf). Not just a storage engine: a TCP server you can connect to and use. Thread-per-connection concurrency with per-command locking. WAL durability with crash recovery, BTreeMap memtable, timestamped SSTables with bloom filter footers, k-way merge compaction, and a binary protocol with CRC32 checksums and byte-level corruption recovery. The architecture behind LevelDB, RocksDB, and Cassandra. ~2,100 LOC, 99 tests.",
    impact_metric: "~2,100 lines, 99 tests, 6 phases",
    impact_detail: "Started from a paper and built the storage layer that powers production databases. Not a contained exercise: a real TCP database server you can host, connect to, and query. Every layer built from scratch: binary protocol, WAL, memtable, SSTables, bloom filters, compaction, corruption recovery, concurrency.",
    objective: "Build a key-value database incrementally from the Bitcask paper toward the LSM-tree architecture that powers LevelDB, RocksDB, and Cassandra. Each phase adds a real layer: durability, sorted storage, probabilistic search, compaction, crash recovery, networking, and concurrency.",
    approach: &[
        "Phases 1-3 (Bitcask foundation): append-only WAL, sync_all() after every write, atomic rename compaction. 10-byte binary header per entry (magic 0x4E48, CRC32, wincode length prefix). Corruption recovery scans byte-by-byte past garbage to find the next valid entry, typed via CorruptionType enum (NotEnoughBytes, MagicBytesMismatch, ChecksumMismatch, ParseError)",
        "Memtable: BTreeMap<String, Entry> replaces HashMap offsets from the Bitcask phase. Sorted order is what makes SSTable flush cheap: just iterate and write. MemTable::process() tracks byte-level size for flush decisions. WAL replays into memtable on startup for crash recovery",
        "SSTable flush + read: 4MB threshold flushes sorted entries to data/sstables/{timestamp:020}.sst where lexicographic order is chronological. Read path checks memtable first (no I/O), then scans SSTables newest-to-oldest. Bloom filter checked before scanning any file",
        "K-way compaction: all SSTables processed simultaneously, not sequentially. Per-iteration finds the global minimum key across all active cursors. Newest file wins on duplicate keys. seen_keys: HashSet tracks winners; tombstone winners are silently dropped from output so they don't accumulate. Triggers every 10 flushes, intermediate memtable flushes keep memory bounded",
        "Bloom filters: one per SSTable stored as an in-file footer. Kirsch-Mitzenmacher double hashing with two xxh3 seeds, k=7, 10 bits/key for ~1% false positive rate. BloomFilterReader is a blanket impl on R: Read + Seek, so any file handle gains bloom filter reading automatically",
        "Entry consolidation: initial WalEntry/SstEntry split caused a tombstone resurrection bug. Single Entry enum threads tombstones through all layers (WAL, memtable, SSTables). compact() drops tombstone winners from output via Entry::Set guard. Regression test written before the refactor",
        "TCP server with concurrent connections: thread::spawn per connection, Arc<Mutex<Log>> shared across threads. Per-command locking keeps the critical section short: lock, execute, drop, flush. Clients make progress concurrently rather than waiting for entire connection lifetimes",
        "Generic Runner<R: BufRead, W: Write>: same read/parse/execute loop powers both the CLI (stdin/stdout) and the TCP server (TcpStream/BufWriter<TcpStream>). bin/lib crate split keeps orchestration in the binary, all logic in the library",
    ],
    snippets: &[
        Snippet {
            title: "Corruption Recovery",
            code: r#"// 10-byte header: [magic: 0x4E48 (2B)][crc32 (4B)][len (4B)]
// If magic or checksum fails, scan forward byte-by-byte
fn header_read_next(&mut self) -> anyhow::Result<Option<Entry>> {
    loop {
        match try_read_header(&mut self.reader) {
            Ok(entry) => return Ok(Some(entry)),
            Err(CorruptionType::NotEnoughBytes) => return Ok(None), // EOF
            Err(CorruptionType::MagicBytesMismatch) => {
                // Advance 1 byte past the bad position, retry
                self.reader.seek(SeekFrom::Current(-(HEADER_SIZE as i64 - 1)))?;
            }
            Err(CorruptionType::ChecksumMismatch) => {
                // CRC32 doesn't match — skip this entry
                self.reader.seek(SeekFrom::Current(-(HEADER_SIZE as i64 - 1)))?;
            }
            Err(CorruptionType::ParseError) => {
                // Magic+CRC valid but wincode parse failed — skip entry
            }
        }
    }
}"#,
            description: "A crash mid-write can leave partial data in the WAL. Instead of failing on startup, the reader scans past garbage to find the next valid entry. Four distinct corruption types so callers know exactly what went wrong.",
        },
        Snippet {
            title: "Bloom Filter",
            code: r#"// Kirsch-Mitzenmacher: two xxh3 seeds, k=7, ~1% false positive rate
fn positions(key: &[u8], bit_count: usize) -> impl Iterator<Item = usize> {
    let h1 = xxh3::hash64_with_seed(key, 0);
    let h2 = xxh3::hash64_with_seed(key, 1);
    (0..7).map(move |i| {
        (h1.wrapping_add((i as u64).wrapping_mul(h2)) % bit_count as u64) as usize
    })
}

impl BloomFilter {
    pub fn insert(&mut self, key: &[u8]) {
        for pos in positions(key, self.bit_count) {
            self[pos / 8] |= 1 << (pos % 8);
        }
    }
    // any bit unset → definitely absent; all set → probably present
    pub fn may_contain(&self, key: &[u8]) -> bool {
        positions(key, self.bit_count).all(|pos| self[pos / 8] & 1 << (pos % 8) != 0)
    }
}

// blanket impl: any R: Read + Seek gains read_bloom_filter() automatically
impl<R: Read + Seek> BloomFilterReader for R {
    fn read_bloom_filter(&mut self) -> anyhow::Result<Option<BloomFilter>> { ... }
}"#,
            description: "insert() and may_contain() are symmetric: same positions(), opposite bit operations. Two hash seeds replace k separate functions. The blanket impl makes the trait the extension point: no wrapper, just import it.",
        },
        Snippet {
            title: "K-Way Merge: compact()",
            code: r#"pub fn compact(&mut self) -> anyhow::Result<()> {
    let mut entries: Vec<_> = read_dir(&self.sstables_path)?.collect::<Result<_, _>>()?;
    entries.sort_by_key(|e| Reverse(e.file_name())); // newest-to-oldest
    let to_delete: Vec<_> = entries.iter().map(|e| e.path()).collect();

    let mut sstables: Vec<(Option<Entry>, SSTable)> = entries
        .into_iter()
        .filter_map(|e| SSTable::from_path(e.path()).ok().flatten())
        .map(|sst| (None, sst))
        .collect();
    for (entry, sstable) in sstables.iter_mut() {
        *entry = sstable.read_next_entry()?;
    }

    let mut memtable = MemTable::new();
    let mut seen_keys: HashSet<String> = HashSet::new();
    loop {
        sstables.retain(|(entry, _)| entry.is_some());
        if sstables.is_empty() { break; }

        let min = { /* global minimum key across all active cursors */ };

        for (entry, sstable) in sstables.iter_mut() {
            let entry_ref = entry.as_ref().unwrap();
            let is_participant = entry_ref.key() == min;
            let winner_found = seen_keys.contains(min.as_str());
            if is_participant && !winner_found {
                seen_keys.insert(min.clone());
                if let Entry::Set { .. } = entry_ref {
                    memtable.process(entry_ref.clone())?;
                }
                // Entry::Delete: mark seen but drop — tombstone served its purpose
            }
            if is_participant {
                *entry = sstable.read_next_entry()?;
            }
        }
        if memtable.should_flush() { memtable.flush_to(self.sstables_path.clone())?; }
    }
    if !memtable.is_empty() { memtable.flush_to(self.sstables_path.clone())?; }
    for path in to_delete { remove_file(path)?; }
    Ok(())
}"#,
            description: "Files newest-to-oldest means the first participant is the winner by definition. seen_keys tracks all winners so all copies advance. The Entry::Set guard is the tombstone suppression point: recorded as seen so older copies lose, never written to output.",
        },
    ],
    obstacles: &[
        "Tombstone resurrection: split Entry into WalEntry/SstEntry assuming SSTables only need Sets. Deleting a flushed key cleared the memtable only, but the SSTable still had the original Set and get() would find it again. Fix: single Entry enum, tombstones propagate through all layers, compact() suppresses them via Entry::Set guard. Regression test written before the refactor to verify the fix",
        "Compaction winner tracking evolved: first version used memtable.contains_key() which could not distinguish a tombstone winner from a Set winner. Replaced with seen_keys: HashSet so tombstone winners can be explicitly dropped from output instead of silently surviving",
        "flush_count must be initialized from the existing SSTable count on startup, not zero. A restart after writes would otherwise compact on the wrong schedule since it wouldn't know how many flushes happened before the restart",
        "Per-command vs per-connection locking: holding the Mutex for an entire connection lifetime would serialize all clients. Per-command locking (lock, execute, drop) keeps the critical section short so concurrent clients actually make progress",
    ],
    progress: "Complete. All 6 phases done: Bitcask foundation, durability, binary protocol, LSM-tree (memtable + SSTables + bloom filters + compaction), networking, and concurrency. 99 tests across 7 modules including TCP integration tests.",
    impact: "Started from a paper and built a complete, connectable key-value database implementing the storage architecture behind LevelDB, RocksDB, and Cassandra. Every layer built from scratch: binary protocol with corruption recovery, WAL durability, sorted memtable flush, bloom filter accelerated reads, k-way merge compaction, TCP server with concurrent access. Not a library, a database you can host and connect to.",
};

const UPSEE: Project = Project {
    name: "Upsee",
    slug: "upsee",
    headline: "Real-time pullup counter. Webcam + MoveNet pose estimation via tract ONNX runtime.",
    category: "ML Inference",
    repo_url: "https://github.com/scadoshi/upsee",
    summary: "Real-time pullup counter using webcam + MoveNet pose estimation model (https://huggingface.co/qualcomm/Movenet) via the tract ONNX runtime (https://github.com/sonos/tract). Runs entirely on-device with no cloud inference. Custom Square trait for center-cropping frames, confidence filtering to skip bad frames, and hysteresis state machine for accurate counting. ~145 LOC.",
    impact_metric: "~145 lines, on-device ML",
    impact_detail: "Full ML inference pipeline running locally in Rust. No Python, no cloud API, no latency. Frame capture to rep count in real time.",
    objective: "Build an end-to-end ML inference pipeline in Rust that counts pullups in real time using a webcam and the MoveNet pose estimation model (https://huggingface.co/qualcomm/Movenet). No cloud inference, everything runs on-device via the tract ONNX runtime (https://github.com/sonos/tract).",
    approach: &[
        "tract ONNX runtime as a Rust-native alternative to Python inference. Load model, optimize, run: three method calls to go from ONNX file to runnable inference",
        "Custom Square trait on ImageBuffer: center-crops webcam frames to square aspect ratio before resizing. Improved keypoint confidence significantly",
        "Tensor construction via Array4::from_shape_fn: reshapes 192x192 RGB image into [1, 3, 192, 192] NCHW tensor with 0-1 normalization in one pass",
        "Confidence threshold filtering: averages confidence scores across 4 keypoints (shoulders + wrists), skips frames below 0.4 to avoid acting on unreliable data",
        "Hysteresis-based state machine: UP threshold at 0.05 (arms high), DOWN threshold at 0.15 (arms extended). Dead zone between them absorbs sensor noise",
        "30 warmup frames before starting inference to let camera auto-exposure settle",
    ],
    snippets: &[
        Snippet {
            title: "Inference Pipeline",
            code: r#"// Load and optimize MoveNet ONNX model
let model = tract_onnx::onnx()
    .model_for_path(MODEL_PATH)?
    .into_optimized()?
    .into_runnable()?;

// Per frame: crop, resize, normalize, infer
let mut image = camera.frame()?.decode_image::<RgbFormat>()?;
let resized = resize(image.square().inner(), 192, 192, FilterType::Triangle);

// Build [1, 3, 192, 192] NCHW tensor, normalized 0-1
let tensor: Tensor = Array4::from_shape_fn(
    (1, 3, 192, 192),
    |(_, c, y, x)| resized[(x as _, y as _)][c] as f32 / 255.0
).into();

// Run inference — output: [1, 1, 17, 3] (17 keypoints × y,x,confidence)
let result = model.run(tvec!(tensor.into()))?;"#,
            description: "Three lines to load the model, then per-frame: crop to square, resize, normalize into a tensor, and run inference. tract handles the ONNX graph execution.",
        },
        Snippet {
            title: "Hysteresis State Machine",
            code: r#"// Two separate thresholds prevent oscillation:
const UP_THRESHOLD: f32 = 0.05;   // shoulders near wrist level
const DOWN_THRESHOLD: f32 = 0.15;  // shoulders dropped away
// Gap (0.05 to 0.15) = dead zone that absorbs noise

match state {
    Down => if diff < UP_THRESHOLD { state = Up; reps += 1; }
    Up   => if diff > DOWN_THRESHOLD { state = Down; }
}"#,
            description: "Without hysteresis, noise near the threshold causes rapid state flipping and false counts. The dead zone between thresholds means the signal must move decisively before a transition registers.",
        },
    ],
    obstacles: &[
        "Tensor shape [1, 3, 192, 192] in NCHW format was not intuitive. Required reading the ONNX model metadata and tract source to understand MoveNet's expected input layout",
        "Quantized MoveNet model (w8a16) is incompatible with tract: QuantizeLinear op is unsupported. Had to use the full-precision float model instead",
        "Single threshold caused false counts from keypoint jitter. Hysteresis with separate UP/DOWN thresholds and a dead zone solved it",
        "Webcam frames needed square cropping before resize to avoid distorting the aspect ratio, which degraded keypoint confidence. Built a custom Square trait on ImageBuffer for center-cropping",
        "tract documentation is sparse compared to Python ML libraries. Required reading source code, ONNX model metadata, and the tract examples to get the pipeline working",
    ],
    progress: "Working prototype. Counts pullups in real time from webcam feed. Roadmap: threshold tuning with more data, temporal smoothing, Raspberry Pi deployment, multi-threaded capture + inference.",
    impact: "Demonstrates ML inference in Rust without Python or cloud dependencies. Shows ability to work through an unfamiliar domain (ML, tensor operations, pose estimation) by reading specs, model metadata, and source code rather than relying on tutorials. ~145 lines from webcam to rep counter.",
};

const CAPTURE: Project = Project {
    name: "Capture",
    slug: "capture",
    headline: "Cross-platform security camera. Input device grabbing, intruder photos, platform-specific I/O.",
    category: "Systems Programming",
    repo_url: "https://github.com/scadoshi/capture",
    summary: "Cross-platform security camera that grabs all input devices, snaps intruder photos on any interaction, and only releases with a secret key. Linux uses raw evdev with nix::poll, macOS uses rdev with Accessibility API callbacks. Custom traits on third-party types for device identification and secret key detection. ~225 LOC.",
    impact_metric: "~225 lines, 2 platforms",
    impact_detail: "Same goal on two platforms, completely different implementations. Linux uses raw evdev with nix::poll, macOS uses rdev with Accessibility API callbacks. Conditional compilation keeps both behind a clean interface.",
    objective: "Build a cross-platform security camera that locks input devices, takes a photo of anyone who touches the keyboard or mouse, and only unlocks with a secret key. Must work on both macOS and Linux despite fundamentally different I/O models.",
    approach: &[
        "Conditional compilation: cfg(target_os) switches between platform modules. Platform-specific deps in Cargo.toml via [target.'cfg(...)'.dependencies]",
        "Linux: enumerate /dev/input/event* devices, filter via capability heuristics (Identify trait on evdev::Device), grab each, poll with nix::poll, ungrab on exit",
        "macOS: rdev::grab with Accessibility API. Callback returns None to swallow events. No clean stop API, forced to process::exit(0) on secret key",
        "Custom traits on third-party types: Identify trait on Device (is_probably_keyboard, is_probably_mouse), IsSecret trait on InputEvent for secret key matching",
        "CaptureState with jiff timestamps for 1-second debounce between photos. Rc<Mutex<CaptureState>> for interior mutability in event callbacks",
        "Camera warmup: 30 frames discarded on init to let auto-exposure settle before taking real photos",
    ],
    snippets: &[
        Snippet {
            title: "Platform Divergence",
            code: r#"// Same goal, completely different implementations:
//
// | Concern        | macOS                     | Linux                      |
// |----------------|---------------------------|----------------------------|
// | Grab mechanism | rdev::grab (Accessibility)| evdev device.grab() each   |
// | Event loop     | Callback-based            | nix::poll across FDs       |
// | Permissions    | Accessibility API approval| input group membership     |
// | Shutdown       | process::exit(0)          | device.ungrab() on all     |
//
// Capability-based heuristics for device identification:
// is_probably_keyboard() = EV_REPEAT + KEY_A + KEY_ENTER + KEY_SPACE
// is_probably_mouse()    = REL_X + REL_Y relative axes"#,
            description: "The same feature requires fundamentally different system APIs on each platform. Conditional compilation keeps both behind a shared interface.",
        },
        Snippet {
            title: "Trait Extensions on Third-Party Types",
            code: r#"// Identify trait on evdev::Device — capability-based heuristics
impl Identify for Device {
    fn is_probably_keyboard(&self) -> bool {
        self.supported_events().contains(EventType::REPEAT)
            && self.supported_keys().is_some_and(|keys| {
                keys.contains(KEY_A) && keys.contains(KEY_ENTER)
            })
    }
    fn is_probably_mouse(&self) -> bool {
        self.supported_relative_axes().is_some_and(|axes| {
            axes.contains(REL_X) && axes.contains(REL_Y)
        })
    }
}

// IsSecret trait on evdev::InputEvent — secret key detection
impl IsSecret for InputEvent {
    fn is_secret(&self) -> bool {
        matches!(self.destructure(),
            EventSummary::Key(_, KeyCode::KEY_ESC, 1))
    }
}"#,
            description: "Custom traits on third-party types. Linux doesn't label devices as 'keyboard' or 'mouse', so you detect them by what they can do. Same pattern for secret key detection: extend the event type rather than match inline.",
        },
    ],
    obstacles: &[
        "rdev grabs ALL evdev devices on Linux, including Bluetooth controllers and network adapters, causing disconnects. Discovered this at runtime. Dropped to raw evdev with selective grabbing based on device capabilities",
        "macOS rdev::grab has no clean stop API. The grab loop blocks forever with no break mechanism. Forced to use process::exit(0), which means no cleanup or graceful shutdown on macOS",
        "Linux poll loop requires rebuilding PollFd vec each iteration because PollFd borrows the device file descriptor, causing borrow conflicts if held across the loop body",
        "Device identification is heuristic-based. A device that reports EV_REPEAT + alpha keys is 'probably a keyboard'. No guaranteed way to distinguish real keyboards from virtual or composite devices",
    ],
    progress: "Working on both macOS and Linux. Grabs input, takes timestamped photos, unlocks with secret key. Clean ungrab on Linux, forced exit on macOS.",
    impact: "Demonstrates systems-level programming across platforms. Shows ability to drop down to raw OS interfaces (evdev, nix::poll) when higher-level libraries don't fit the use case. Custom traits on third-party types for clean abstraction of platform-specific behavior. ~225 LOC.",
};
