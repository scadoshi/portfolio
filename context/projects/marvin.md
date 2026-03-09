# Marvin

## Headline

CLI chatbot built on Rig — streaming responses, tool use, web search, context management.

## Category

AI Tooling

## What It Is

An interactive CLI chatbot built on the Rig framework (Rust AI framework) with Claude as the backend. Supports streaming responses, tool use (math and web search via Tavily), chat persistence, token tracking, context compaction, and dynamic model switching. ~1,750 LOC.

## What It Proves

- AI agent plumbing in Rust: tool use with automatic JSON Schema generation via schemars
- Streaming responses with cumulative token tracking (input + output)
- Dynamic model discovery: fetches available Claude models from the Anthropic API at startup
- 4 Tavily web tools (search, extract, crawl, sitemap) sharing an Arc<TavilyClient>
- Context management: /compact replaces history with AI-generated summary to stay within context limits
- Chat persistence: /save writes to JSON, /import loads previous sessions, auto-incrementing session IDs
- Trait-based command pattern: each slash command is a trait impl on Chat, routed via ChatInput enum
- Contributed back to Rig framework (fixed deprecated model constants, opened PR)

## Key Technical Highlights

### Tool Architecture
```rust
// Each tool uses schemars for automatic JSON Schema generation
#[derive(JsonSchema, Deserialize)]
struct SearchArgs {
    query: String,
    topic: Option<String>,
    search_depth: Option<String>,
    max_results: Option<u32>,
    // ...
}

// Tools share HTTP client via Arc
impl Tool for SearchWeb {
    async fn call(&self, args: SearchArgs) -> Result<String, ToolError> {
        self.client.search(args).await  // Arc<TavilyClient>
    }
}
```

### Context Compaction
When conversation history grows too long, /compact asks the agent to summarize the entire conversation into a condensed context message, then replaces the history with just that summary. Preserves continuity while freeing token budget.

### Command Routing
```
ChatInput::Message(text)     → stream to agent, accumulate tokens
ChatInput::Help              → show commands
ChatInput::History           → last 10 messages (truncated at 300 chars)
ChatInput::Tokens            → cumulative usage with comma formatting
ChatInput::Model             → switch Claude model (rebuilds agent)
ChatInput::Compact           → condense history via AI summary
ChatInput::Save              → persist to chats/<id>.json
ChatInput::Import(id)        → load previous session
ChatInput::Summarize         → ask agent for conversation summary
ChatInput::Clear             → reset history
ChatInput::Exit              → save and quit
```

## What I Learned

- How Rig wires up tool use with JSON Schema (schemars generates schemas from Rust types at compile time)
- Streaming vs non-streaming agent responses and how token usage is extracted from stream chunks
- The Arc pattern for sharing a single HTTP client across multiple tool instances
- Context window management as a practical problem — not just a number, but something you actively manage
- How to contribute to an open-source Rust project (found deprecated constants in Rig, opened PR)

## Status

Active. Core chatbot with streaming, tools, persistence, and context management all working. Roadmap includes RAG with local files, persistent memory across sessions, and MCP server integration.

## Repo

~/Work/marvin
