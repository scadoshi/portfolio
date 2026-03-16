#[derive(Clone, Copy, PartialEq)]
pub enum ProjectType {
    Featured,
    SideQuest,
}

pub struct Project {
    pub name: &'static str,
    pub slug: &'static str,
    pub project_type: ProjectType,
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
    &[MARVIN, NIGHTHAWK, UPSEE, CAPTURE]
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
    project_type: ProjectType::Featured,
    headline: "Full-stack MTG deck builder. Axum backend, Dioxus frontend, PostgreSQL, 100k+ cards.",
    category: "Full-Stack Application",
    repo_url: "https://github.com/scadoshi/zwipe",
    summary: "A mobile-first Magic: The Gathering deck builder with swipe-based navigation. Three binaries in a Cargo workspace: zerver (Axum REST API), zwiper (Dioxus cross-platform app), and zervice (background task runner). The frontend imports the backend as a library dependency for shared domain types.",
    impact_metric: "~24,500 lines of production Rust",
    impact_detail: "Hexagonal architecture with strict type safety throughout. Every domain boundary enforced at the type level. Production-strict linting: .unwrap(), .expect(), panic!, todo!, dbg!, and print! are all denied at compile time. 33 enforced Clippy rules.",
    objective: "Build a full-stack MTG deck builder with swipe-based navigation, targeting web, iOS, Android, and desktop from a single Rust codebase. Three workspace binaries: zerver (Axum REST API), zwiper (Dioxus frontend), and zervice (background service for card sync and session cleanup).",
    approach: &[
        "Hexagonal architecture applied consistently across ~24,500 lines of Rust. Port traits define what operations are needed (AuthRepository, CardRepository, DeckRepository). Adapters implement those ports for specific technologies. Domain logic has zero external dependencies",
        "Domain-driven design with validated newtypes: Username (3-20 chars, profanity filter), Password (8-128 chars, uppercase/lowercase/digit/symbol required, max 3 consecutive repeats, checked against common password dictionary), EmailAddress, UserId, DeckId, JwtSecret. Invalid data is unrepresentable",
        "Structured error chain: SQLx errors → PostgreSQL constraint violation detection (unique=23505, check=23514) → domain-specific error enums (RegisterUserError::Duplicate) → HTTP status codes (409 Conflict). Internal details logged but never exposed to clients",
        "JWT access tokens (HS256, 24-hour expiry) + rotating refresh tokens (max 5 per user, SHA-256 hashed, 14-day expiry). Old refresh token deleted on use, preventing replay attacks. Session limits auto-enforced by background service",
        "Argon2id password hashing with OS-random salts (resistant to GPU/ASIC attacks). Common password blocklist with 170+ patterns following NIST guidelines",
        "PostgreSQL with compile-time verified SQLx queries: 7 migrations, JSONB operators (@> contains, <@ contained by, ?| has any key), dynamic query building for card search with 10+ filter criteria",
        "Background service binary (zervice): hourly Scryfall delta sync handling 100k+ cards in batches of 327 (respecting PostgreSQL's 65k parameter limit), expired refresh token cleanup, max session enforcement",
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
            title: "Partial Updates with Option<Option<T>>",
            code: r#"// None        = field not provided (don't change)
// Some(None)  = explicitly set to null
// Some(Some(v)) = update to new value
pub struct UpdateDeck {
    pub name: Option<Option<String>>,
    pub description: Option<Option<String>>,
}"#,
            description: "Distinguishing 'not provided' from 'set to null' in partial update operations. A pattern that becomes essential when building real APIs.",
        },
        Snippet {
            title: "JSONB Card Search",
            code: r#"// Advanced card search with dual color identity modes
// @> = contains (cards with AT LEAST these colors)
// <@ = contained by (cards with ONLY these colors)
query_builder.push(" AND c.color_identity @> ");
query_builder.push_bind(color_json);

// ?| = has any of these keys (for legality filtering)
query_builder.push(" AND c.legalities ?| ");
query_builder.push_bind(format_keys);"#,
            description: "PostgreSQL JSONB operators for flexible card filtering. Supports both 'at least these colors' and 'only these colors' search modes.",
        },
    ],
    obstacles: &[
        "PostgreSQL parameter limits required batching card upserts at ~327 cards per batch (65,535 max params / ~200 fields per card)",
        "Scryfall API rate limiting and delta sync required careful orchestration with batch processing and sync metrics tracking",
        "Full documentation pass with #![warn(missing_docs)] resolved 243 warnings across the codebase",
        "Clippy configured with 33 enforced lints including strict denies on unwrap, expect, panic, todo, and dbg_macro",
    ],
    progress: "Auth, card database, deck management, and card search complete. Working on deck card browser with full-screen swipeable card viewer.",
    impact: "Demonstrates complete full-stack capability in Rust: database migrations, JWT auth with refresh token rotation, reactive frontend, background services, all in one language with shared domain types between frontend and backend.",
};

const HALO_ACTION_IMPORTER: Project = Project {
    name: "Halo Action Importer",
    slug: "halo-action-importer",
    project_type: ProjectType::Featured,
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
    project_type: ProjectType::Featured,
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
    project_type: ProjectType::SideQuest,
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
    project_type: ProjectType::SideQuest,
    headline: "LSM-tree storage engine from scratch. WAL, memtable, SSTables, k-way compaction.",
    category: "Database Internals",
    repo_url: "https://github.com/scadoshi/nighthawk",
    summary: "A log-structured storage engine built phase by phase. Started from the Bitcask paper as a simple append-only log, then evolved into a full LSM-tree: BTreeMap memtable, WAL-backed durability, SSTable flush, and k-way merge compaction. The architecture behind LevelDB, RocksDB, and Cassandra. ~800 lines of engine code with 81 tests covering every layer.",
    impact_metric: "~800 lines of engine, 81 tests",
    impact_detail: "Started as a Bitcask implementation and grew into a full LSM-tree storage engine. Each phase added a real layer of database behavior. By Phase 4, it had the same core architecture as the storage engines used in production systems like RocksDB.",
    objective: "Build a key-value storage engine incrementally, starting from the Bitcask paper (https://riak.com/assets/bitcask-intro.pdf) and evolving toward the LSM-tree architecture that powers LevelDB, RocksDB, and Cassandra. Each phase adds a concrete layer of real database behavior: durability, binary formats, sorted flush, and merge compaction.",
    approach: &[
        "Phases 1-3 (Bitcask foundation): append-only WAL with in-memory index, sync_all() durability, log compaction via atomic rename, custom binary format with 10-byte headers (magic bytes 0x4E48, CRC32 checksum, length prefix via wincode serialization), and byte-by-byte corruption recovery with a typed CorruptionType enum",
        "Phase 4 Step 1 (Memtable): upgraded index from HashMap<String, u64> offsets to BTreeMap<String, Entry> values. Sorted order is required for SSTable flush. MemTable::process() unifies insert/remove and tracks byte-level size. WAL is replayed into the memtable on startup",
        "Phase 4 Step 2 (SSTable flush): when memtable exceeds 4MB, all entries are written in sorted order to data/sstables/{timestamp:020}.sst. Microsecond Unix timestamp in the filename means lexicographic order equals chronological order. After flush: sync_all(), truncate WAL, clear memtable",
        "Phase 4 Step 3 (Read path): check memtable first (BTreeMap lookup, no disk I/O). On miss, scan SSTables newest-to-oldest using the existing binary header reader. First match wins. Recency is always truth",
        "Phase 4 Step 4 (K-way compaction): k-way merge across all SSTable files. Each iteration finds the global minimum key, the newest file wins on duplicates, tombstones are dropped. Triggers every 10 flushes via flush_count on Log. Original SSTables deleted after compacted output is written",
        "81 tests covering every layer: command parser, entry serialization, corruption recovery, memtable operations, WAL rebuild on startup, SSTable flush and read path, and compaction correctness including tombstone handling and duplicate key resolution",
    ],
    snippets: &[
        Snippet {
            title: "LSM-Tree Architecture",
            code: r#"// Write path:
// set "foo" "bar"
//   → append to WAL (crash safe, sync'd to disk)
//   → insert into memtable (BTreeMap<String, Entry>)
//   → if memtable.size() > 4MB: flush to SSTable
//   → every 10 flushes: k-way merge compaction

// Read path:
// get "foo"
//   → check memtable (BTreeMap::get, O(log n), no disk I/O)
//   → scan SSTables newest-to-oldest (recency = truth)

// On disk:
// data/wal                       — append-only WAL, replayed on startup
// data/sstables/00000000000000…  — sorted, timestamped SSTable files"#,
            description: "WAL provides crash safety. Memtable absorbs writes in sorted order. SSTables are immutable sorted segments. Compaction merges them back down. Each layer has one job.",
        },
        Snippet {
            title: "K-Way Merge: compact()",
            code: r#"pub fn compact(&mut self) -> anyhow::Result<()> {
    let mut entries: Vec<_> = read_dir(&self.sstables_path)?.flatten().collect();
    entries.sort_by_key(|e| Reverse(e.file_name())); // newest-to-oldest
    let to_delete: Vec<_> = entries.iter().map(|e| e.path()).collect();

    // Pair each file with its current entry cursor
    let mut files: Vec<_> = entries
        .into_iter()
        .filter_map(|e| OpenOptions::new().read(true).open(e.path()).ok())
        .map(|f| (None::<Entry>, f))
        .collect();
    for (entry, file) in files.iter_mut() {
        *entry = file.read_next_entry_with_header()?;
    }

    let mut memtable = MemTable::new();
    loop {
        files.retain(|(entry, _)| entry.is_some());
        if files.is_empty() { break; }

        // Find global minimum key across all active cursors
        let min = {
            let mut min = None::<String>;
            for (entry, _) in files.iter() {
                let curr = entry.as_ref().unwrap();
                if min.as_ref().is_none_or(|m| curr.key().cmp(m).is_lt()) {
                    min = Some(curr.key().to_owned());
                }
            }
            min.unwrap()
        };

        // First file (newest) with this key wins; all participants advance
        for (entry, file) in files.iter_mut() {
            let is_participant = entry.as_ref().unwrap().key() == min;
            let winner_found = memtable.contains_key(&min);
            if is_participant && !winner_found {
                memtable.process(entry.as_ref().unwrap())?;
            }
            if is_participant {
                *entry = file.read_next_entry_with_header()?;
            }
        }
        if memtable.should_flush() {
            memtable.flush_to(self.sstables_path.clone())?;
        }
    }
    if !memtable.is_empty() {
        memtable.flush_to(self.sstables_path.clone())?;
    }
    for path in to_delete { remove_file(path)?; }
    Ok(())
}"#,
            description: "Files sorted newest-to-oldest means the first file with a given key is always the winner. All participants advance their cursor on each key so no entry is ever skipped. Tombstones are implicitly dropped: MemTable::process() removes keys on Delete, so they never appear in the flushed output.",
        },
        Snippet {
            title: "TempDir Lifetime in Tests",
            code: r#"// Every test that touches the filesystem uses this helper:
fn temp_log() -> (TempDir, Log) {
    let dir = tempfile::tempdir().unwrap();
    let log = Log::new(
        dir.path(),
        dir.path().join("memtable"),
        dir.path().join("sstables"),
        true,
    ).unwrap();
    (dir, log) // TempDir returned — caller keeps directory alive
}

// Usage in every test:
let (_dir, mut log) = temp_log();
//   ^^^^
// Binding _dir keeps TempDir alive for the test scope.
// let (_, mut log) = ... drops TempDir immediately,
// deleting the directory before the test body runs."#,
            description: "TempDir implements Drop — when it goes out of scope the temp directory is deleted. Discarding it with _ drops it at the statement. Binding it as _dir keeps it alive until end of scope. This came up when adding filesystem tests: the first version wiped the directory the test was about to use.",
        },
    ],
    obstacles: &[
        "Compaction correctness: tombstones must be dropped during merge (not written to output), and duplicate keys must resolve to the newest source file. Getting this wrong silently resurrects deleted keys with no error at runtime",
        "TempDir lifetime in tests: creating a Log inside a temp dir would drop the TempDir immediately, deleting the directory before the test ran. Fixed by returning (TempDir, Log) tuples from all test helpers to keep the directory alive",
        "flush_count must be initialized from the existing SSTable count on startup, not zero. Without this, the compaction trigger fires on the wrong schedule after a process restart",
        "Upgrading from HashMap to BTreeMap required threading sorted order through the entire flush path. The payoff: BTreeMap iterates in key order automatically, which is what makes SSTable files cheaply sortable without a separate pass",
    ],
    progress: "Phases 1-4 complete. WAL, memtable, SSTable flush and read path, and k-way compaction all working with 81 tests. Next: bloom filters (probabilistic skip for SSTable reads) and leveled compaction (L0/L1 tiers to control read/write amplification).",
    impact: "Started from a paper and evolved into the storage architecture behind LevelDB, RocksDB, and Cassandra. Not just reading about LSM-trees but building one from scratch: WAL semantics, sorted flush, merge compaction, and the test coverage to verify it all holds.",
};

const UPSEE: Project = Project {
    name: "Upsee",
    slug: "upsee",
    project_type: ProjectType::SideQuest,
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
    project_type: ProjectType::SideQuest,
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
