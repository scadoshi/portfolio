pub struct Project {
    pub name: &'static str,
    pub slug: &'static str,
    pub headline: &'static str,
    pub category: &'static str,
    pub repo_url: &'static str,
    pub summary: &'static str,
    pub card_bullets: &'static [&'static str],
    pub impact_metric: &'static str,
    pub objective: &'static str,
    pub tags: &'static [&'static str],
    pub media: &'static [MediaItem],
    pub approach: &'static [&'static str],
    pub snippets: &'static [Snippet],
    pub obstacles: &'static [&'static str],
    pub progress: &'static str,
    pub impact: &'static str,
    pub status: ProjectStatus,
}

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum MediaKind {
    Image,
    Video,
}

#[derive(Clone, PartialEq)]
pub struct MediaItem {
    pub src: Asset,
    pub alt: &'static str,
    pub caption: Option<&'static str>,
    pub kind: MediaKind,
}

#[derive(Clone, Copy)]
pub enum ProjectStatus {
    Done,
    Doing,
}

impl ProjectStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Done => "Done",
            Self::Doing => "Doing",
        }
    }
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Done => "status-done",
            Self::Doing => "status-doing",
        }
    }
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
    &[NIGHTHAWK, DIPROTODON, MARVIN, CAPTURE, UPSEE]
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
    headline: "Full-stack MTG deck builder. Axum backend, Dioxus frontend, PostgreSQL, 110k+ cards.",
    category: "Full-Stack Application",
    repo_url: "https://github.com/scadoshi/zwipe",
    summary: "Mobile-first Magic: The Gathering deck builder with swipe-based navigation.",
    card_bullets: &[
        "Native iOS + Android from one Dioxus codebase",
        "Axum REST API backed by PostgreSQL (110k+ cards, materialized search)",
        "Background sync service + static marketing site, all in the same workspace",
        "Full commander support \u{2014} partners, backgrounds, oathbreaker",
    ],
    impact_metric: "Full-stack mobile app \u{2014} ~37,300 lines of Rust",
    objective: "Build a full-stack MTG deck builder with swipe-based navigation as a single-language Rust project. Five workspace crates: zwipe-core (shared domain), zerver (Axum API), zwiper (Dioxus mobile app), zervice (background sync), zite (static marketing site). Full commander support \u{2014} partners, backgrounds, oathbreaker. See the [architecture](https://zwipe.net/about) and [demo](https://zwipe.net). App Store submission pending.",
    tags: &["rust", "full-stack", "ios", "dioxus", "postgresql"],
    media: &[
        MediaItem {
            src: asset!("/assets/projects/zwipe/login.mp4"),
            alt: "Signing in to a Zwipe account",
            caption: Some("Signing in to a Zwipe account"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/zwipe/user_profile.mp4"),
            alt: "User profile screen",
            caption: Some("User profile screen"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/zwipe/deck_profile.mp4"),
            alt: "Editing the deck profile",
            caption: Some("Editing the deck profile"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/zwipe/add_deck_cards.mp4"),
            alt: "Swiping cards to add to a deck",
            caption: Some("Swiping cards to add to a deck"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/zwipe/deck_cards.mp4"),
            alt: "Browsing the deck card list",
            caption: Some("Browsing the deck card list"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/zwipe/import.mp4"),
            alt: "Creating a deck and importing a decklist",
            caption: Some("Creating a deck and importing a decklist"),
            kind: MediaKind::Video,
        },
    ],
    approach: &[
        "Rust on mobile via Dioxus \u{2014} one Rust codebase compiles to a native mobile app, no JS bridge, no separate frontend repo",
        "Shared domain crate (zwipe-core) used by both the Axum API and the Dioxus app: one CardFilter type drives SQL queries server-side and in-memory filtering on the device, via extension traits on Vec<Card>",
        "Swipe stack built from scratch \u{2014} OnSwipe trait with OnTouch/OnMouse adapters, dual-threshold detection (velocity OR distance), axis locking. Ten files, zero gesture libraries",
        "Production-grade auth: Argon2id with NIST-compliant 170+ pattern blocklist, rotating refresh tokens (replay-safe via delete-on-use), Password type consumed on hash so plaintext can't leak",
        "SQLx at scale \u{2014} five-strategy upsert chain handles batching, PartialEq delta detection, and per-row fallback; 88-column Scryfall sync respects PostgreSQL's 65k parameter limit (~327 cards per batch)",
        "Background service (zervice) for nightly Scryfall delta sync of 110k+ printings, materialized view refresh for deduplicated search (~35k unique cards), refresh-token cleanup, session enforcement",
        "Production posture: .unwrap, .expect, panic!, todo!, dbg!, print! all denied at compile time. 33 enforced Clippy rules, 340+ tests, security audit complete, nightly Cloudflare R2 backups",
    ],
    snippets: &[
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
    progress: "Feature-complete. Full deck management, swipe-based deck building, commander system, maybeboard/sideboard, import/export, email verification, 15 themes. Security audit complete; nightly backups. Live at [zwipe.net](https://zwipe.net); App Store submission pending.",
    impact: "Full-stack mobile delivery in pure Rust \u{2014} shared domain types across the Axum API, the Dioxus app, and a background sync service. ~37,300 lines across five crates, 340+ tests, zero unwrap.",
    status: ProjectStatus::Doing,
};

const HALO_ACTION_IMPORTER: Project = Project {
    name: "Halo Action Importer",
    slug: "halo-action-importer",
    headline: "Production bulk import tool. Millions of records, resilient retry, incremental caching.",
    category: "Production Data Tooling",
    repo_url: "https://github.com/scadoshi/halo_action_importer",
    summary: "CLI for bulk importing actions into the Halo Software suite from CSV and Excel.",
    card_bullets: &[
        "Layered architecture (inbound/domain/outbound) with bin/lib crate split",
        "Built for hours-long unattended runs against unreliable production APIs",
        "Handles millions of records with automatic recovery from transient failures",
        "~3,230 LOC across 20 files",
    ],
    impact_metric: "Weeks of manual work, automated",
    objective: "Bulk-import millions of records into Halo Software from CSV and Excel against a production API. Must survive every real failure mode: network errors, token expiry, missing tickets, partial batch failures, inconsistent data formats across client exports.",
    tags: &["rust", "csv", "etl", "api-resilience"],
    media: &[],
    approach: &[
        "Per-failure-mode recovery, not blanket retry-with-backoff. 401 \u{2192} refresh token; 504/network \u{2192} retry immediately; missing ticket \u{2192} permanent skip via run-wide HashSet<u32>; deserialization error \u{2192} skip row and continue",
        "Ticket-grouped retry: when a batch fails, group actions by ticket_id and retry each group independently. Maximizes successful imports and identifies exactly which tickets don't exist",
        "Cache evolution as the dataset grew: single Halo report endpoint \u{2192} split across resources \u{2192} fully local cache from a direct DB query of ~8M existing action IDs. Each stage rethought how the tool remembers its own work",
        "Two-tier cache with fs2 file locking: JSON tracks existing IDs per resource (per-run cache), text file tracks IDs imported during the current run (append-only). Both survive restarts and concurrent writes",
        "Structured per-run output: log/YYYY-MM-DD_HH-MM-SS/ with full.log, retry.csv (re-importable), and summary.json with performance metrics, error breakdown by type, affected ticket IDs",
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
        "Deduplication at scale: single Halo report worked at ~100k IDs but timed out as the dataset grew. Splitting across resources bought time; still timed out at millions. Final answer: direct DB query for all ~8M existing action IDs, local cache as source of truth (safe because I was the only importer)",
        "Binary search retry was the wrong abstraction for batch failures \u{2014} O(log(batch) * failures) too many API calls. Ticket-grouped retry is simpler and more efficient",
        "Parallel instances against the same cache directory hit corruption bugs. Fixed with fs2 exclusive locks on both cache files",
    ],
    progress: "Production. Actively used for real data migrations.",
    impact: "Reduced migration timelines from weeks to days. Runs unattended for hours against millions of records with automatic recovery from any transient failure.",
    status: ProjectStatus::Done,
};

const HALO_CUSTOM_FIELD_BUILDER: Project = Project {
    name: "Halo Custom Field Builder",
    slug: "halo-custom-field-builder",
    headline: "Shipped CLI tool. Bulk-creating custom fields across Halo Software products with cross-platform binaries.",
    category: "Production Data Tooling",
    repo_url: "https://github.com/scadoshi/halo_custom_field_builder",
    summary: "CLI that bulk-creates custom fields across Halo Software products from CSV definitions.",
    card_bullets: &[
        "Layered architecture (inbound/domain/outbound) with bin/lib crate split",
        "Type-safe domain modeling; OAuth 2.0 with token caching",
        "Interactive debug TUI; import results tracking; log management",
        "Cross-platform binaries via GitHub Actions. ~1,370 LOC",
    ],
    impact_metric: "Manual UI clicks to one CSV import",
    objective: "Read custom field definitions from CSV and create them across Halo Software products via the API. Must support all 8 field types, handle auth, respect rate limits, ship as cross-platform binaries.",
    tags: &["rust", "cli", "api", "cross-platform"],
    media: &[
        MediaItem {
            src: asset!("/assets/projects/halo-custom-field-builder/full_process.mp4"),
            alt: "Full process: debug mode, regular mode, CSV used, and logs rendered",
            caption: Some("Full process: debug mode, regular mode, CSV used, and logs rendered"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/halo-custom-field-builder/build_field_by_hand.mp4"),
            alt: "Manually creating a custom field",
            caption: Some("Manually creating a custom field"),
            kind: MediaKind::Video,
        },
    ],
    approach: &[
        "Type-safe domain modeling: Name (max 64, alphanumeric + underscore), Label (max 256), FieldType (8 variants with sub-type enums). All validated at construction \u{2014} invalid data rejected before any API call",
        "OAuth 2.0 client credentials flow with Arc<Mutex<Option<AuthToken>>> caching. 30-second expiry buffer prevents the edge-case 401 between check and call",
        "Layered architecture (inbound/domain/outbound) with bin/lib crate split. Same pattern Zwipe uses at larger scale; lets the library logic be tested independently of the CLI",
        "Interactive debug TUI with colored output: import mode runs everything, debug mode walks field-by-field with process/skip/quit. Per-field import results tracked with timestamps",
        "GitHub Actions matrix build: Windows, macOS Intel + ARM, Linux. Cargo caching, distribution packaging with README + sample CSV",
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
        "Selection options for SingleSelect/MultiSelect fields contain commas, which collide with Halo's comma-separated API format. Built selection_options_string() to strip commas from individual options before joining",
        "CSV header-position parsing instead of fixed column indices: real client CSVs don't always have columns in the expected order. Row-level error messages include row number + specific field issue",
        "Log auto-cleanup (max 100 files, 7-day retention). Without it, repeated production runs accumulate unbounded log files",
    ],
    progress: "Shipped. Tagged v1.0.0 with cross-platform releases via GitHub Actions. Actively used in production for client implementations.",
    impact: "Reduced enterprise configuration time from hours to minutes. Deployed across Fortune 500 client implementations.",
    status: ProjectStatus::Done,
};

const MARVIN: Project = Project {
    name: "Marvin",
    slug: "marvin",
    headline: "CLI chatbot on Rig framework. Streaming, tool use, web search, context management.",
    category: "AI Tooling",
    repo_url: "https://github.com/scadoshi/marvin",
    summary: "Interactive CLI chatbot built on Rig with Claude. A Rust agent-plumbing learning project.",
    card_bullets: &[
        "Streaming responses; 4 Tavily web tools with Arc-shared client; math tools",
        "Chat persistence with session IDs; token tracking; context compaction",
        "Dynamic model discovery from Anthropic's API",
        "Found + fixed deprecated model constants in Rig (PR across 17 files). ~1,750 LOC",
    ],
    impact_metric: "~1,750 lines of Rust",
    objective: "Learn the Rig AI framework by building a real CLI chatbot. Each feature should teach something new about Rig or Rust \u{2014} prioritizing learning over shipping.",
    tags: &["rust", "ai", "cli", "llm"],
    media: &[],
    approach: &[
        "Found and fixed deprecated Anthropic model constants in Rig causing 404 errors. Filed issue #1370 (https://github.com/0xPlaygrounds/rig/issues/1370), submitted a PR across 17 files. The learning project found a real bug in its own framework",
        "Command pattern architecture: each slash command is a trait impl routed via a ChatInput enum. Started as a 220-line monolith, refactored to clean module boundaries as complexity grew",
        "4 Tavily web tools (search, extract, crawl, sitemap) sharing an Arc<TavilyClient> for efficient client reuse",
        "schemars derives JSON Schema from Rust types at compile time \u{2014} no manual schema maintenance, no drift between types and definitions",
        "Dynamic model discovery from Anthropic's /v1/models API instead of hardcoded constants that go stale",
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
        "Stdout buffering: print!() without a newline requires manual flush() for immediate display during streaming",
        "Tavily API rejects null values for optional fields. Fixed with #[serde(skip_serializing_if = \"Option::is_none\")]",
        "Architecture outgrew the 220-line monolith. Refactored to command pattern with per-command modules \u{2014} each command independently testable",
    ],
    progress: "Active. Streaming, tools, persistence, and context management all working. Roadmap: RAG with local files, persistent memory, MCP server integration.",
    impact: "Learning project that contributed back to its own framework. Demonstrates the loop: pick a real goal, hit real friction, fix it upstream, ship something usable.",
    status: ProjectStatus::Done,
};

const NIGHTHAWK: Project = Project {
    name: "Nighthawk",
    slug: "nighthawk",
    headline: "LSM-tree key-value database from scratch. TCP server, concurrent connections, WAL, SSTables, bloom filters, k-way compaction.",
    category: "Database Internals",
    repo_url: "https://github.com/scadoshi/nighthawk",
    summary: "LSM-tree key-value database built phase by phase from the Bitcask paper. The architecture behind LevelDB, RocksDB, and Cassandra.",
    card_bullets: &[
        "TCP server with thread-per-connection concurrency, per-command locking",
        "WAL durability, BTreeMap memtable, bloom-filtered SSTables",
        "K-way merge compaction; byte-level corruption recovery",
        "~2,100 LOC, 99 tests",
    ],
    impact_metric: "~2,100 lines, 99 tests, 6 phases",
    objective: "Build a key-value database incrementally from the Bitcask paper (https://riak.com/assets/bitcask-intro.pdf) toward the LSM-tree architecture that powers LevelDB, RocksDB, and Cassandra. Each phase adds a real layer: durability, sorted storage, probabilistic search, compaction, crash recovery, networking, concurrency.",
    tags: &["rust", "kv-store", "lsm-tree", "networking"],
    media: &[],
    approach: &[
        "Built phase by phase from the Bitcask paper (https://riak.com/assets/bitcask-intro.pdf) to full LSM-tree \u{2014} WAL, memtable, SSTables, bloom filters, k-way compaction, TCP server, concurrency. Six distinct architectural layers, each one a real piece of how production KV systems work",
        "WAL with sync_all() after every write; 10-byte binary header (magic 0x4E48 + CRC32 + length); corruption recovery scans byte-by-byte past garbage, typed via a CorruptionType enum so callers know exactly what went wrong",
        "BTreeMap memtable makes SSTable flush trivial (iterate + write). WAL replays into memtable on startup for crash recovery. 4MB threshold keeps memory bounded",
        "Bloom filters as in-file SSTable footers. Kirsch-Mitzenmacher double hashing with two xxh3 seeds, k=7, ~1% false positive rate. BloomFilterReader as a blanket impl on R: Read + Seek \u{2014} any file handle gains it",
        "K-way compaction across all SSTables simultaneously, not sequentially. seen_keys HashSet drops tombstone winners so they never accumulate. Single Entry enum threads tombstones through every layer (WAL, memtable, SSTables)",
        "TCP server: thread-per-connection, Arc<Mutex<Log>>, per-command locking (lock \u{2192} execute \u{2192} drop \u{2192} flush). Generic Runner<R: BufRead, W: Write> powers both the CLI and the TCP server from the same loop",
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
        "Tombstone resurrection: splitting Entry into WalEntry/SstEntry assumed SSTables only needed Sets. Deleting a flushed key cleared the memtable but the SSTable still had the original Set, and get() would find it again. Fix: single Entry enum threading tombstones through all layers; compact() suppresses them via an Entry::Set guard. Regression test written before the refactor",
        "flush_count must be initialized from the existing SSTable count on startup, not zero. A restart after writes would otherwise compact on the wrong schedule",
        "Per-command vs per-connection locking: holding the Mutex for an entire connection lifetime would serialize all clients. Per-command locking (lock \u{2192} execute \u{2192} drop) keeps the critical section short so clients actually make progress",
    ],
    progress: "Complete. All 6 phases done. 99 tests across 7 modules including TCP integration tests.",
    impact: "Started from a paper and built a complete, connectable key-value database \u{2014} the storage architecture behind LevelDB, RocksDB, and Cassandra. Every layer built from scratch: binary protocol with corruption recovery, WAL durability, sorted memtable flush, bloom-filter-accelerated reads, k-way merge compaction, TCP server with concurrent access.",
    status: ProjectStatus::Done,
};

const DIPROTODON: Project = Project {
    name: "Diprotodon",
    slug: "diprotodon",
    headline: "Redis-compatible in-memory KV server in Rust. Hand-written RESP wire protocol, real redis-cli clients connect.",
    category: "Network Protocols & Systems",
    repo_url: "https://github.com/scadoshi/diprotodon",
    summary: "Redis-compatible in-memory KV server in Rust. Real redis-cli clients connect.",
    card_bullets: &[
        "Hand-written RESP wire protocol \u{2014} no library does the work",
        "Parser-as-framer returning Incomplete / Malformed / Ok((frame, leftover))",
        "Binary-safe end-to-end (Vec<u8>, not String)",
        "Hexagonal layout; generic Session<R, W> for cursor-based tests",
        "~1,187 LOC, 50+ tests across every protocol layer",
    ],
    impact_metric: "~1,187 lines, 50+ tests, M0\u{2013}M2 complete",
    objective: "Build a Redis-compatible KV server one wire-protocol layer at a time, hand-writing the substance so the muscle survives the project. Each milestone adds a real layer: TCP echo, RESP framing, command dispatch, in-memory KV ops, then TTL, AOF, and Pub/Sub. Sibling-paired with a Go port (wombat) to feel the translation between languages.",
    tags: &["rust", "redis", "tcp", "protocol"],
    media: &[],
    approach: &[
        "Hand-written RESP wire protocol \u{2014} no library does the work. Real redis-cli clients connect and run all four M2 commands (PING, GET/SET/DEL/EXISTS) against it",
        "Parser-as-framer: Frame::parse_one(&[u8]) -> Result<(Frame, &[u8]), FrameError>. Returns the parsed frame plus a leftover slice borrowing from the input \u{2014} no allocation for the rest-of-buffer. Incomplete is a load-bearing error variant, not an Option",
        "Binary safe end-to-end: Vec<u8>, not String. Bulk-string payloads can be arbitrary bytes (jpegs, interior CRLF, whatever). UTF-8 is never enforced where the protocol doesn't require it",
        "Iterative array parsing. Recursive parse_array would blow the stack on MGET key1..key1000; a Vec + loop is one extra concept and zero risk",
        "SimpleInner newtype with three trust levels: strict TryFrom for untrusted bytes, infallible ok()/pong() for known-safe literals, sanitized() that strips CR/LF for arbitrary server-authored error strings. Inbound strict, outbound lossy",
        "Generic Session<R: Read, W: Write>. Cursor<Vec<u8>> as R lets tests script RESP bytes in and out without a real socket",
        "Hexagonal layout (domain / inbound / outbound). Domain knows nothing about RESP, RESP knows nothing about Redis semantics. One-way coupling: inbound and outbound depend on domain, never the reverse",
    ],
    snippets: &[
        Snippet {
            title: "Parser-as-Framer",
            code: r#"pub fn parse_one(bytes: &[u8]) -> Result<(Frame, &[u8]), FrameError> {
    let (header, rest) = bytes.split_crlf().ok_or(FrameError::Incomplete)?;
    let (sigil, len_bytes) = header.split_first().ok_or(FrameError::Malformed)?;
    let len: usize = std::str::from_utf8(len_bytes)?.parse()?;
    match sigil {
        b'$' => Self::parse_bulk_string(rest, len),
        b'*' => Self::parse_array(rest, len),
        _ => Err(FrameError::UnknownSigil),
    }
}"#,
            description: "Frame::parse_one returns the parsed frame and a leftover slice borrowing from the input. Incomplete is a real error variant, not an Option \u{2014} it's load-bearing for the session reader's read-more loop. split_crlf returns None when no CRLF is found, which is the Incomplete signal at the byte-splitter layer.",
        },
        Snippet {
            title: "Drain on Success, Clear on Garbage, Preserve on Incomplete",
            code: r#"pub fn parse_frame(&mut self) -> Result<Frame, FrameError> {
    match Frame::parse_one(&self.buf) {
        Ok((frame, bytes)) => {
            let consumed = self.buf.len() - bytes.len();
            self.buf.drain(..consumed);  // keep only the leftover
            Ok(frame)
        }
        Err(e) => {
            if !matches!(e, FrameError::Incomplete) {
                self.buf.clear();  // poisoned wire, throw it away
            }
            Err(e)  // Incomplete preserves the buf for next read
        }
    }
}"#,
            description: "The borrow checker forbids holding the leftover slice while mutating self.buf \u{2014} so capture bytes.len() (a usize, Copy), drop the borrow, then drain. Three different policies in three arms: success drains the consumed prefix, hard error clears, Incomplete preserves so the next read appends to a valid in-progress frame.",
        },
        Snippet {
            title: "SimpleInner: Strict In, Lossy Out",
            code: r#"impl TryFrom<&[u8]> for SimpleInner {
    type Error = SimpleInnerError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.contains(&b'\r') { return Err(SimpleInnerError::IncludesCarriageReturn); }
        if value.contains(&b'\n') { return Err(SimpleInnerError::IncludesLineFeed); }
        Ok(Self(value.to_vec()))
    }
}

impl SimpleInner {
    pub fn ok() -> Self { Self(b"OK".to_vec()) }
    pub fn pong() -> Self { Self(b"PONG".to_vec()) }
    pub fn sanitized(bytes: impl Into<Vec<u8>>) -> Self {
        Self(bytes.into().into_iter()
            .filter(|b| *b != b'\r' && *b != b'\n')
            .collect())
    }
}"#,
            description: "Same newtype, three constructors with different trust levels. Inbound (or untrusted) bytes get TryFrom's strict validation \u{2014} rejected if CR/LF present. Known-safe literals (\"OK\", \"PONG\") get infallible constructors. Arbitrary server-authored error strings get sanitized \u{2014} CR/LF stripped because the wire format forbids them anyway. The invariant survives all three paths.",
        },
    ],
    obstacles: &[
        "Bytes-to-Vec read footgun: first version called inner.read(&mut new) where new was Vec::new() \u{2014} reads into a zero-length slice return Ok(0) forever, buf never grew, the get_frame loop spun. Fix: read into a sized stack array ([0u8; 1024]) and extend_from_slice(&new[..n]) using the returned count. Clippy's unused_io_amount catches it now",
        "Borrow-checker corner on parse_frame: parse_one returns (Frame, &[u8]) borrowing from self.buf. Trying to self.buf.drain(..) while the slice was alive failed. Fix: extract bytes.len() (a Copy usize) before mutating",
        "split_crlf contract: when no CRLF is in the buffer, should it return None or Some((entire_buf, &[]))? None is correct \u{2014} it preserves the Incomplete signal up to the parser. The byte-splitter doesn't have errors; the parser does. That boundary matters",
    ],
    progress: "M0\u{2013}M2 complete: TCP echo, RESP protocol + dispatch, GET/SET/DEL/EXISTS. Test coverage on every protocol layer. Roadmap: M3 EXPIRE/TTL, M4 AOF persistence (deliberately not LSM \u{2014} nighthawk already proves that ground), M5 Pub/Sub.",
    impact: "Real network protocol implemented from the byte level \u{2014} framing, error variants, streaming, binary safety, layered architecture \u{2014} interoperable with a real client, not a mock. Paired with nighthawk to cover both halves of how production KV systems are built: nighthawk the on-disk LSM storage engine, diprotodon the in-memory protocol server. Both hand-written.",
    status: ProjectStatus::Doing,
};

const UPSEE: Project = Project {
    name: "Upsee",
    slug: "upsee",
    headline: "Real-time pullup counter. Webcam + MoveNet pose estimation via tract ONNX runtime.",
    category: "ML Inference",
    repo_url: "https://github.com/scadoshi/upsee",
    summary: "Real-time pullup counter using webcam + MoveNet pose estimation. Runs entirely on-device.",
    card_bullets: &[
        "tract ONNX runtime for inference; no cloud dependency",
        "Custom Square trait for center-cropping frames",
        "Confidence filtering + hysteresis state machine for accurate counts",
        "~145 LOC",
    ],
    impact_metric: "~145 lines, on-device ML",
    objective: "Build an end-to-end ML inference pipeline in Rust that counts pullups in real time from a webcam, using the MoveNet pose estimation model (https://huggingface.co/qualcomm/Movenet). No cloud inference \u{2014} everything runs on-device via the tract ONNX runtime (https://github.com/sonos/tract).",
    tags: &["rust", "ml", "computer-vision", "real-time"],
    media: &[
        MediaItem {
            src: asset!("/assets/projects/upsee/upsee-demo.mp4"),
            alt: "Upsee counting pullups in real time from webcam input",
            caption: Some("Real-time pose estimation and rep counting via on-device MoveNet inference"),
            kind: MediaKind::Video,
        },
    ],
    approach: &[
        "tract ONNX runtime as the Rust-native inference path. Three method calls from ONNX file to runnable inference: model_for_path \u{2192} into_optimized \u{2192} into_runnable. No Python, no cloud",
        "Custom Square trait on ImageBuffer center-crops webcam frames to square aspect before resizing. Distortion-free resize meaningfully improved keypoint confidence",
        "Tensor build via Array4::from_shape_fn: reshapes a 192x192 RGB image into a [1, 3, 192, 192] NCHW tensor with 0-1 normalization in one pass",
        "Confidence threshold filtering: averages scores across 4 keypoints (shoulders + wrists), skips frames below 0.4 so the counter never acts on unreliable data",
        "Hysteresis state machine: UP at 0.05 (arms high), DOWN at 0.15 (arms extended). Dead zone between thresholds absorbs sensor noise so jitter doesn't false-count",
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
        "Single threshold caused false counts from keypoint jitter. Hysteresis with separate UP/DOWN thresholds and a dead zone solved it",
        "Quantized MoveNet model (w8a16) is incompatible with tract \u{2014} QuantizeLinear op is unsupported. Used the full-precision float model instead",
        "tract documentation is sparse compared to Python ML libraries. Required reading source, ONNX model metadata, and tract examples to get the pipeline working",
    ],
    progress: "Working prototype. Counts pullups in real time from webcam. Roadmap: threshold tuning, temporal smoothing, Raspberry Pi deployment, multi-threaded capture + inference.",
    impact: "ML inference in Rust without Python or cloud dependencies. ~145 lines from webcam frame to rep count.",
    status: ProjectStatus::Done,
};

const CAPTURE: Project = Project {
    name: "Capture",
    slug: "capture",
    headline: "Cross-platform security camera. Input device grabbing, intruder photos, platform-specific I/O.",
    category: "Systems Programming",
    repo_url: "https://github.com/scadoshi/capture",
    summary: "Cross-platform security camera. Grabs all input devices, snaps intruder photos, only releases with a secret key.",
    card_bullets: &[
        "Linux: raw evdev with nix::poll for selective device grabbing",
        "macOS: rdev with Accessibility API callbacks",
        "Custom traits on third-party types for device ID + secret key detection",
        "~225 LOC",
    ],
    impact_metric: "~225 lines, 2 platforms",
    objective: "Cross-platform security camera that grabs input devices, snaps a photo of anyone who touches keyboard or mouse, and only unlocks with a secret key. Same goal, two fundamentally different OS I/O models.",
    tags: &["rust", "cross-platform", "security", "camera"],
    media: &[],
    approach: &[
        "Conditional compilation: cfg(target_os) switches between platform modules. Platform-specific deps via [target.'cfg(...)'.dependencies] in Cargo.toml",
        "Linux: enumerate /dev/input/event* devices, filter by capability heuristics (Identify trait on evdev::Device), grab each, poll with nix::poll, ungrab on exit",
        "macOS: rdev::grab with Accessibility API callbacks. Callback returns None to swallow events. No clean stop API \u{2014} process::exit(0) on secret key",
        "Custom traits on third-party types: Identify on evdev::Device (is_probably_keyboard, is_probably_mouse), IsSecret on InputEvent. Extension beats wrapping",
        "CaptureState with jiff timestamps for 1-second photo debounce. Rc<Mutex<CaptureState>> for interior mutability inside event callbacks",
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
        "rdev grabs ALL evdev devices on Linux \u{2014} Bluetooth controllers, network adapters \u{2014} causing disconnects. Discovered at runtime. Dropped to raw evdev with selective grabbing by capability",
        "macOS rdev::grab has no clean stop API. The grab loop blocks forever with no break mechanism. Forced to process::exit(0) \u{2014} no cleanup or graceful shutdown on macOS",
        "Linux poll loop must rebuild PollFd vec each iteration: PollFd borrows the device file descriptor, so holding it across the loop body fails the borrow check",
    ],
    progress: "Working on both macOS and Linux. Grabs input, takes timestamped photos, unlocks with secret key. Clean ungrab on Linux, forced exit on macOS.",
    impact: "Systems-level programming across platforms. Drops to raw OS interfaces (evdev, nix::poll) when higher-level libraries don't fit. Custom traits on third-party types for clean abstraction of platform-specific behavior.",
    status: ProjectStatus::Done,
};
