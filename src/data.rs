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
        "Axum + PostgreSQL backend, 110k+ cards, materialized search",
        "5 workspace crates, 416 tests, zero unwrap in production code",
    ],
    impact_metric: "Live: zwipe.net. App Store submission pending.",
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
        "Production-grade auth: Argon2id with NIST-compliant 170+ pattern blocklist, rotating refresh tokens (replay-safe via delete-on-use), Password type consumed on hash so plaintext can't leak",
        "SQLx at scale \u{2014} five-strategy upsert chain handles batching, PartialEq delta detection, and per-row fallback; 88-column Scryfall sync respects PostgreSQL's 65k parameter limit (~327 cards per batch)",
        "Production posture: .unwrap, .expect, panic!, todo!, dbg!, print! all denied at compile time. 33 enforced Clippy rules, 416 tests, security audit complete, nightly Cloudflare R2 backups",
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
    impact: "Full-stack mobile delivery in pure Rust \u{2014} shared domain types across the Axum API, the Dioxus app, and a background sync service. ~45,200 lines across five crates, 416 tests, zero unwrap.",
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
        "Runs unattended for hours against unreliable APIs, recovers from every transient failure",
        "Per-failure-mode retry: 401 refresh, 504/network retry, missing-ticket permanent skip",
        "Two-tier cache survives restarts and concurrent writes via fs2 file locks",
    ],
    impact_metric: "Weeks of manual work, automated.",
    objective: "Bulk-import millions of records into Halo Software from CSV and Excel against a production API. Must survive every real failure mode: network errors, token expiry, missing tickets, partial batch failures, inconsistent data formats across client exports.",
    tags: &["rust", "csv", "etl", "api-resilience"],
    media: &[
        MediaItem {
            src: asset!("/assets/projects/halo-action-importer/full_run.mp4"),
            alt: "Full import run with batching and only-parse validation",
            caption: Some("Full run: batching across multiple sizes and only-parse mode validating inbound data"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/halo-action-importer/showing_imported_actions.mp4"),
            alt: "Imported actions resulting from the run",
            caption: Some("Resulting actions imported into Halo"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/halo-action-importer/logging_and_cache.mp4"),
            alt: "Per-run log directory and cache files",
            caption: Some("Per-run log directory and the two-tier cache"),
            kind: MediaKind::Video,
        },
    ],
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
        "Type-safe domain modeling \u{2014} invalid data rejected before any API call",
        "OAuth 2.0 with cached tokens; 30-second expiry buffer prevents edge-case 401s",
        "Cross-platform binaries via GitHub Actions matrix (Windows, macOS Intel + ARM, Linux)",
    ],
    impact_metric: "Hours to minutes. Deployed across Fortune 500 client implementations.",
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
    objective: "Learn the [Rig](https://github.com/0xPlaygrounds/rig) AI framework by building a real CLI chatbot on [Anthropic's Claude](https://www.anthropic.com/claude). Each feature should teach something new about Rig or Rust, prioritizing learning over shipping.",
    tags: &["rust", "ai", "cli", "llm"],
    media: &[
        MediaItem {
            src: asset!("/assets/projects/marvin/help_cmd_std_chat.mp4"),
            alt: "Help command and a standard chat exchange",
            caption: Some("/help and a standard chat exchange"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/marvin/commands.mp4"),
            alt: "Slash command tour",
            caption: Some("Slash command tour"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/marvin/import_chat_math_tool_call.mp4"),
            alt: "Importing a chat and a math tool call",
            caption: Some("Importing a chat and a math tool call"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/marvin/tavily_web_tool_calls.mp4"),
            alt: "Tavily web search tool calls",
            caption: Some("Tavily web search tool calls"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/marvin/model_switch_mid_chat.mp4"),
            alt: "Switching Anthropic models mid-chat",
            caption: Some("Switching Anthropic models mid-chat"),
            kind: MediaKind::Video,
        },
    ],
    approach: &[
        "Flagged a production bug in [Rig](https://github.com/0xPlaygrounds/rig): hardcoded [Anthropic](https://www.anthropic.com) model constants resolving to deprecated IDs and 404ing the API. Filed [issue #1370](https://github.com/0xPlaygrounds/rig/issues/1370), submitted a stopgap PR, and argued in-thread that constants tied to an external source of truth are the wrong primitive \u{2014} suggested fetching /v1/models at runtime instead. Maintainers acknowledged the deeper fix is a larger refactor; Marvin implements the runtime-discovery pattern locally",
        "Command pattern architecture: each slash command is a trait impl routed via a ChatInput enum. Started as a 220-line monolith, refactored to clean module boundaries as complexity grew",
        "4 [Tavily](https://tavily.com) web tools (search, extract, crawl, sitemap) sharing an Arc<TavilyClient> for efficient client reuse",
        "schemars derives JSON Schema from Rust types at compile time \u{2014} no manual schema maintenance, no drift between types and definitions",
        "Dynamic model discovery from [Anthropic's](https://www.anthropic.com) /v1/models API instead of hardcoded constants that go stale",
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
            title: "Command Dispatch",
            code: r#"// Every line of user input parses into a ChatInput variant,
// then the runner pattern-matches it to a command module.
enum ChatInput {
    Message(String),
    Help,
    Clear,
    Save,
    Load(String),
    Model(String),
    Tokens,
    Compact,
    Exit,
    // ...one variant per slash command
}

loop {
    let line = read_line()?;
    match ChatInput::parse(&line) {
        ChatInput::Message(text) => chat.stream(text).await?,
        ChatInput::Help          => commands::help::run(),
        ChatInput::Clear         => commands::clear::run(&mut chat),
        ChatInput::Save          => commands::save::run(&chat)?,
        ChatInput::Load(name)    => commands::load::run(&mut chat, &name)?,
        ChatInput::Model(name)   => commands::model::run(&mut chat, &name).await?,
        ChatInput::Tokens        => commands::tokens::run(&chat),
        ChatInput::Compact       => commands::compact::run(&mut chat).await?,
        ChatInput::Exit          => break,
        // ...
    }
}"#,
            description: "Adding a command is a two-step change: a new ChatInput variant and a new module. No conditionals in the loop, no flag-string soup. The 220-line main.rs grew into this; the architecture earned its complexity.",
        },
        Snippet {
            title: "Dynamic Model Discovery",
            code: r#"// Hardcoded model constants in Rig were 404ing on Anthropic's API.
// Fix: fetch the live model list at startup instead of trusting constants.
async fn list_models(api_key: &str) -> Result<Vec<Model>> {
    let resp = http
        .get("https://api.anthropic.com/v1/models")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .send()
        .await?
        .error_for_status()?;
    Ok(resp.json::<ModelList>().await?.data)
}

// /model with no argument lists what's actually available right now.
// /model <id> switches mid-chat. No release required when Anthropic
// ships a new model.
"#,
            description: "Rig's hardcoded constants drift the moment Anthropic ships a model \u{2014} that's the bug behind issue #1370. Marvin sidesteps the whole class of problem by asking Anthropic what exists, right now, at startup. /model with no argument is whatever Claude shipped this week.",
        },
    ],
    obstacles: &[
        "Stdout buffering: print!() without a newline requires manual flush() for immediate display during streaming",
        "[Tavily](https://tavily.com) API rejects null values for optional fields. Fixed with #[serde(skip_serializing_if = \"Option::is_none\")]",
        "Architecture outgrew the 220-line monolith. Refactored to command pattern with per-command modules. Each command independently testable",
    ],
    progress: "Active. Streaming, tools, persistence, and context management all working. Roadmap: RAG with local files, persistent memory, MCP server integration.",
    impact: "Learning project that fed real signal back to its own framework. Flagged a production bug in Rig, proposed the architectural fix in-thread, and shipped the better pattern locally rather than waiting on the upstream refactor.",
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
    media: &[
        MediaItem {
            src: asset!("/assets/projects/nighthawk/cli_repl.mp4"),
            alt: "Interactive CLI REPL session",
            caption: Some("Interactive CLI REPL"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/nighthawk/server_and_two_clients.mp4"),
            alt: "TCP server handling two concurrent clients",
            caption: Some("TCP server handling two concurrent clients"),
            kind: MediaKind::Video,
        },
    ],
    approach: &[
        "Built phase by phase from the Bitcask paper (https://riak.com/assets/bitcask-intro.pdf) to full LSM-tree \u{2014} WAL, memtable, SSTables, bloom filters, k-way compaction, TCP server, concurrency. Six distinct architectural layers, each one a real piece of how production KV systems work",
        "WAL with sync_all() after every write; 10-byte binary header (magic 0x4E48 + CRC32 + length); corruption recovery scans byte-by-byte past garbage, typed via a CorruptionType enum so callers know exactly what went wrong",
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
        "Hexagonal ports: domain Service orchestrates the cache + a CacheRepository persister",
        "Durability: atomic snapshot (temp+rename) + AOF replay through the same RESP parse path",
        "~4,500 LOC, 219 tests across every protocol, storage, and persistence layer",
    ],
    impact_metric: "~4,500 lines, 219 tests, hand-written RESP + durability",
    objective: "Build a Redis-compatible KV server by hand, layer by layer, so the muscle survives the project. TCP, RESP framing, command dispatch, in-memory KV with TTL, durable persistence (snapshot + AOF) behind a hexagonal port, graceful shutdown \u{2014} all written without reaching for a protocol crate.",
    tags: &["rust", "redis", "tcp", "protocol"],
    media: &[
        MediaItem {
            src: asset!("/assets/projects/diprotodon/server_run_redis_cli_connect.mp4"),
            alt: "redis-cli connecting to the server: SET, GET, DEL, PING",
            caption: Some("redis-cli connecting: SET, GET, DEL, then PING repeatedly"),
            kind: MediaKind::Video,
        },
    ],
    approach: &[
        "Parser-as-framer: Frame::parse_one(&[u8]) -> Result<(Frame, &[u8]), FrameError>. Returns the parsed frame plus a leftover slice borrowing from the input \u{2014} no allocation for the rest-of-buffer. Incomplete is a load-bearing error variant, not an Option",
        "Storage is HashMap<Vec<u8>, Entry> where Entry { value, absolute_ttl: Option<u64> } \u{2014} one struct per key, not parallel maps. Lazy expiry on every read path so clients never see expired keys, plus a background sweeper thread for memory hygiene",
        "Hexagonal ports: the domain defines two trait boundaries \u{2014} CacheRepository (the persister implements it) and CacheService (the domain Service implements it; the session calls it). Adapter errors map into a domain-owned RepositoryError at the boundary, so the domain never names an outbound type",
        "Durability via snapshot + AOF, hybrid recovery. Snapshot is a wincode dump written temp-file-then-rename (atomic; never a half-written file). AOF is the wire protocol \u{2014} each mutating command is appended as the exact RESP bytes a client would have sent, so replay reuses Frame::parse_one + Command::try_from. Snapshot-then-truncate compaction holds the cache lock across both so no mutation escapes between the two",
        "Graceful shutdown without a signal-handling crate: Arc<AtomicBool> flag, TcpListener::set_nonblocking(true) so accept() returns WouldBlock and the loop can check the flag, stdin EOF or \"quit\" as the trigger. Every spawned thread is collected as a JoinHandle and joined cleanly before run() returns",
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
            title: "Graceful Shutdown, No Signal Crate",
            code: r#"let listener = TcpListener::bind(BIND_ADDRESS)?;
listener.set_nonblocking(true)?;  // accept() returns WouldBlock instead of parking
let shutdown = Arc::new(AtomicBool::new(false));
let mut handles = Vec::<JoinHandle<()>>::new();

// stdin trigger: EOF or "quit"/"exit" flips the flag
let shutdown_clone = shutdown.clone();
handles.push(spawn(move || {
    let mut s = String::new();
    loop {
        s.clear();
        match std::io::stdin().read_line(&mut s) {
            Ok(0) => { shutdown_clone.store(true, Ordering::Relaxed); break; }
            Ok(_) if matches!(s.trim().to_lowercase().as_str(), "quit" | "exit") => {
                shutdown_clone.store(true, Ordering::Relaxed); break;
            }
            _ => (),
        }
    }
}));

// main loop: check flag, throttle WouldBlock, prune finished handles
loop {
    if shutdown.load(Ordering::Relaxed) { break; }
    handles.retain(|h| !h.is_finished());
    match listener.accept() {
        Ok((stream, _)) => { /* spawn session, push handle */ }
        Err(e) if e.kind() == ErrorKind::WouldBlock => {
            std::thread::sleep(Duration::from_millis(50));  // don't burn a core
        }
        Err(e) => eprintln!("accept failed: {e}"),
    }
}
for h in handles { let _ = h.join(); }"#,
            description: "No ctrlc/signal-hook crate. set_nonblocking turns accept() into a poll \u{2014} pair it with a 50ms sleep on WouldBlock so the loop checks the shutdown flag instead of burning a CPU. A stdin thread is the trigger (EOF or \"quit\"). Every spawned worker (sessions, persistence, sweeper) is collected as a JoinHandle and joined before run() returns, so in-flight work finishes cleanly.",
        },
        Snippet {
            title: "AOF Is the Wire Protocol",
            code: r#"// Every mutation logged as the exact RESP bytes a client would have sent.
// One serializer (Frame::write_to) for both the network and the log.
impl From<MutatingCommand> for Frame {
    fn from(value: MutatingCommand) -> Self {
        match value {
            MC::Set { key, value } => Frame::Array(vec![
                Frame::BulkString(b"SET".to_vec()),
                Frame::BulkString(key),
                Frame::BulkString(value),
            ]),
            // ...DEL, EXPIRE, EXPIREAT, PERSIST
        }
    }
}

// Replay reuses Frame::parse_one + Command::try_from — the same inbound
// parse path the network uses. Cache-only execute() so replay doesn't
// re-write the log it's reading.
pub fn replay(&self, cache: &Cache) -> Result<(), AofError> {
    let mut bytes = fs::read(&*self.path)?;
    let mut buf = bytes.as_slice();
    loop {
        match Frame::parse_one(buf) {
            Ok((frame, rest)) => {
                buf = rest;
                cache.execute(Command::try_from(frame)?)?;
            }
            // Trailing torn frame = crash mid-append. Stop cleanly,
            // keep everything parsed so far.
            Err(FrameError::Incomplete) => break,
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}

// Checkpoint: snapshot then truncate, both under the cache lock so
// no mutation escapes between the two. Crash between = re-apply
// already-snapshotted commands. Harmless.
fn snapshot(&self, cache: &Cache) -> Result<(), RepositoryError> {
    let guard = cache.lock().map_err(|_| PersisterError::MutexPoisoned)?;
    self.snapshot.store(&guard)?;  // wincode dump via temp+rename
    self.aof.clear()?;             // truncate log
    Ok(())
}"#,
            description: "The AOF being byte-for-byte the wire protocol means replay reuses the inbound parse path — no separate decoder, no version-skew between disk and network format. Snapshots use temp-file-then-rename for atomicity. Checkpointing holds the cache lock across snapshot+clear; the order (snapshot first, clear second) means a crash between them only causes harmless re-application of already-durable commands.",
        },
    ],
    obstacles: &[
        "Self-deadlock on the TTL read path: get_absolute_ttl held the mutex guard, then called self.remove() on the expired branch \u{2014} which tries to re-lock the same std::sync::Mutex from the same thread. std mutexes aren't reentrant; the thread hangs forever. Fix: drop(guard) explicitly before the re-entry. Guards live to end of scope, not end of statement",
        "get_frame read-before-parse bug: original loop called reader.read() first, then parse_frame(). When one TCP read delivered multiple frames (common \u{2014} TCP coalesces small writes), the first call returned the first frame fine; the second call's first move was a read that hit EOF, returned None, and the queued second frame in the buffer was never seen. Fix: parse first, only read on Incomplete, return None when an Incomplete is followed by a zero-byte read",
        "AOF/snapshot atomicity: between the snapshot read and the AOF truncate, a writer could land a new mutation that gets wiped without ever being captured. Fix: hold the cache lock across both. Order matters too \u{2014} snapshot first, then clear, so a crash between just re-applies already-durable commands. Harmless.",
    ],
    progress: "M1\u{2013}M4 complete. All commands over real RESP, snapshot + AOF persistence behind a hexagonal port, graceful shutdown end-to-end. 219 tests. Next: async migration, then Pub/Sub, then MULTI/EXEC.",
    impact: "Paired with nighthawk to cover both halves of how production KV systems are built \u{2014} nighthawk the on-disk LSM storage engine, diprotodon the in-memory protocol server with WAL-style durability. Both hand-written, both interoperable with real clients (redis-cli for diprotodon, raw TCP for nighthawk).",
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
    media: &[
        MediaItem {
            src: asset!("/assets/projects/capture/demo.mp4"),
            alt: "Keyboard and mouse input triggering intruder captures",
            caption: Some("Keyboard and mouse input triggering intruder captures"),
            kind: MediaKind::Video,
        },
    ],
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
