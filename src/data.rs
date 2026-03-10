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

pub struct SideQuest {
    pub name: &'static str,
    pub category: &'static str,
    pub repo_url: &'static str,
    pub description: &'static str,
    pub highlights: &'static [&'static str],
    pub snippet_title: &'static str,
    pub snippet_code: &'static str,
}

pub fn featured_projects() -> &'static [Project] {
    &[ZWIPE, HALO_ACTION_IMPORTER, HALO_CUSTOM_FIELD_BUILDER]
}

pub fn side_quests() -> &'static [SideQuest] {
    &[MARVIN, NIGHTHAWK, UPSEE, CAPTURE]
}

pub fn find_project(slug: &str) -> Option<&'static Project> {
    featured_projects().iter().find(|p| p.slug == slug)
}

const ZWIPE: Project = Project {
    name: "Zwipe",
    slug: "zwipe",
    headline: "Full-stack MTG deck builder. Axum backend, Dioxus frontend, PostgreSQL, 35k+ cards.",
    category: "Full-Stack Application",
    repo_url: "https://github.com/scadoshi/zwipe",
    summary: "A mobile-first Magic: The Gathering deck builder with swipe-based navigation. Two Rust crates in a workspace: an Axum REST API and a Dioxus cross-platform app.",
    impact_metric: "24,500 lines of production Rust",
    impact_detail: "Enterprise-grade hexagonal architecture. Strictly type-safe with newtype patterns throughout. Designed to be clean, extensible, and maintainable at scale. Every domain boundary enforced at the type level.",
    objective: "Build a full-stack MTG deck builder with swipe-based navigation, targeting web, iOS, Android, and desktop from a single Rust codebase. Two workspace crates: zerver (Axum REST API) and zwiper (Dioxus frontend).",
    approach: &[
        "Hexagonal architecture applied consistently across ~24,500 lines of Rust",
        "Domain-driven design with newtypes for type safety (UserId, DeckId, JwtSecret, all validated at construction)",
        "JWT + rotating refresh tokens (max 5 per user, SHA-256 hashed, 14-day expiry)",
        "Argon2id password hashing with common password blocklist (170+ patterns, NIST guidelines)",
        "PostgreSQL with SQLx: 7 migrations, JSONB operators, window functions, composite constraints",
        "Background job binary (zervice) for Scryfall delta sync handling 35k+ cards in batches",
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
        "PostgreSQL parameter limits required batching card upserts at ~327 cards per batch",
        "Scryfall API rate limiting and delta sync required careful orchestration with batch processing",
        "Full documentation pass with #![warn(missing_docs)] resolved 243 warnings across the codebase",
        "Clippy configured with 26 enforced lints for consistent code quality",
    ],
    progress: "Auth, card database, deck management, and card search complete. Working on deck card browser with full-screen swipeable card viewer.",
    impact: "Demonstrates complete full-stack capability in Rust: database migrations, JWT auth, reactive frontend, all in one language with shared domain types.",
};

const HALO_ACTION_IMPORTER: Project = Project {
    name: "Halo Action Importer",
    slug: "halo-action-importer",
    headline: "Production bulk import tool. Millions of records, resilient retry, incremental caching.",
    category: "Production Data Tooling",
    repo_url: "https://github.com/scadoshi/halo_action_importer",
    summary: "CLI tool for bulk importing actions into the Halo Software suite from CSV and Excel. Built for real migrations involving millions of records against production APIs with real failure modes.",
    impact_metric: "Weeks of manual work, automated",
    impact_detail: "Halo's built-in browser import runs one entry at a time, loses progress on page refresh, and offers minimal error handling. For millions of entries that's literal weeks of runtime coordinating spreadsheets manually. This tool: dump everything into one file, run it, and forget it. Split files across directories for parallel execution when you need faster runtimes.",
    objective: "Build a CLI tool for bulk importing actions into Halo Software products from CSV and Excel files. Must handle millions of records against a production API with real failure modes: network errors, token expiry, missing tickets, and partial batch failures.",
    approach: &[
        "Production-grade error recovery: infinite retry on network/timeout failures, automatic token refresh on 401s",
        "Ticket-grouped retry logic: when a batch fails, split by ticket_id and retry each group independently",
        "Incremental caching with file locking (fs2): tracks fetched report IDs and imported action IDs, survives process restarts",
        "Structured output per run: log/YYYY-MM-DD_HH-MM-SS/ directory with full.log, retry.csv, and summary.json",
        "CLI with practical flags: --batch-size, --only-parse (validation mode), --only-cache, --input-path",
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
    ],
    obstacles: &[
        "Binary search retry was the wrong abstraction for batch failures. Replaced with ticket-grouped retry that's both simpler and more efficient",
        "File locking for concurrent cache writes: hit corruption bugs, fixed properly with fs2",
        "Building software that runs unattended for hours against unreliable APIs required thinking through every failure mode",
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
    summary: "CLI tool that reads custom field definitions from CSV and creates them across Halo Software products via API. Supports all 8 field types, OAuth 2.0, rate limiting, and interactive debug mode.",
    impact_metric: "Manual UI clicks to one CSV import",
    impact_detail: "Instead of building configuration manually through Halo's UI one field at a time, prepare a CSV and import. ~1000 fields in about 15 minutes on a single thread. Great workflow: gather client requirements, use AI to generate a CSV to spec, then import. Building forms becomes trivially fast as long as the fields fall within the tool's scope.",
    objective: "Build a CLI tool that reads custom field definitions from CSV files and creates them in Halo Software products via the API. Must support all 8 field types, handle authentication, respect rate limits, and distribute as cross-platform binaries.",
    approach: &[
        "Domain modeling with validation: Name, Label, and FieldType all validated at construction via TryFrom",
        "Two-layer serialization: CustomField (domain) maps to HttpCustomField (API representation)",
        "OAuth 2.0 client credentials flow with automatic token refresh and 30-second expiry buffer",
        "Rate limiting (500ms between requests) to stay under Halo's 700/5min API limit",
        "Interactive debug mode: field-by-field review with skip/process/quit",
        "GitHub Actions CI/CD: cross-platform builds, distribution packaging, tagged v1.0.0 release",
    ],
    snippets: &[
        Snippet {
            title: "Field Type System",
            code: r#"FieldType::Text(TextInputType)                  // 7 input variants
FieldType::SingleSelect(SingleSelectInputType)  // 3 input variants
FieldType::Date(DateInputType)                  // 2 input variants
FieldType::Memo                                 // no sub-types
FieldType::MultiSelect
FieldType::Time
FieldType::Checkbox
FieldType::Rich"#,
            description: "Type-safe enum covering all 8 Halo field types. Each variant's input type sub-enum is validated at parse time. Invalid combinations are caught before any API call.",
        },
        Snippet {
            title: "Two-Layer Serialization",
            code: r#"// Domain type: validated, type-safe
struct CustomField {
    name: Name,         // max 64, alphanumeric + underscore
    label: Label,       // max 256, visible chars only
    field_type: FieldType,
}

// API type: matches Halo's expected JSON shape
struct HttpCustomField {
    name: String,
    label: String,
    type_id: i32,
    input_type_id: Option<i32>,
}

impl From<CustomField> for HttpCustomField { ... }"#,
            description: "Clean separation between domain validation and API serialization. The domain type guarantees correctness; the HTTP type handles the wire format.",
        },
    ],
    obstacles: &[
        "Cross-platform binary distribution required GitHub Actions CI/CD with separate build targets for Windows, macOS (Intel + ARM), and Linux",
        "Rate limiting as a design consideration: 500ms delay between requests to stay under Halo's 700/5min limit",
        "CSV validation with useful error messages: row numbers, specific field issues, and optional field handling",
    ],
    progress: "Shipped. Tagged v1.0.0 with cross-platform releases via GitHub Actions.",
    impact: "Reduced enterprise configuration time from hours to minutes. Deployed across Fortune 500 client implementations. ~727 lines of code demonstrating the same layered architecture used at larger scale in Zwipe.",
};

const MARVIN: SideQuest = SideQuest {
    name: "Marvin",
    category: "AI Tooling",
    repo_url: "https://github.com/scadoshi/marvin",
    description: "Interactive CLI chatbot built on the Rig framework with Claude as the backend. Streaming responses, tool use (math and web search via Tavily), chat persistence, token tracking, context compaction, and dynamic model switching. ~1,750 LOC.",
    highlights: &[
        "AI agent plumbing in Rust: tool use with automatic JSON Schema generation via schemars",
        "4 Tavily web tools sharing an Arc<TavilyClient> for concurrent access",
        "Context management: /compact replaces history with AI-generated summary to stay within limits",
        "Contributed back to Rig framework (fixed deprecated model constants, opened PR)",
    ],
    snippet_title: "Tool Architecture",
    snippet_code: r#"// Each tool uses schemars for automatic JSON Schema generation
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
};

const NIGHTHAWK: SideQuest = SideQuest {
    name: "Nighthawk",
    category: "Database Internals",
    repo_url: "https://github.com/scadoshi/nighthawk",
    description: "A key-value store modeled after the Bitcask paper. Append-only log with in-memory HashMap index, binary serialization with custom headers (magic bytes, CRC32, length-prefixed entries), log compaction via atomic rename, and byte-by-byte corruption recovery.",
    highlights: &[
        "Custom binary format: magic bytes, CRC32 checksums, length-prefixed entries",
        "Corruption recovery: byte-by-byte scanning to find next valid entry",
        "Crash safety: sync_all() after every write, atomic rename for compaction",
        "Trait-based design: Header trait on File, Index trait on HashMap, Execute trait on Log",
    ],
    snippet_title: "On-Disk Binary Format",
    snippet_code: r#"// Each entry on disk:
[magic: 0x4E48] [crc32: 4 bytes] [entry_len: 4 bytes] [wincode Entry]
     "NH"        integrity check    length prefix        serialized data

// Corruption recovery:
// If magic bytes don't match or CRC fails,
// scan forward byte-by-byte until next valid header.
// Distinguishes: HeaderNotFound, MagicBytesNotFound,
//   ChecksumMismatch, EntryParseError"#,
};

const UPSEE: SideQuest = SideQuest {
    name: "Upsee",
    category: "ML Inference",
    repo_url: "https://github.com/scadoshi/upsee",
    description: "Real-time pullup counter using webcam + MoveNet pose estimation model via tract ONNX runtime. Runs entirely on-device with no cloud inference. End-to-end ML inference pipeline in Rust.",
    highlights: &[
        "Full pipeline: frame capture, preprocessing, inference, postprocessing, state machine",
        "Tensor manipulation: reshaping webcam frames into [1, 3, 192, 192] NCHW tensors",
        "Hysteresis-based state machine prevents noise-induced false counts",
        "tract as a Rust-native alternative to Python inference runtimes",
    ],
    snippet_title: "Hysteresis State Machine",
    snippet_code: r#"// Two separate thresholds prevent oscillation:
// Transition to UP:   shoulder-wrist diff < 0.05 (arms pulled high)
// Transition to DOWN: shoulder-wrist diff > 0.15 (arms extended)
// Gap = dead zone that absorbs noise

[Webcam] > [Square crop] > [Resize 192x192] > [Normalize 0-1]
    > [MoveNet inference] > [Keypoint extraction]
    > [State machine: DOWN / UP] > [Rep counter]"#,
};

const CAPTURE: SideQuest = SideQuest {
    name: "Capture",
    category: "Systems Programming",
    repo_url: "https://github.com/scadoshi/capture",
    description: "Cross-platform security camera that grabs all input devices, snaps intruder photos on any interaction. Press a secret key to unlock. Works on macOS and Linux with completely different I/O strategies per platform.",
    highlights: &[
        "Conditional compilation (cfg(target_os)) for platform-specific I/O strategies",
        "Linux: raw evdev device enumeration, capability-based filtering, nix::poll for multiplexed I/O",
        "macOS: rdev callback-based grab via Accessibility API",
        "Bug discovery: rdev grabs ALL evdev devices (including Bluetooth controllers), dropped to raw evdev with selective grabbing",
    ],
    snippet_title: "Platform Divergence",
    snippet_code: r#"// Same goal, completely different implementations:
//
// | Concern        | macOS                     | Linux                      |
// |----------------|---------------------------|----------------------------|
// | Grab mechanism | rdev::grab (Accessibility)| evdev device.grab() each   |
// | Event loop     | Callback-based            | nix::poll across FDs       |
// | Permissions    | Accessibility API approval| input group membership     |
// | Shutdown       | process::exit(0)          | device.ungrab() on all     |
//
// Capability-based heuristics for device identification:
// is_probably_keyboard() = EV_REPEAT + KEY_A + KEY_ENTER
// is_probably_mouse()    = REL_X + REL_Y relative axes"#,
};
