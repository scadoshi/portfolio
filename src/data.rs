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
    /// Live-site URL, when the project has one (rendered as an action pill).
    pub site_url: Option<&'static str>,
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
    pub fn label(self) -> &'static str {
        match self {
            Self::Done => "Done",
            Self::Doing => "Doing",
        }
    }
    /// Maps to the shared status pill; the label ("Done"/"Doing") is passed
    /// separately so the pill keeps our wording, not the component's default.
    pub fn banner_status(self) -> zwipe_components::BannerStatus {
        match self {
            Self::Done => zwipe_components::BannerStatus::Done,
            Self::Doing => zwipe_components::BannerStatus::Doing,
        }
    }
}

pub struct Snippet {
    pub title: &'static str,
    /// highlight.js language class, e.g. "rust" or "csharp". Only the
    /// grammars loaded in main.rs will actually color.
    pub lang: &'static str,
    pub code: &'static str,
    pub description: &'static str,
}

const CAIRN: Project = Project {
    name: "Cairn",
    slug: "cairn",
    headline: "Lifetime rep counter for iOS. Local SQLite, no server, no account. Nine months of my own training data.",
    category: "Mobile App",
    repo_url: "https://github.com/scadoshi/cairn",
    summary: "A counter for things you do every day. It keeps the lifetime total plus the rates that make a total mean something.",
    card_bullets: &[
        "Rust + Dioxus 0.7, single crate, hexagonal: the domain has no UI, no SQLite, no clock",
        "Local SQLite with a five-step migration ladder; no server and nothing to sign into",
        "Goals per day, week or year, with pace and a tag counting down the day's share",
        "~6,250 lines, 73 tests, 2,400 of them pure domain",
    ],
    impact_metric: "97,600 reps logged across 224 days",
    objective: "Count the things I actually do, forever, without an account or a subscription. A lifetime total is only interesting next to the rates around it: this year, per day, where I stand against a goal, what today still owes. Everything lives on the phone.",
    tags: &["rust", "dioxus", "ios", "sqlite"],
    media: &[
        MediaItem {
            src: asset!("/assets/projects/cairn/00-demo.mp4"),
            alt: "Logging reps across three counters, opening one, and changing a setting",
            caption: Some(
                "A minute of ordinary use: tap, watch the goal tag close, open a counter",
            ),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/cairn/01-home.jpeg"),
            alt: "Cairn home screen: the mark, today's totals across every counter, a quote, and the first counter card",
            caption: Some("One screen. Today across every counter, then the counters themselves"),
            kind: MediaKind::Image,
        },
        MediaItem {
            src: asset!("/assets/projects/cairn/02-counters.jpeg"),
            alt: "Three counter cards with goal tags counting down, and a +10 toast over the last one",
            caption: Some("Every counter, each with what today still owes. A tap logs and says so"),
            kind: MediaKind::Image,
        },
        MediaItem {
            src: asset!("/assets/projects/cairn/03-counter.jpeg"),
            alt: "One counter: goal pace figures and a 60-day chart with a 7-day average",
            caption: Some("Pace against a 200-a-day goal, and 60 days with the 7-day average"),
            kind: MediaKind::Image,
        },
        MediaItem {
            src: asset!("/assets/projects/cairn/04-config.jpeg"),
            alt: "Config screen showing theme, mark, dark mode, date format, day and week start, and rest days",
            caption: Some("Every setting explains itself behind the question mark beside it"),
            kind: MediaKind::Image,
        },
    ],
    approach: &[
        "Hexagonal in one crate. The domain never touches Dioxus, rusqlite or the clock: today is an argument, which is what makes the stats testable and what will let them run on a watch",
        "Events are the source of truth and daily totals are derived, so changing when a day starts rebuilds history instead of losing it",
        "The hourly quote is a pure function of the clock hour, so there is no cache to invalidate. The stride through the list is coprime with its length, so all 41 appear before any repeat",
        "Every deploy backs the phone up first, because it holds taps that exist nowhere else",
    ],
    snippets: &[
        Snippet {
            title: "The quote with no state",
            lang: "rust",
            code: r"/// The quote for the hour that `at` falls in.
pub fn for_time<Tz: TimeZone>(at: &DateTime<Tz>) -> Option<&'static Quote> {
    at_hour(at.timestamp().div_euclid(3600))
}

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
}",
            description: "Whole hours since the epoch, so it turns over on the hour rather than an hour after launch. Nothing is written down, so nothing can drift. The comment is there because I shipped the divide by zero to my own phone.",
        },
        Snippet {
            title: "The test the suite was missing",
            lang: "rust",
            code: r#"/// Every other store test starts from `in_memory()`, which is version 0,
/// so all five rungs always run and the version guards are never
/// exercised. That leaves the path a real phone takes, an existing
/// database being upgraded, with no coverage at all: changing
/// `if version < 5` to `if version < 4` used to pass the whole suite
/// while breaking every install that already had data.
#[test]
fn opens_a_database_left_at_every_older_version() {
    let ladder = [SCHEMA_V1, SCHEMA_V2, SCHEMA_V3, SCHEMA_V4, SCHEMA_V5];
    for version in 1..=SCHEMA_VERSION {
        let path = aged_db(&dir, version, &ladder[..version as usize]);
        let s = SqliteStore::open(&path)
            .unwrap_or_else(|e| panic!("v{version} database would not open: {e}"));
        assert_eq!(s.list_counters().unwrap().len(), 1, "v{version} lost data");
    }
}"#,
            description: "Found by mutation testing rather than by reading. Two one-character changes to the migration ladder passed all 64 tests while breaking every existing install, because nothing ever opened a database that already had a version.",
        },
    ],
    obstacles: &[
        "The home screen died on launch. I read context inside the loop over counters, so the first render ran zero hooks and the second ran three, and Dioxus treats a changed hook count as fatal. `dx check` catches it, but it had been red on an unrelated call for weeks",
        "`-delta.min(n)` negates the comparison rather than `delta`, so every subtraction became an addition. It lasted about a minute on my phone",
        "Most famous quotes are misattributed. \"You don't stop running because you get old\" is Jack Kirk, not McDougall, and Tyson never said \"punched in the mouth\". A third of the candidates were rejected",
    ],
    progress: "On my phone since 22 September 2026, with nine months of imported history: 224 days, 9,829 taps, 97,600 reps. Counters, goals with pace, trend charts, streaks, CSV export and the hourly quote are in. TestFlight is next.",
    impact: "The thing I open every day, which is the only real test of a personal tool. It also taught me to distrust a green suite: mutation testing found three assertions that could not fail.",
    site_url: None,
    status: ProjectStatus::Doing,
};

pub fn featured_projects() -> &'static [Project] {
    &[ZWIPE, HALO_ACTION_IMPORTER, HALO_CUSTOM_FIELD_BUILDER]
}

pub fn side_quests() -> &'static [Project] {
    &[
        CHICKADEE, STELLER, MARVIN, GOTCHA, UPSEE, CAIRN, RUSTMAS, SHARPMAS,
    ]
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
    headline: "Full-stack MTG deck builder. Axum backend, Dioxus frontend, PostgreSQL, 118k+ printings.",
    category: "Full-Stack Application",
    repo_url: "https://github.com/scadoshi/zwipe",
    summary: "Mobile-first Magic: The Gathering deck builder with swipe-based navigation.",
    card_bullets: &[
        "Native iOS + Android from one Dioxus codebase",
        "Axum + PostgreSQL backend, 118k+ printings, materialized search",
        "6 workspace crates, 745 tests, unwrap banned by CI",
    ],
    impact_metric: "Live on the App Store, Google Play, and zwipe.net.",
    objective: "Build a full-stack MTG deck builder with swipe-based navigation as a single-language Rust project. Six workspace crates: zwipe-core (shared domain), zerver (Axum API, plus a zervice background-sync binary), zwiper (Dioxus mobile app), zwipe-client (the typed API client both clients call), zwipe-components (shared UI), zite (the public site: guides, changelog, shared deck pages). Full commander support: partners, backgrounds, oathbreaker. See the [architecture](https://zwipe.net/about) and [demo](https://zwipe.net). Live on the App Store and Google Play.",
    tags: &["rust", "full-stack", "ios", "dioxus", "postgresql"],
    media: &[
        MediaItem {
            src: asset!("/assets/projects/zwipe/1_create_deck.mp4"),
            alt: "Creating a deck and picking a commander",
            caption: Some("Create a deck and pick your commander"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/zwipe/3_add_cards.mp4"),
            alt: "Swiping new cards into a deck",
            caption: Some("Swipe new cards into your deck"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/zwipe/5_filter.mp4"),
            alt: "Filtering the card pool then swiping to add",
            caption: Some("Filter the card pool, then swipe to add"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/zwipe/6_card_details.mp4"),
            alt: "Opening full card details while swiping",
            caption: Some("Open full card details while you swipe"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/zwipe/7_oracle_tags.mp4"),
            alt: "Browsing the tag dictionary and filtering by oracle tag",
            caption: Some("Browse the tag dictionary and filter by oracle tag"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/zwipe/4_remove_cards.mp4"),
            alt: "Swiping cards back out of a deck",
            caption: Some("Swipe cards back out of your deck"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/zwipe/8_deck_cards.mp4"),
            alt: "Browsing a deck's art, printings, and groupings",
            caption: Some("Browse your deck: art, printings, and groupings"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/zwipe/9_mvps.mp4"),
            alt: "Starring MVP cards so they lead the deck",
            caption: Some("Star MVPs so your key cards lead the deck"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/zwipe/2_import.mp4"),
            alt: "Importing a decklist from a link",
            caption: Some("Import a decklist from a link"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/zwipe/10_deck_stats.mp4"),
            alt: "Checking a deck's stats, curve, and draw odds",
            caption: Some("Check your deck's stats, curve, and draw odds"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/zwipe/11_share_deck.mp4"),
            alt: "Sharing a deck with a public link",
            caption: Some("Share any deck with a public link"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/zwipe/12_profile.mp4"),
            alt: "Switching themes and reading the changelog",
            caption: Some("Switch themes and catch up on the changelog"),
            kind: MediaKind::Video,
        },
    ],
    approach: &[
        "One Rust codebase compiles to native iOS and Android through Dioxus. No JS bridge, no separate frontend repo",
        "A shared domain crate backs both the Axum API and the app, so the filter UI and the server's SQL search are built from the same query builder and cannot drift apart",
        "Argon2id, single-use rotating refresh tokens, and a Password type that is consumed on hash so plaintext has nowhere to leak to",
        "Every API call is described once, method and path and response type together, and one generic function sends them all. The app and the site cannot disagree about what an endpoint looks like",
        "CI promotes 22 clippy rules to errors, unwrap among them. 745 tests, nightly Postgres backups to R2",
    ],
    snippets: &[
        Snippet {
            title: "One Search Predicate, Two Front Doors",
            lang: "rust",
            code: r#"// CardCriteria: the shared predicate core (~50 fields) with matches()
// CardQuery:    criteria + Limit + offset + sort, the server's SQL path
// Cards:        a Vec<Card> already in hand, criteria only, no pagination

let builder = CardQueryBuilder::with_name_contains("dragon");

// Server path: a bounded query POSTed to the API, compiled to SQL
// against the latest_cards materialized view
let query: CardQuery = builder.build()?;

// Device path: filter a loaded deck locally with the same predicate,
// no round-trip
let shown: Vec<Card> = Cards::from(deck_cards)
    .matching(&builder.build_criteria()?)
    .sorted(CardSortKey::Cmc, true)
    .into();

// Grouping stays an extension trait on Vec<Card>: fixed-order labeled
// buckets, and CardRole grouping is multi-bucket (one card, many roles)
let groups = deck_cards.group_by(GroupByOption::CardType);"#,
            description: "One predicate core, two front doors. The server compiles it to SQL, the app runs it in memory, and the same builder emits either, so the filter UI cannot drift from the API.",
        },
        Snippet {
            title: "Swipe Gesture Engine",
            lang: "rust",
            code: r"// The gesture logic lives once, in a trait. Touch and mouse adapt to it.
trait OnSwipe {
    fn onswipestart(&mut self, point: ClientPoint);
    fn onswipemove(&mut self, point: ClientPoint);
    fn onswipeend(&mut self, point: ClientPoint, config: &SwipeConfig);
}

// A swipe registers on distance OR velocity, and the axis locks on the
// first movement so a diagonal drag cannot fire two directions.
if distance > config.distance_threshold
    || (distance > 10.0 && speed > config.speed_threshold)
{
    match self.traversing_axis {
        Some(Axis::X) if delta.x < 0.0 => self.latest_swipe = Some(Dir::Left),
        Some(Axis::X) if delta.x > 0.0 => self.latest_swipe = Some(Dir::Right),
        _ => {}
    }
}",
            description: "Built across 11 files with no gesture library. Axis locking and a velocity threshold are what make quick flicks register without diagonal drags firing twice.",
        },
        Snippet {
            title: "87-Column Upsert Automation",
            lang: "rust",
            code: r#"// One constant holds all 87 field names; everything else derives from it,
// so a new Scryfall column never has to be added in five places.
const FIELDS: &str = "arena_id id lang mtgo_id oracle_id cmc ...";

// Traits give QueryBuilder the card-shaped methods, so the whole
// 87-column upsert is one chain.
QueryBuilder::new("INSERT INTO scryfall_data (")
    .push(scryfall_data_fields())
    .push(") VALUES ")
    .bind_cards(scryfall_data)
    .push(bulk_upsert_conflict_fields())
    .push(" RETURNING *;")

// Five strategies compose, each adding one capability: chunk and skip
// unchanged, diff against the DB, chunk within the parameter limit, one
// statement per chunk, then single-card fallback."#,
            description: "Postgres caps a statement at 65,535 parameters, which at 87 fields a card means chunking. The single-card fallback is what stops one bad record taking 100k others with it.",
        },
    ],
    obstacles: &[
        "ScryfallData has 87 fields, and hexagonal architecture wants a separate database type. Maintaining 87 fields twice felt untenable solo, so the domain type carried a feature-gated sqlx derive for a while. That bend has since been unbent: a real database type lives in the outbound layer and converts inward",
        "Postgres caps a statement at 65,535 parameters, and 87 fields per card means batching. Zwipe uses half the limit, which works out to 376 cards a batch. Five upsert strategies compose to handle it, with card-by-card fallback so one bad record never blocks 100k others",
        "Search over 118k printings returned the same card once per printing, and substring search crawled. A materialized view pre-deduplicates to one row per name with trigram indexes, refreshed nightly",
    ],
    progress: "Live on the [App Store](https://apps.apple.com/us/app/zwipe-tcg/id6761341603), [Google Play](https://play.google.com/store/apps/details?id=com.scadoshi.zwipe), and [zwipe.net](https://zwipe.net), with regular releases since launch. Full deck management, swipe-based building, the commander system (partners, backgrounds, oathbreaker), synergy-ranked card suggestions, deck sharing via public links, draw-odds and price/land targets, card roles, maybeboard/sideboard, import/export, and 31 themes. Security audit complete; nightly backups.",
    impact: "Full-stack mobile delivery in pure Rust: shared domain types across the Axum API, the Dioxus app, and a background sync service. over 100,000 lines across five crates, 600+ tests, unwrap banned by CI.",
    site_url: Some("https://zwipe.net"),
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
            caption: Some(
                "Full run: batching across multiple sizes and only-parse mode validating inbound data",
            ),
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
        "Recovery is per failure mode rather than blanket retry. A 401 refreshes the token, a 504 retries immediately, a missing ticket is skipped for the rest of the run, and a bad row is skipped without taking the batch with it",
        "When a batch fails, actions are regrouped by ticket and retried per group. That salvages the most imports and names exactly which tickets do not exist",
        "Deduplication moved from a Halo report, to a split across resources, to a local cache of roughly 8M existing action IDs pulled straight from the database. Each stage was the previous one falling over",
    ],
    snippets: &[
        Snippet {
            title: "Resilience Pattern",
            lang: "rust",
            code: r"// Every failure mode has a specific recovery strategy
401 Unauthorized    → refresh token, retry immediately
504 Gateway Timeout → retry immediately (no delay)
Network error       → retry immediately
Missing ticket      → mark ticket as missing, skip future actions
Deserialization     → skip row, continue processing",
            description: "No blanket retry-with-backoff. Each failure mode gets the recovery strategy that actually makes sense for it.",
        },
        Snippet {
            title: "Retry Strategy Evolution",
            lang: "rust",
            code: r"// v1: Binary search to find bad ticket in failed batch
//     O(log(batch_size) * failures): too many API calls
//
// v2: Ticket-grouped retry (current)
//     Group actions by ticket_id, retry each group
//     O(unique_tickets): maximizes successful imports
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
}",
            description: "The commit history shows this progression. Binary search was clever but wrong. Ticket-grouped retry is simpler and more efficient.",
        },
        Snippet {
            title: "Cache Evolution",
            lang: "rust",
            code: r"// v1: Single report endpoint
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
}",
            description: "The biggest obstacle was remembering work already done. Each stage worked until the dataset outgrew it. I was the only one importing so a local cache was safe as the source of truth.",
        },
    ],
    obstacles: &[
        "Dedup at scale. A single report worked at ~100k IDs and timed out as the data grew; splitting it bought time and then timed out too. The answer was a direct DB query for all ~8M IDs, safe only because I was the sole importer",
        "Binary search was the wrong abstraction for batch failures. Too many API calls for what ticket-grouped retry does more simply",
    ],
    progress: "Production. Actively used for real data migrations.",
    impact: "Reduced migration timelines from weeks to days. Runs unattended for hours against millions of records with automatic recovery from any transient failure.",
    site_url: None,
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
        "Type-safe domain modeling: invalid data rejected before any API call",
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
        "Name, Label and FieldType validate at construction, so a malformed field is rejected before anything reaches the API",
        "OAuth tokens cache behind an Arc<Mutex> with a 30-second expiry buffer, which closes the window where a token passes the check and expires before the call",
        "Import mode runs straight through; debug mode walks field by field with process, skip or quit, which is what made it usable against real client data",
    ],
    snippets: &[
        Snippet {
            title: "Layered Architecture",
            lang: "rust",
            code: r"// bin/main.rs: orchestration only
// lib/: all logic lives here
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
//   http_custom_field  Domain-to-API type mapping via From impl",
            description: "Same inbound/domain/outbound pattern used in Zwipe. Each layer has a clear responsibility. Domain types know nothing about CSV or HTTP. The bin crate just wires the layers together.",
        },
        Snippet {
            title: "Domain Validation",
            lang: "rust",
            code: r"// Newtypes with validation at construction
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
}",
            description: "Invalid data is rejected at parse time with specific error messages (row number + field name). By the time a CustomField reaches the API client, it is guaranteed valid.",
        },
    ],
    obstacles: &[
        "Selection options contain commas, and Halo's API separates options by comma. Options get stripped before joining",
        "Real client CSVs do not keep columns in the expected order, so parsing goes by header position rather than index, and errors name the row and the field",
    ],
    progress: "Shipped. Tagged v1.0.0 with cross-platform releases via GitHub Actions. Actively used in production for client implementations.",
    impact: "Reduced enterprise configuration time from hours to minutes. Ships as tagged cross-platform binaries, so an implementer runs it without a Rust toolchain.",
    site_url: None,
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
        "Found a live bug in [Rig](https://github.com/0xPlaygrounds/rig): hardcoded model constants had gone stale and were 404ing the API. Filed [issue #1370](https://github.com/0xPlaygrounds/rig/issues/1370) with a stopgap PR, and argued in the thread that constants pinned to someone else's source of truth are the wrong primitive. Marvin fetches the model list at runtime instead",
        "Each slash command is a trait impl routed through an enum. It started as a 220-line monolith and grew module boundaries as it earned them",
        "schemars derives the JSON schema for each tool from the Rust types, so the definitions cannot drift from the code",
    ],
    snippets: &[
        Snippet {
            title: "Tool Architecture",
            lang: "rust",
            code: r"// Each tool uses schemars for automatic JSON Schema generation
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
}",
            description: "schemars derives JSON Schema from Rust types at compile time. No manual schema writing, no drift between types and definitions. Arc sharing keeps a single HTTP client across all 4 web tools.",
        },
        Snippet {
            title: "Command Dispatch",
            lang: "rust",
            code: r"// Every line of input parses into a ChatInput variant, then the runner
// matches it to a command module. Adding a command is a variant and an arm.
enum ChatInput {
    Message(String),
    Model(String),
    Compact,
    Exit,
    // ...one variant per slash command
}

match ChatInput::parse(&read_line()?) {
    ChatInput::Message(text) => chat.stream(text).await?,
    ChatInput::Model(name)   => commands::model::run(&mut chat, &name).await?,
    ChatInput::Compact       => commands::compact::run(&mut chat).await?,
    ChatInput::Exit          => break,
    // ...
}",
            description: "Adding a command is a two-step change: a new ChatInput variant and a new module. No conditionals in the loop, no flag-string soup. The 220-line main.rs grew into this; the architecture earned its complexity.",
        },
        Snippet {
            title: "Dynamic Model Discovery",
            lang: "rust",
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
            description: "Rig's hardcoded constants drift the moment Anthropic ships a model; that's the bug behind issue #1370. Marvin sidesteps the whole class of problem by asking Anthropic what exists, right now, at startup. /model with no argument is whatever Claude shipped this week.",
        },
    ],
    obstacles: &[
        "Streaming output appeared in chunks until it turned out print!() without a newline needs a manual flush()",
        "Tavily rejects null for optional fields, which serde sends by default. #[serde(skip_serializing_if)] fixed it",
    ],
    progress: "Active. Streaming, tools, persistence, and context management all working. Roadmap: RAG with local files, persistent memory, MCP server integration.",
    impact: "A learning project that ended up sending a fix back to the framework it was built on. Flagged a production bug in Rig, proposed the architectural fix in-thread, and shipped the better pattern locally rather than waiting on the upstream refactor.",
    site_url: None,
    status: ProjectStatus::Done,
};

const CHICKADEE: Project = Project {
    name: "Chickadee",
    slug: "chickadee",
    headline: "LSM-tree key-value database from scratch. TCP server, concurrent connections, WAL, SSTables, bloom filters, k-way compaction.",
    category: "Database Internals",
    repo_url: "https://github.com/scadoshi/chickadee",
    summary: "LSM-tree key-value database built phase by phase from the Bitcask paper. The architecture behind LevelDB, RocksDB, and Cassandra.",
    card_bullets: &[
        "TCP server with thread-per-connection concurrency, per-command locking",
        "WAL durability, BTreeMap memtable, bloom-filtered SSTables",
        "K-way merge compaction; byte-level corruption recovery",
        "~2,250 LOC, 99 tests",
    ],
    impact_metric: "~2,250 lines, 99 tests, 6 phases",
    objective: "Build a key-value database incrementally from the Bitcask paper (https://riak.com/assets/bitcask-intro.pdf) toward the LSM-tree architecture that powers LevelDB, RocksDB, and Cassandra. Each phase adds a real layer: durability, sorted storage, probabilistic search, compaction, crash recovery, networking, concurrency.",
    tags: &["rust", "kv-store", "lsm-tree", "networking"],
    media: &[
        MediaItem {
            src: asset!("/assets/projects/chickadee/01-cli-basics.mp4"),
            alt: "CLI REPL: set, get, and a delete that writes a tombstone",
            caption: Some("Set, get, and a delete that writes a tombstone rather than erasing"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/chickadee/02-wal-on-disk.mp4"),
            alt: "Hex dump of the write-ahead log showing headers, magic bytes, keys and values",
            caption: Some(
                "The WAL in hex: a 10-byte header per entry, and CD marking every boundary",
            ),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/chickadee/03-durability.mp4"),
            alt: "Process killed and restarted, with keys rebuilt by replaying the log",
            caption: Some("Kill the process and the memtable dies with it. The log rebuilds it"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/chickadee/04-corruption-recovery.mp4"),
            alt: "One byte flipped in the log; the bad entry is dropped and later entries survive",
            caption: Some("One flipped byte: the bad entry is dropped, everything after it lives"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/chickadee/05-concurrent-clients.mp4"),
            alt: "Two TCP clients reading and writing the same store on separate threads",
            caption: Some("Two connections, two threads, one store"),
            kind: MediaKind::Video,
        },
    ],
    approach: &[
        "Built phase by phase from the Bitcask paper up to a full LSM-tree: WAL, memtable, SSTables, bloom filters, k-way compaction, then a TCP server on top",
        "Every entry carries a 10-byte header with magic bytes and a CRC32. When a checksum fails the reader scans byte-by-byte to the next magic marker, so corruption costs one entry instead of the file",
        "Bloom filters sit in each SSTable as a footer. Kirsch-Mitzenmacher double hashing, two xxh3 seeds, about 1% false positives, which keeps negative lookups off the disk",
        "Compaction merges every SSTable at once rather than pairwise, and drops tombstones that have outlived what they were hiding",
    ],
    snippets: &[
        Snippet {
            title: "Corruption Recovery",
            lang: "rust",
            code: r"// 10-byte header: [magic: 0x4443 (2B)][crc32 (4B)][len (4B)]
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
                // CRC32 doesn't match, skip this entry
                self.reader.seek(SeekFrom::Current(-(HEADER_SIZE as i64 - 1)))?;
            }
            Err(CorruptionType::ParseError) => {
                // Magic+CRC valid but wincode parse failed, skip entry
            }
        }
    }
}",
            description: "A crash mid-write can leave partial data in the WAL. Instead of failing on startup, the reader scans past garbage to find the next valid entry. Four distinct corruption types so callers know exactly what went wrong.",
        },
        Snippet {
            title: "Bloom Filter",
            lang: "rust",
            code: r"// Kirsch-Mitzenmacher: two xxh3 seeds, k=7, ~1% false positive rate
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
}",
            description: "insert() and may_contain() are symmetric: same positions(), opposite bit operations. Two hash seeds replace k separate functions. The blanket impl makes the trait the extension point: no wrapper, just import it.",
        },
        Snippet {
            title: "K-Way Merge: compact()",
            lang: "rust",
            code: r"// Every SSTable is merged at once rather than pairwise. Each holds a
// cursor, and each pass takes the globally smallest key across all of them.
loop {
    sstables.retain(|(entry, _)| entry.is_some());
    if sstables.is_empty() { break; }

    let min = /* smallest key across all active cursors */;

    for (entry, sstable) in sstables.iter_mut() {
        let is_participant = entry.as_ref().unwrap().key() == min;
        // Newest file wins: seen_keys means a later version already landed.
        if is_participant && !seen_keys.contains(min.as_str()) {
            seen_keys.insert(min.clone());
            if let Entry::Set { .. } = entry.as_ref().unwrap() {
                memtable.process(entry.as_ref().unwrap().clone())?;
            }
            // A tombstone is marked seen and dropped. It has done its job.
        }
        if is_participant { *entry = sstable.read_next_entry()?; }
    }
}",
            description: "Files are read newest-to-oldest, so the first version of a key wins and the rest are skipped. Tombstones get recorded and then dropped, which is what stops them accumulating forever.",
        },
    ],
    obstacles: &[
        "Tombstone resurrection. Splitting Entry into separate WAL and SSTable types assumed SSTables only ever held writes, so deleting a flushed key cleared the memtable while the SSTable still had the original. The key came back on the next read. One Entry enum now threads tombstones through every layer",
        "flush_count has to be read from the existing SSTable count at startup rather than starting at zero, or a restart compacts on the wrong schedule",
    ],
    progress: "Complete. All 6 phases done. 99 tests across 7 modules including TCP integration tests.",
    impact: "Started from a paper and ended with a database you can connect to. The same storage architecture behind LevelDB, RocksDB and Cassandra, built a layer at a time.",
    site_url: None,
    status: ProjectStatus::Done,
};

const STELLER: Project = Project {
    name: "Steller",
    slug: "steller",
    headline: "Redis-compatible in-memory KV server in Rust. Hand-written RESP wire protocol, real redis-cli clients connect.",
    category: "Network Protocols & Systems",
    repo_url: "https://github.com/scadoshi/steller",
    summary: "Redis-compatible in-memory KV server in Rust. Real redis-cli clients connect.",
    card_bullets: &[
        "Hand-written RESP parser. No protocol crate, no async runtime",
        "The append-only log is the wire format, so replay reuses the inbound parse path",
        "Pub/sub fan-out over per-session writer threads",
        "~5,900 lines, 238 tests, benchmarked against Redis 8",
    ],
    impact_metric: "~5,900 lines, 238 tests, benchmarked against Redis 8",
    objective: "Build a Redis-compatible KV server by hand, layer by layer, so the muscle survives the project. TCP, RESP framing, command dispatch, in-memory KV with TTL, durable persistence (snapshot + AOF) behind a hexagonal port, graceful shutdown, pub/sub fan-out. All written without reaching for a protocol crate.",
    tags: &["rust", "redis", "tcp", "protocol"],
    media: &[
        MediaItem {
            src: asset!("/assets/projects/steller/01-basics.mp4"),
            alt: "redis-cli connecting to steller: PING, SET, GET, EXISTS, DEL",
            caption: Some("A real redis-cli connects and never notices it isn't Redis"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/steller/02-ttl-and-set-options.mp4"),
            alt: "SET with EX and PX options, and a key expiring on its own",
            caption: Some("SET options on millisecond deadlines, and a PX key expiring live"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/steller/03-pubsub.mp4"),
            alt: "Two clients: one subscribed, one publishing, with the push delivered out of band",
            caption: Some(
                "Pub/sub without async: the push lands while the subscriber's reader is blocked",
            ),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/steller/04-persistence-across-restart.mp4"),
            alt: "Server shut down and restarted, with the value and its TTL both surviving",
            caption: Some("Restart: the value survives and the TTL comes back lower, not reset"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/steller/05-error-handling.mp4"),
            alt: "Three malformed commands answered with errors, session still serving",
            caption: Some("Bad input gets an error and the session keeps going"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/steller/06-benchmark-vs-redis.svg"),
            alt: "SET throughput by client count, steller against Redis 8, from one to 256 clients",
            caption: Some(
                "Against Redis 8, median of three runs. Ahead under 32 clients, flat above it, and the process dies near 3,000",
            ),
            kind: MediaKind::Image,
        },
    ],
    approach: &[
        "The parser is the framer. parse_one returns the frame plus whatever bytes are left over, and Incomplete is a real error variant rather than an Option, because the read loop leans on the difference between \"need more\" and \"malformed\"",
        "The AOF is the wire protocol. Every mutation is logged as the exact RESP bytes a client would have sent, so replay reuses Frame::parse_one and Command::try_from instead of a second decoder that could drift",
        "Units are newtypes. Seconds and Milliseconds are distinct types, and Seconds only survives between the parser reading a token and converting it. That caught a live bug where replay was multiplying every deadline by 1000 on each restart",
        "Pub/sub with no async. Each session splits into a reader and a writer thread, so a published message lands on a subscriber while its own reader is still blocked on the socket",
        "Blocking accept, woken on shutdown by a self-connect. The stdin thread flips the flag with Release, then opens one throwaway connection to the listener's own address; the accept loop loads with Acquire as it comes in, drops it, and exits. An idle server costs nothing and a new connection is picked up at once. The non-blocking loop with a 50ms sleep it replaced put 49ms on every connection's first request, which the first benchmark run made visible",
    ],
    snippets: &[
        Snippet {
            title: "Parser-as-Framer",
            lang: "rust",
            code: r"pub fn parse_one(bytes: &[u8]) -> Result<(Frame, &[u8]), FrameError> {
    let (header, rest) = bytes.split_crlf().ok_or(FrameError::Incomplete)?;
    let (sigil, len_bytes) = header.split_first().ok_or(FrameError::Malformed)?;
    let len: usize = std::str::from_utf8(len_bytes)?.parse()?;
    match sigil {
        b'$' => Self::parse_bulk_string(rest, len),
        b'*' => Self::parse_array(rest, len),
        _ => Err(FrameError::UnknownSigil),
    }
}",
            description: "Returns the frame plus whatever bytes were left over. Incomplete is an error variant rather than an Option because the read loop has to tell \"need more\" from \"malformed\".",
        },
        Snippet {
            title: "The Log Is the Wire Protocol",
            lang: "rust",
            code: r#"impl From<WriteCommand> for Frame {
    fn from(value: WriteCommand) -> Self {
        match value {
            WC::Set { key, value, expires_at } => {
                let mut parts = vec![
                    Frame::BulkString(b"SET".to_vec()),
                    Frame::BulkString(key),
                    Frame::BulkString(value),
                ];
                // PXAT, not EXAT. The millisecond verb is the one the parser
                // reads back without converting. The seconds verb would hand
                // replay a millisecond value and multiply it by 1000 again.
                if let Some(at) = expires_at {
                    parts.push(Frame::BulkString(b"PXAT".to_vec()));
                    parts.push(Frame::BulkString(at.get().to_string().into_bytes()));
                }
                Frame::Array(parts)
            }
            // ...DEL, PEXPIREAT, PERSIST
        }
    }
}"#,
            description: "Mutations are logged as the exact bytes a client would have sent, so replay reuses the inbound parser. Which means the encoder has to pick verbs that read back unchanged. This one nearly shipped wrong.",
        },
        Snippet {
            title: "Pub/Sub Fan-Out",
            lang: "rust",
            code: r"// The push is serialized once, then the bytes go into each subscriber's
// mpsc: the same channel their writer thread already drains to the socket.
pub fn publish(&self, message: Vec<u8>, channel: &[u8]) -> Result<u32, ChannelsError> {
    let mut reached = 0;
    let mut guard = self.channels.lock().map_err(|_| ChannelsError::MutexPoisoned)?;
    if let Some(subs) = guard.get_mut(channel) {
        // retain() fans out and prunes dead sessions in one pass
        subs.retain(|_id, tx| match tx.send(message.clone()) {
            Ok(()) => { reached += 1; true }
            Err(_) => false,
        });
    }
    Ok(reached)
}",
            description: "No thread per subscriber. The session's own writer thread is the subscriber output.",
        },
        Snippet {
            title: "Drain, Then Truncate",
            lang: "rust",
            code: r"pub fn clear(&self) -> Result<(), AofError> {
    let mut guard = self.writer.lock().map_err(|_| AofError::MutexPoisoned)?;
    guard.flush()?;
    guard.get_ref().set_len(0)?;
    Ok(())
}",
            description: "Compaction empties the log once a snapshot holds everything in it. The first version opened a second, non-append handle to truncate and then dropped the old BufWriter, whose pending bytes flushed at the stale cursor into the emptied file. The OS filled the gap with zeros and replay refused the file. Drain first, and never let a second handle exist.",
        },
    ],
    obstacles: &[
        "Self-deadlock on the TTL read path. get_expires_at held the mutex guard and then called self.remove() on the expired branch. std mutexes are not reentrant, so the thread hung forever. Guards live to the end of scope, not the end of the statement",
        "A read-before-parse bug in the frame loop. TCP coalesces writes, so one read can deliver two frames; reading first meant the second sat in the buffer unseen until EOF. Parse first, read only on Incomplete",
        "The server could not restart after the first benchmark run. Compaction truncated the log through a second, non-append handle before draining the old BufWriter, so up to 8 KiB of pending bytes landed at a stale offset in an empty file and the OS filled the gap with zeros: 8,992,628 NULs, then 7,372 bytes of real frames. Replay read byte 0 and refused, correctly. It only fires past 8 KiB of writes between snapshots, so no manual session could have hit it. Fixed, with a regression test that fails on the old body",
    ],
    progress: "Done through M6: RESP, TTLs, snapshot and AOF persistence, graceful shutdown, pub/sub, and SET options on millisecond deadlines. 238 tests. Benchmarked against Redis 8 in September 2026; the first run found a compaction bug and a 49ms accept stall, both fixed. An async migration and MULTI/EXEC are there if I come back to it.",
    impact: "The in-memory half of a pair with chickadee, which is the on-disk LSM engine. Between them they cover both sides of how a KV system gets built. Real clients drive both.",
    site_url: None,
    status: ProjectStatus::Done,
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
    objective: "Build an end-to-end ML inference pipeline in Rust that counts pullups in real time from a webcam, using the MoveNet pose estimation model (https://huggingface.co/qualcomm/Movenet). No cloud inference: everything runs on-device via the tract ONNX runtime (https://github.com/sonos/tract).",
    tags: &["rust", "ml", "computer-vision", "real-time"],
    media: &[MediaItem {
        src: asset!("/assets/projects/upsee/upsee-demo.mp4"),
        alt: "Upsee counting pullups in real time from webcam input",
        caption: Some("Real-time pose estimation and rep counting via on-device MoveNet inference"),
        kind: MediaKind::Video,
    }],
    approach: &[
        "tract is the Rust-native inference path. Three calls take an ONNX file to runnable: model_for_path, into_optimized, into_runnable. No Python, no cloud",
        "A Square trait center-crops webcam frames before the resize. Skipping the distortion measurably improved keypoint confidence",
        "Scores are averaged across four keypoints and frames below 0.4 are skipped, so the counter never acts on a bad read",
        "A hysteresis state machine separates up from down by a dead zone, which is what stops jitter from false-counting",
    ],
    snippets: &[
        Snippet {
            title: "Inference Pipeline",
            lang: "rust",
            code: r"// Load and optimize MoveNet ONNX model
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

// Run inference. Output: [1, 1, 17, 3] (17 keypoints × y,x,confidence)
let result = model.run(tvec!(tensor.into()))?;",
            description: "Three lines to load the model, then per-frame: crop to square, resize, normalize into a tensor, and run inference. tract handles the ONNX graph execution.",
        },
        Snippet {
            title: "Hysteresis State Machine",
            lang: "rust",
            code: r"// Two separate thresholds prevent oscillation:
const UP_THRESHOLD: f32 = 0.05;   // shoulders near wrist level
const DOWN_THRESHOLD: f32 = 0.15;  // shoulders dropped away
// Gap (0.05 to 0.15) = dead zone that absorbs noise

match state {
    Down => if diff < UP_THRESHOLD { state = Up; reps += 1; }
    Up   => if diff > DOWN_THRESHOLD { state = Down; }
}",
            description: "Without hysteresis, noise near the threshold causes rapid state flipping and false counts. The dead zone between thresholds means the signal must move decisively before a transition registers.",
        },
    ],
    obstacles: &[
        "Single threshold caused false counts from keypoint jitter. Hysteresis with separate UP/DOWN thresholds and a dead zone solved it",
        "Quantized MoveNet model (w8a16) is incompatible with tract: the QuantizeLinear op is unsupported. Used the full-precision float model instead",
        "tract documentation is sparse compared to Python ML libraries. Required reading source, ONNX model metadata, and tract examples to get the pipeline working",
    ],
    progress: "Working prototype. Counts pullups in real time from webcam. Roadmap: threshold tuning, temporal smoothing, Raspberry Pi deployment, multi-threaded capture + inference.",
    impact: "ML inference in Rust without Python or cloud dependencies. ~145 lines from webcam frame to rep count.",
    site_url: None,
    status: ProjectStatus::Done,
};

const GOTCHA: Project = Project {
    name: "Gotcha",
    slug: "gotcha",
    headline: "Cross-platform security camera. Input device grabbing, intruder photos, platform-specific I/O.",
    category: "Systems Programming",
    repo_url: "https://github.com/scadoshi/gotcha",
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
    media: &[MediaItem {
        src: asset!("/assets/projects/gotcha/demo.mp4"),
        alt: "Keyboard and mouse input triggering intruder captures",
        caption: Some("Keyboard and mouse input triggering intruder captures"),
        kind: MediaKind::Video,
    }],
    approach: &[
        "cfg(target_os) switches between platform modules, with the platform-only dependencies scoped in Cargo.toml so neither target pulls the other's crates",
        "Linux enumerates /dev/input/event*, filters by capability, grabs each device and polls with nix::poll. macOS goes through rdev and Accessibility callbacks, returning None to swallow the event",
        "Custom traits on third-party types rather than wrappers: Identify on evdev::Device, IsSecret on InputEvent. Extension keeps each platform's quirks behind one interface",
    ],
    snippets: &[
        Snippet {
            title: "Platform Divergence",
            lang: "rust",
            code: r"// Same goal, completely different implementations:
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
// is_probably_mouse()    = REL_X + REL_Y relative axes",
            description: "The same feature requires fundamentally different system APIs on each platform. Conditional compilation keeps both behind a shared interface.",
        },
        Snippet {
            title: "Trait Extensions on Third-Party Types",
            lang: "rust",
            code: r"// Identify trait on evdev::Device: capability-based heuristics
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

// IsSecret trait on evdev::InputEvent: secret key detection
impl IsSecret for InputEvent {
    fn is_secret(&self) -> bool {
        matches!(self.destructure(),
            EventSummary::Key(_, KeyCode::KEY_ESC, 1))
    }
}",
            description: "Custom traits on third-party types. Linux doesn't label devices as 'keyboard' or 'mouse', so you detect them by what they can do. Same pattern for secret key detection: extend the event type rather than match inline.",
        },
    ],
    obstacles: &[
        "rdev grabs every evdev device on Linux, including Bluetooth controllers and network adapters, which disconnects them. Only visible at runtime. Dropped to raw evdev and grab by capability instead",
        "rdev::grab on macOS has no clean stop. The loop blocks forever with nothing to break it, so the secret key calls process::exit(0) and macOS gets no graceful shutdown",
    ],
    progress: "Working on both macOS and Linux. Grabs input, takes timestamped photos, unlocks with secret key. Clean ungrab on Linux, forced exit on macOS.",
    impact: "Systems-level programming across platforms. Drops to raw OS interfaces (evdev, nix::poll) when higher-level libraries don't fit. Custom traits on third-party types, so each platform's quirks stay behind one interface.",
    site_url: None,
    status: ProjectStatus::Done,
};

const RUSTMAS: Project = Project {
    name: "Rustmas",
    slug: "rustmas",
    headline: "Advent of Code tooling in Rust. Fetches inputs, runs solutions, checks them against an independent solver, submits for stars.",
    category: "Developer Tooling",
    repo_url: "https://github.com/scadoshi/rustmas",
    summary: "Advent of Code tooling in Rust. One binary that downloads puzzle inputs, runs your solutions, validates the answers against an independent solver, and submits them.",
    card_bullets: &[
        "Ports and adapters: the domain imports no HTTP, no filesystem, no CLI",
        "Two HTTP clients, since only one of them needs your session cookie",
        "Validated addresses make an out-of-range year or day unrepresentable",
        "~2,180 lines, 72 tests",
    ],
    impact_metric: "~2,180 lines, 72 tests, both service contracts verified live",
    objective: "Build the tooling around Advent of Code rather than just the puzzles: fetch an input, run a day, and know whether the answer is right before spending a submission. Wrong answers to adventofcode.com cost an escalating cooldown, so the tool checks every answer against an independent solver (https://github.com/fornwall/advent-of-code) first and only sends what that solver agrees with.",
    tags: &["rust", "cli", "http", "tooling"],
    media: &[
        MediaItem {
            src: asset!("/assets/projects/rustmas/01-run-a-day.mp4"),
            alt: "Running one day's solution, showing both answers and their timings",
            caption: Some("One day, both parts, and what each one cost. Offline, from cache"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/rustmas/02-independent-solver.mp4"),
            alt: "The same day validated against an independent solver, both parts marked correct",
            caption: Some(
                "Someone else's implementation, used as a regression check. No account needed",
            ),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/rustmas/03-every-year-at-once.mp4"),
            alt: "Every written solution across eleven years running in under half a second",
            caption: Some(
                "Drop the filters and it runs everything, skipping unwritten days before they download",
            ),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/rustmas/04-the-cache.mp4"),
            alt: "The cache on disk: puzzle text, input, and a SHA-256 of the cookie that fetched it",
            caption: Some(
                "The cache keeps the whole puzzle, plus a hash of the cookie that fetched the input",
            ),
            kind: MediaKind::Video,
        },
    ],
    approach: &[
        "The two clients are named for who they talk to, not which one is official. AocClient carries the session cookie and grades each part exactly once. SolverClient needs no account and answers the same question every time, which is what makes it a regression check rather than a one-shot",
        "Dispatch hands back a function pointer instead of calling it. The registry can be asked whether a day exists without holding its input, so a run over every year skips unwritten days before downloading anything for them",
        "Year and Day are newtypes with private fields, and Day::new takes a built Year rather than a raw number, so a day cannot exist without a validated year behind it",
    ],
    snippets: &[
        Snippet {
            title: "The Day Registry",
            lang: "rust",
            code: r"// One arm per day. Returns the solver rather than calling it.
fn solver_for(year: i32, day: i32) -> Option<Solver> {
    Some(match (year, day) {
        (2015, 1) => solve::<year_2015::day_01::Puzzle>,
        (2016, 1) => solve::<year_2016::day_01::Puzzle>,
        _ => return None,
    })
}

// The trait each day implements. Sized on purpose: new returns Self,
// so it could never go through a vtable, and the match above already
// knows every concrete type.
pub trait Solution: Sized {
    fn new(input: impl AsRef<str>) -> anyhow::Result<Self>;
    fn part_one(&self) -> anyhow::Result<Answer>;
    fn part_two(&self) -> anyhow::Result<Answer>;
}",
            description: "Handing back a function pointer means the registry can answer \"is this day written\" without an input in hand, so a run over every year downloads nothing for days nobody has solved.",
        },
        Snippet {
            title: "Merging the Two Verdicts",
            lang: "rust",
            code: r#"// AOC's word supersedes the solver's, so a starred part reads as
// starred rather than repeating that the solver agreed.
let notes: String = match (&self.solver_verdict, &self.aoc_verdict) {
    (_, Some(AocVerdict::Correct)) => "new star".to_string(),
    (_, Some(AocVerdict::AlreadySolved)) => "starred".to_string(),
    (Some(v), Some(s)) => format!("{}, {}", v, s),
    (Some(v), None) => v.to_string(),
    (None, Some(s)) => s.to_string(),
    (None, None) => String::new(),
};

// year 2015 day 1 in 12.707us (3.291us parsing)
//   part one: 138 (correct) [7.125us]
//   part two: 1771 (new star) [2.291us]"#,
            description: "AlreadySolved reads like a rejection but it is the site confirming the star exists, which is why it renders as starred rather than as a complaint.",
        },
    ],
    obstacles: &[
        "Swapping the session cookie silently invalidated every cached input. 2015 day 1 answered 280 one day and 138 the next, and only the changed answers gave it away. Inputs now carry a SHA-256 of the cookie that fetched them",
        "A local cache of confirmed answers was designed in detail and then dropped. The argument for it was that AOC grades each part exactly once, so a cache looked like the only durable record. The site is stateful and reports AlreadySolved, so the fact called irreplaceable was always one request away",
    ],
    progress: "Feature complete. fetch, solve, --validate and --submit all work against both services, 72 tests pass, and both service contracts are recorded in context/references.md from live probing rather than guesswork. Day one of every year except 2019 is solved. Next are the day twos.",
    impact: "A finished tool with its reasoning written down, including the options that were rejected and why. The design notes are what made the C# rebuild (Sharpmas) a language exercise rather than a redesign.",
    site_url: None,
    status: ProjectStatus::Done,
};

const SHARPMAS: Project = Project {
    name: "Sharpmas",
    slug: "sharpmas",
    headline: "Advent of Code tooling in C#, rebuilt from the Rust original down to the output format.",
    category: "Cross-Language Port",
    repo_url: "https://github.com/scadoshi/sharpmas",
    summary: "The same Advent of Code tooling, rebuilt in C#. The design was already settled, so every decision left was a question about the language.",
    card_bullets: &[
        "Static abstract interface members where Rust has associated functions",
        "Closed record hierarchies stand in for Rust enums",
        "AnswerResult carries a failure, since C# has no Result",
        "~2,270 lines, 121 tests",
    ],
    impact_metric: "~2,270 lines, 121 tests, one design across two languages",
    objective: "Learn C# by rebuilding a finished Rust tool rather than by reading about it. Rustmas (https://github.com/scadoshi/rustmas) already settled what the tool should do and recorded why, so nothing here is a design question. Every open question is a language question: what is the C# idiom for this, and where is there honestly no analogue.",
    tags: &["csharp", "dotnet", "cli", "port"],
    media: &[
        MediaItem {
            src: asset!("/assets/projects/sharpmas/01-run-a-day.mp4"),
            alt: "The C# port running one day's solution with the same output shape as rustmas",
            caption: Some(
                "Same subcommands, same filters, same output. Different language under it",
            ),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/sharpmas/02-independent-solver.mp4"),
            alt: "Answers validated against the independent solver, both parts marked correct",
            caption: Some("The same check against the same independent solver"),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/sharpmas/03-sum-types-in-csharp.mp4"),
            alt: "An abstract record with a private constructor and sealed nested cases",
            caption: Some(
                "What C# has instead of an enum: a private constructor closing the set of cases",
            ),
            kind: MediaKind::Video,
        },
        MediaItem {
            src: asset!("/assets/projects/sharpmas/04-same-tool-two-languages.mp4"),
            alt: "rustmas and sharpmas side by side running the same command on the same puzzle",
            caption: Some(
                "Side by side on the same puzzle. Same answers, same verdicts, honest timings",
            ),
            kind: MediaKind::Video,
        },
    ],
    approach: &[
        "ISolution<TSelf> with a static abstract Parse. Rust's trait has an associated function returning Self, and C#'s static abstract interface members are the nearest thing. A static member has nothing to dispatch on, so the runner is generic for that reason alone",
        "Where Rust has an enum, C# gets an abstract record with sealed nested leaves and a private base constructor, since only a nested type can reach a private constructor. That is as near as C# gets to a sum type nothing outside can extend",
        "AnswerResult stands in for Result. A failure is held rather than thrown out of the run, so one broken part does not hide the other's answer",
    ],
    snippets: &[
        Snippet {
            title: "The Same Contract in C#",
            lang: "csharp",
            code: r"// Rust: an associated function returning Self.
//   fn new(input: impl AsRef<str>) -> anyhow::Result<Self>;
//
// C#: a static abstract interface member, reachable only through a
// type parameter, since a static member has no receiver.
public interface ISolution<TSelf>
    where TSelf : ISolution<TSelf>
{
    public static abstract TSelf Parse(string input);
    public Answer PartOne();
    public Answer PartTwo();
}

// Which is why the runner is generic rather than taking an interface.
public static async Task<Solved> Solve<T>(
    SolverClient client, bool validate, string input, Day day
) where T : ISolution<T>
{
    var solution = T.Parse(input);   // only reachable via T
    ...
}",
            description: "The closest C# gets to Rust's trait. The consequence is structural: nothing can hold an ISolution and call Parse on it, so every caller down to the registry has to know the concrete type, exactly as the Rust side does through monomorphized generics.",
        },
        Snippet {
            title: "Closing the Set of Cases",
            lang: "csharp",
            code: r#"public abstract record Answer
{
    // Private, so the set of cases is closed: only nested types
    // can reach it.
    private Answer() { }

    public sealed record Value(string Data) : Answer;   // submittable
    public sealed record Visual(string Art) : Answer;   // art, not an answer
    public sealed record None : Answer;                 // day 25 part two
    public sealed record Unwritten : Answer;            // nobody wrote it yet

    public sealed override string ToString() => this switch
    {
        Value(string data) => $"{data}",
        Visual(string art) => $"\n{art}\n",
        None => "(none)",
        Unwritten => "(unwritten)",
        // Rust's compiler proves this unreachable. C#'s does not.
        _ => throw new UnreachableException($"unhandled: {GetType().Name}"),
    };
}"#,
            description: "The private constructor closes the hierarchy so no case can be added outside this file. The last arm is the honest part: C# cannot prove the switch exhaustive, so the impossible case still gets written down.",
        },
    ],
    obstacles: &[
        "Enumerable.Range takes a count, not an end. It cost two bugs while porting the day filter, both caught by tests mirrored from rustmas before anything ran, and one of them had already been recorded weeks earlier in the same repo's journal",
        "Porting a settled design makes it easy to transliterate Rust into C# that compiles and reads badly. The guard messages are the example that stuck: matching Rust's phrasing would have meant fighting the analyzer for a worse message than the framework already produces",
    ],
    progress: "The tool is finished and matches rustmas feature for feature, with 121 tests passing and no build warnings. The last catch-up landed on 2026-08-23: the eager Filter type, Answer.Unwritten, the day 25 gate, and all four hierarchies closed. Two days are solved so far, 2015 day 1 and 2016 day 1, with every answer confirmed by the solver and matching rustmas. What is left is solutions and the shared helpers they will want.",
    impact: "A cross-language port carried end to end, with both sides public and comparable file by file. The design was fixed going in, so what the repo records is where the two languages actually diverge, and where C# has no good answer at all.",
    site_url: None,
    status: ProjectStatus::Done,
};

#[cfg(test)]
mod tests {
    use super::{featured_projects, side_quests};
    // `static_routes` is a `Routable` method; without the trait in scope the
    // test does not compile.
    use dioxus::prelude::Routable as _;

    /// public/sitemap.xml is hand-maintained, so it drifts. It already has:
    /// cairn landed in 3c17f1c and never made it into the file. Compare it
    /// against the routes SSG actually renders.
    ///
    /// `/404` is the one route SSG renders that belongs nowhere near here: it
    /// is prerendered for deploy.yml to ship as 404.html, and it carries
    /// noindex. `static_routes()` in main.rs appends it, this does not.
    #[test]
    fn sitemap_lists_every_prerendered_route() {
        let sitemap = include_str!("../public/sitemap.xml");
        let mut listed: Vec<&str> = sitemap
            .split("<loc>https://scottyfermo.com")
            .skip(1)
            .filter_map(|s| s.split_once("</loc>").map(|(path, _)| path))
            .collect();
        let mut expected: Vec<String> = crate::Route::static_routes()
            .iter()
            .map(ToString::to_string)
            .chain(
                featured_projects()
                    .iter()
                    .map(|p| format!("/projects/{}", p.slug)),
            )
            .chain(
                side_quests()
                    .iter()
                    .map(|p| format!("/side-quests/{}", p.slug)),
            )
            .collect();
        listed.sort_unstable();
        expected.sort_unstable();
        assert_eq!(listed, expected);
    }
}
