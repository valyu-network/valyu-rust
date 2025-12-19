# Valyu Rust SDK

[![Crates.io](https://img.shields.io/crates/v/valyu.svg)](https://crates.io/crates/valyu)
[![Documentation](https://docs.rs/valyu/badge.svg)](https://docs.rs/valyu)
[![License](https://img.shields.io/crates/l/valyu.svg)](https://github.com/valyu-network/valyu-rust/blob/main/LICENSE)
[![Alpha](https://img.shields.io/badge/status-alpha-orange.svg)](https://github.com/valyu-network/valyu-rust)

Official Rust SDK for the [Valyu AI API](https://valyu.ai).

**Search for AIs** - Valyu's Deepsearch API gives AI the context it needs. Integrate trusted, high-quality public and proprietary sources, with full-text multimodal retrieval.

Get **$10 free credits** for the Valyu API when you sign up at [platform.valyu.ai](https://platform.valyu.ai)!
No credit card required.

> **⚠️ Alpha Release**: This SDK is currently in alpha. The API is stable, but some features and interfaces may change based on user feedback. We welcome your input!

## How Does It Work?

We do all the heavy lifting for you - **one unified API** for all data:

- **Academic & Research Content** - Access millions of scholarly papers, journals, and textbooks
- **Real-time Web Search** - Get the latest information from across the internet
- **Medical Literature** - Peer-reviewed studies, clinical trials, and regulatory documents
- **Structured Financial Data** - Stock prices, market data, and financial metrics
- **Intelligent Reranking** - Results across all sources are automatically sorted by relevance
- **Transparent Pricing** - Pay only for what you use with clear CPM (cost per thousand) pricing

## Features

- **Type-safe API**: Full type coverage with serde support for all API endpoints
- **Async/await**: Built on tokio and reqwest for efficient async operations
- **Builder pattern**: Fluent interface for constructing complex search requests
- **Comprehensive error handling**: Detailed error types for all failure scenarios
- **Four powerful endpoints**: DeepSearch, Contents, Answer, and DeepResearch APIs
- **Well documented**: Extensive documentation with examples for all features
- **Production ready**: Designed for reliability and ease of use in production environments

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
valyu = "0.1"
tokio = { version = "1", features = ["full"] }
```

Get your API key from [platform.valyu.ai](https://platform.valyu.ai).

## Quick Start

### Basic Usage

```rust
use valyu::ValyuClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with your API key
    let client = ValyuClient::new("your-api-key");

    // Perform a simple search
    let response = client.search("quantum computing").await?;

    // Process results
    if let Some(results) = &response.results {
        for result in results {
            println!("{}: {}",
                result.title.as_deref().unwrap_or("Untitled"),
                result.url.as_deref().unwrap_or("No URL")
            );
        }
    }

    Ok(())
}
```

### Advanced Usage with Builder Pattern

```rust
use valyu::{ValyuClient, DeepSearchRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ValyuClient::new("your-api-key");

    // Build a custom search request with specific parameters
    let request = DeepSearchRequest::new("artificial intelligence")
        .with_max_results(10)
        .with_search_type("web")
        .with_fast_mode(true)
        .with_response_length("medium")
        .with_relevance_threshold(0.7)
        .with_date_range("2024-01-01", "2024-12-31");

    let response = client.deep_search(&request).await?;

    println!("Transaction ID: {}", response.tx_id.as_deref().unwrap_or("N/A"));
    println!("Cost: ${:.4}", response.total_deduction_dollars.unwrap_or(0.0));

    Ok(())
}
```

## Authentication

Set your API key in one of these ways:

### Environment Variable (Recommended)

```bash
export VALYU_API_KEY="your-api-key-here"
```

Then in your code:

```rust
use std::env;
use valyu::ValyuClient;

let api_key = env::var("VALYU_API_KEY").expect("VALYU_API_KEY must be set");
let client = ValyuClient::new(api_key);
```

### Direct Initialization

```rust
use valyu::ValyuClient;

let client = ValyuClient::new("your-api-key-here");
```

### Using .env File

For local development, use the `dotenvy` crate:

```toml
[dependencies]
dotenvy = "0.15"
```

```rust
use std::env;
use valyu::ValyuClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let api_key = env::var("VALYU_API_KEY")?;
    let client = ValyuClient::new(api_key);
    Ok(())
}
```

## Error Handling

The SDK uses a custom `ValyuError` type for detailed error handling:

```rust
use valyu::{ValyuClient, ValyuError};

#[tokio::main]
async fn main() {
    let client = ValyuClient::new("your-api-key");

    match client.search("test").await {
        Ok(response) => {
            if response.success {
                println!("Success! Found {} results",
                    response.results.as_ref().map(|r| r.len()).unwrap_or(0));
            } else {
                eprintln!("API returned error: {:?}", response.error);
            }
        }
        Err(ValyuError::InvalidApiKey) => eprintln!("Invalid API key provided"),
        Err(ValyuError::RateLimitExceeded) => eprintln!("Rate limit exceeded - please retry later"),
        Err(ValyuError::ServiceUnavailable) => eprintln!("Service temporarily unavailable"),
        Err(ValyuError::InvalidRequest(msg)) => eprintln!("Invalid request: {}", msg),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Custom HTTP Client Configuration

```rust
use valyu::ValyuClient;
use std::time::Duration;

let http_client = reqwest::Client::builder()
    .timeout(Duration::from_secs(30))
    .build()
    .unwrap();

let client = ValyuClient::with_client("your-api-key", http_client);
```

## API Reference

### ValyuClient

The main client for interacting with the Valyu API.

#### Methods

- `new(api_key: impl Into<String>) -> Self` - Create a new client with an API key
- `with_base_url(api_key, base_url) -> Self` - Create client with custom base URL
- `with_client(api_key, reqwest::Client) -> Self` - Create client with custom HTTP client
- `search(query: impl Into<String>) -> Result<DeepSearchResponse>` - Simple search with default settings
- `deep_search(request: &DeepSearchRequest) -> Result<DeepSearchResponse>` - Advanced search with custom parameters
- `contents(request: &ContentsRequest) -> Result<ContentsResponse>` - Extract content from URLs
- `answer(request: &AnswerRequest) -> Result<AnswerResponse>` - Get AI-powered answers
- `ask(query: impl Into<String>) -> Result<AnswerResponse>` - Simple answer with defaults
- `deepresearch_create(request: &DeepResearchCreateRequest) -> Result<DeepResearchCreateResponse>` - Create async research task
- `deepresearch_status(task_id) -> Result<DeepResearchStatusResponse>` - Get task status
- `deepresearch_wait(task_id, poll_interval_secs, max_wait_secs) -> Result<DeepResearchStatusResponse>` - Wait for task completion
- `deepresearch_list(api_key_id, limit) -> Result<DeepResearchListResponse>` - List tasks
- `deepresearch_cancel(task_id) -> Result<DeepResearchOperationResponse>` - Cancel running task
- `deepresearch_delete(task_id) -> Result<DeepResearchOperationResponse>` - Delete task
- `research(query: impl Into<String>) -> Result<DeepResearchCreateResponse>` - Simple research with defaults

### DeepSearchRequest

Builder for constructing search requests with optional parameters.

#### Constructor

- `new(query: impl Into<String>) -> Self` - Create a new request with just a query

#### Builder Methods

- `with_max_results(max: u8) -> Self` - Set max results (1-20)
- `with_search_type(type: impl Into<String>) -> Self` - Set search type: "all", "web", or "proprietary"
- `with_fast_mode(enabled: bool) -> Self` - Enable fast mode for reduced latency
- `with_response_length(length: impl Into<String>) -> Self` - Set response length: "short", "medium", "large", or "max"
- `with_relevance_threshold(threshold: f64) -> Self` - Set relevance threshold (0.0-1.0)
- `with_included_sources(sources: Vec<String>) -> Self` - Specify sources to include
- `with_excluded_sources(sources: Vec<String>) -> Self` - Specify sources to exclude
- `with_date_range(start: impl Into<String>, end: impl Into<String>) -> Self` - Set date range (YYYY-MM-DD format)
- `with_max_price(price: f64) -> Self` - Set maximum CPM price
- `with_category(category: impl Into<String>) -> Self` - Set category filter
- `with_country_code(code: impl Into<String>) -> Self` - Set country code (2-letter ISO)
- `with_is_tool_call(is_tool_call: bool) -> Self` - Set whether this is a tool call

### ContentsRequest

Builder for URL content extraction requests.

#### Constructor

- `new(urls: Vec<String>) -> Self` - Create request with URLs (1-10)

#### Builder Methods

- `with_response_length(length: impl Into<String>) -> Self` - Set response length preset
- `with_custom_response_length(chars: i32) -> Self` - Set custom character limit (1K-1M)
- `with_extract_effort(effort: impl Into<String>) -> Self` - Set extraction effort: "normal", "high", or "auto"
- `with_summary(enabled: bool) -> Self` - Enable/disable default summarization
- `with_summary_instructions(instructions: impl Into<String>) -> Self` - Set custom summary instructions
- `with_summary_schema(schema: serde_json::Value) -> Self` - Set JSON schema for structured extraction
- `with_max_price_dollars(max_price: f64) -> Self` - Set maximum price in dollars

### AnswerRequest

Builder for AI-powered answer requests.

#### Constructor

- `new(query: impl Into<String>) -> Self` - Create request with query

#### Builder Methods

- `with_system_instructions(instructions: impl Into<String>) -> Self` - Set custom AI instructions (max 2000 chars)
- `with_structured_output(schema: serde_json::Value) -> Self` - Set JSON schema for structured response
- `with_search_type(type: impl Into<String>) -> Self` - Set search type: "all", "web", or "proprietary"
- `with_fast_mode(enabled: bool) -> Self` - Enable fast mode
- `with_data_max_price(price: f64) -> Self` - Set maximum data CPM price
- `with_included_sources(sources: Vec<String>) -> Self` - Set included sources
- `with_excluded_sources(sources: Vec<String>) -> Self` - Set excluded sources
- `with_date_range(start, end) -> Self` - Set date range filter
- `with_country_code(code: impl Into<String>) -> Self` - Set country code

### Response Types

#### DeepSearchResponse

Fields:
- `success: bool` - Whether the request succeeded
- `error: Option<String>` - Error message if failed
- `tx_id: Option<String>` - Transaction ID
- `query: Option<String>` - The search query
- `results: Option<Vec<SearchResult>>` - Array of search results
- `results_by_source: Option<ResultsBySource>` - Breakdown of results by source
- `total_deduction_dollars: Option<f64>` - Cost in dollars
- `total_characters: Option<i32>` - Total characters in results

#### SearchResult

Individual search result with fields including:
- `title: Option<String>` - Result title
- `url: Option<String>` - Result URL
- `content: Option<String>` - Result content/snippet
- `source: Option<String>` - Source type
- `publication_date: Option<String>` - Publication date
- `authors: Option<Vec<String>>` - List of authors
- `citation: Option<String>` - Citation information
- And more...

#### ContentsResponse

Fields:
- `success: bool` - Whether request succeeded
- `results: Option<Vec<ContentResult>>` - Extracted content results
- `urls_requested: Option<i32>` - Number of URLs requested
- `urls_processed: Option<i32>` - Number successfully processed
- `urls_failed: Option<i32>` - Number that failed
- `total_cost_dollars: Option<f64>` - Total cost

#### AnswerResponse

Fields:
- `success: bool` - Whether request succeeded
- `contents: Option<serde_json::Value>` - AI-generated answer (string or structured)
- `data_type: Option<String>` - "unstructured" or "structured"
- `search_results: Option<Vec<AnswerSearchResult>>` - Sources used
- `search_metadata: Option<AnswerSearchMetadata>` - Search metadata
- `ai_usage: Option<AiUsage>` - Token usage statistics
- `cost: Option<AnswerCost>` - Cost breakdown (search + AI)

### DeepResearchCreateRequest

Builder for creating comprehensive async research tasks.

#### Constructor

- `new(input: impl Into<String>) -> Self` - Create a new research request

#### Builder Methods

- `with_mode(mode: DeepResearchMode) -> Self` - Set research mode: `Fast`, `Lite`, or `Heavy`
- `with_output_formats(formats: Vec<String>) -> Self` - Set output formats: ["markdown"], ["markdown", "pdf"]
- `with_structured_output(schema: serde_json::Value) -> Self` - Use JSON schema for structured output
- `with_strategy(strategy: impl Into<String>) -> Self` - Set natural language research strategy
- `with_search(config: DeepResearchSearchConfig) -> Self` - Set search configuration
- `with_urls(urls: Vec<String>) -> Self` - Add URLs to extract content from (max 10)
- `with_files(files: Vec<DeepResearchFileAttachment>) -> Self` - Add file attachments (max 10)
- `with_mcp_servers(servers: Vec<DeepResearchMCPServerConfig>) -> Self` - Add MCP servers (max 5)
- `with_code_execution(enabled: bool) -> Self` - Enable/disable code execution
- `with_previous_reports(ids: Vec<String>) -> Self` - Use previous reports as context (max 3)
- `with_webhook_url(url: impl Into<String>) -> Self` - Set webhook for completion notification
- `with_metadata(metadata: serde_json::Value) -> Self` - Set custom metadata

### DeepResearchMode

Research mode options:
- `Fast` - Quick lookups, simple questions (1-2 min, $0.15)
- `Lite` - Moderate research depth (5-10 min, $0.50)
- `Heavy` - Comprehensive analysis (15-90 min, $1.50)

### DeepResearchStatusResponse

Fields:
- `success: bool` - Whether request succeeded
- `deepresearch_id: Option<String>` - Task identifier
- `status: Option<DeepResearchStatus>` - Current status: Queued, Running, Completed, Failed, Cancelled
- `query: Option<String>` - Original query
- `mode: Option<DeepResearchMode>` - Research mode used
- `progress: Option<DeepResearchProgress>` - Current/total steps (when running)
- `output: Option<serde_json::Value>` - Research output (when completed)
- `pdf_url: Option<String>` - PDF download URL (if requested)
- `images: Option<Vec<DeepResearchImage>>` - Generated images
- `sources: Option<Vec<DeepResearchSource>>` - Sources used
- `usage: Option<DeepResearchUsage>` - Cost breakdown

## Examples

The repository includes several examples demonstrating different use cases:

### Basic Search

```bash
cargo run --example basic
```

```rust
use valyu::ValyuClient;

let client = ValyuClient::new("your-api-key");
let response = client.search("quantum computing").await?;

println!("Found {} results", response.results.as_ref().map(|r| r.len()).unwrap_or(0));
```

### Academic Research

Search academic papers on specific topics:

```rust
use valyu::DeepSearchRequest;

let request = DeepSearchRequest::new("transformer architecture improvements")
    .with_search_type("proprietary")
    .with_included_sources(vec!["valyu/valyu-arxiv".to_string()])
    .with_relevance_threshold(0.7)
    .with_max_results(10);

let response = client.deep_search(&request).await?;
```

### Web Search with Date Filtering

```rust
let request = DeepSearchRequest::new("AI safety developments")
    .with_search_type("web")
    .with_date_range("2024-01-01", "2024-12-31")
    .with_max_results(5);

let response = client.deep_search(&request).await?;
```

### Hybrid Search

Search both web and proprietary sources:

```rust
let request = DeepSearchRequest::new("quantum computing breakthroughs")
    .with_search_type("all")
    .with_category("technology")
    .with_relevance_threshold(0.6)
    .with_max_price(50.0);

let response = client.deep_search(&request).await?;
```

### Processing Results

```rust
let response = client.search("climate change solutions").await?;

if response.success {
    println!("Search cost: ${:.4}", response.total_deduction_dollars.unwrap_or(0.0));

    if let Some(by_source) = &response.results_by_source {
        println!("Sources: Web={:?}, Proprietary={:?}",
            by_source.web, by_source.proprietary);
    }

    if let Some(results) = &response.results {
        for (i, result) in results.iter().enumerate() {
            println!("\n{}. {}", i + 1, result.title.as_deref().unwrap_or("Untitled"));
            println!("   Source: {}", result.source.as_deref().unwrap_or("Unknown"));
            if let Some(content) = &result.content {
                println!("   Content: {}...", &content[..200.min(content.len())]);
            }
        }
    }
}
```

### Content Extraction

```bash
cargo run --example contents
```

Basic content extraction from URLs:

```rust
use valyu::ContentsRequest;

let request = ContentsRequest::new(vec![
    "https://example.com/article".to_string(),
]);

let response = client.contents(&request).await?;

if let Some(results) = &response.results {
    for result in results {
        println!("Title: {}", result.title.as_deref().unwrap_or("Untitled"));
        if let Some(content) = &result.content {
            println!("Content: {:?}", content);
        }
    }
}
```

### Content with AI Summary

```rust
let request = ContentsRequest::new(vec![
    "https://docs.python.org/3/tutorial/".to_string(),
])
.with_summary(true)
.with_response_length("max");

let response = client.contents(&request).await?;
```

### Structured Data Extraction

```rust
use serde_json::json;

let company_schema = json!({
    "type": "object",
    "properties": {
        "company_name": {"type": "string"},
        "founded_year": {"type": "integer"},
        "key_products": {
            "type": "array",
            "items": {"type": "string"},
            "maxItems": 3
        }
    }
});

let request = ContentsRequest::new(vec![
    "https://en.wikipedia.org/wiki/OpenAI".to_string(),
])
.with_summary_schema(company_schema)
.with_response_length("max");

let response = client.contents(&request).await?;
```

### AI-Powered Answers

```bash
cargo run --example answer
```

Get AI-generated answers with sources:

```rust
use valyu::AnswerRequest;

let request = AnswerRequest::new("What are the latest developments in quantum computing?")
    .with_search_type("web")
    .with_system_instructions("Focus on breakthroughs from 2024");

let response = client.answer(&request).await?;

if let Some(contents) = &response.contents {
    println!("Answer: {}", contents);
}

if let Some(sources) = &response.search_results {
    println!("\nSources ({}):", sources.len());
    for source in sources {
        println!("  - {}", source.title.as_deref().unwrap_or("Untitled"));
    }
}
```

### Structured Answer Output

```bash
cargo run --example answer_structured
```

```rust
use serde_json::json;

let schema = json!({
    "type": "object",
    "properties": {
        "summary": {"type": "string"},
        "key_points": {
            "type": "array",
            "items": {"type": "string"}
        }
    }
});

let request = AnswerRequest::new("quantum computing")
    .with_structured_output(schema);

let response = client.answer(&request).await?;
```

### Multiple URLs Processing

```rust
let request = ContentsRequest::new(vec![
    "https://www.valyu.ai/".to_string(),
    "https://docs.valyu.ai/overview".to_string(),
    "https://www.valyu.ai/blogs/why-ai-agents-and-llms-struggle-with-search-and-data-access".to_string(),
])
.with_summary_instructions("Provide key takeaways in bullet points")
.with_max_price_dollars(2.0);

let response = client.contents(&request).await?;

println!("Processed {}/{} URLs",
    response.urls_processed.unwrap_or(0),
    response.urls_requested.unwrap_or(0));
println!("Cost: ${:.4}", response.total_cost_dollars.unwrap_or(0.0));
```

### DeepResearch (Async Research Tasks)

```bash
cargo run --example deepresearch
```

Perform comprehensive async research:

```rust
use valyu::{DeepResearchCreateRequest, DeepResearchMode};

// Create a research task
let request = DeepResearchCreateRequest::new("What are the key differences between RAG and fine-tuning?")
    .with_mode(DeepResearchMode::Lite)
    .with_output_formats(vec!["markdown".to_string()]);

let task = client.deepresearch_create(&request).await?;
println!("Task created: {:?}", task.deepresearch_id);

// Wait for completion
let result = client.deepresearch_wait(
    task.deepresearch_id.as_ref().unwrap(),
    5,   // Poll every 5 seconds
    900, // Timeout after 15 minutes
).await?;

// Access results
if let Some(output) = &result.output {
    println!("Research output: {}", output);
}

if let Some(sources) = &result.sources {
    println!("Used {} sources", sources.len());
}

if let Some(usage) = &result.usage {
    println!("Total cost: ${:.4}", usage.total_cost);
}
```

### All Examples

```bash
# Basic search example
cargo run --example basic

# Advanced search with custom parameters
cargo run --example advanced

# Content extraction from URLs
cargo run --example contents

# AI-powered answers
cargo run --example answer

# Structured answer output
cargo run --example answer_structured

# DeepResearch async research tasks
cargo run --example deepresearch

# Custom HTTP client configuration
cargo run --example custom_client
```

Note: You'll need to create a `.env` file with your API key:

```bash
cp .env.example .env
# Edit .env and add your VALYU_API_KEY
```

## Known Limitations

As an alpha release, there are some known limitations:

### Current Limitations

- **No built-in retry logic**: The SDK does not automatically retry requests that fail due to transient errors (503 Service Unavailable, 429 Rate Limit). Implement your own retry logic with exponential backoff if needed.

- **No automatic rate limiting**: Rate limit management is left to the user. If you receive 429 errors, implement delays between requests or use a rate limiting library.

- **No streaming support**: All responses are returned as complete objects. If the API adds streaming in the future, SDK updates will be required.

- **Limited client-side validation**: The SDK performs minimal validation on request parameters (e.g., checking ranges for `max_num_results`). Invalid parameters will result in API errors.

- **No pagination support**: If the API adds pagination for large result sets in the future, SDK updates will be needed.

- **Alpha status**: As an alpha release, some APIs and type signatures may change based on user feedback. We will follow semantic versioning for breaking changes.

### Workarounds

#### Implementing Retry Logic

```rust
use valyu::{ValyuClient, ValyuError};
use std::time::Duration;
use tokio::time::sleep;

async fn search_with_retry(
    client: &ValyuClient,
    query: &str,
    max_retries: u32,
) -> Result<valyu::DeepSearchResponse, ValyuError> {
    let mut retries = 0;

    loop {
        match client.search(query).await {
            Ok(response) => return Ok(response),
            Err(ValyuError::RateLimitExceeded) | Err(ValyuError::ServiceUnavailable) => {
                if retries >= max_retries {
                    return Err(ValyuError::ServiceUnavailable);
                }
                retries += 1;
                let delay = Duration::from_secs(2_u64.pow(retries));
                sleep(delay).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

#### Rate Limiting

```rust
use tokio::time::{sleep, Duration};

// Simple rate limiter: max 10 requests per second
let mut last_request = std::time::Instant::now();
let min_interval = Duration::from_millis(100);

for query in queries {
    let elapsed = last_request.elapsed();
    if elapsed < min_interval {
        sleep(min_interval - elapsed).await;
    }

    let response = client.search(&query).await?;
    last_request = std::time::Instant::now();
}
```

We welcome feedback on these limitations and suggestions for improvement!

## Getting Started

1. **Sign up** for a free account at [platform.valyu.ai](https://platform.valyu.ai) and get $10 free credits
2. **Get your API key** from the dashboard
3. **Install the SDK**: Add `valyu = "0.1"` to your `Cargo.toml`
4. **Start building** with the examples above

## Documentation

- [Crate Documentation on docs.rs](https://docs.rs/valyu)
- [Valyu API Reference](https://docs.valyu.ai/api-reference/endpoint/deepsearch)
- [Valyu Platform Documentation](https://docs.valyu.ai)
- [Example Code](./examples)

## Minimum Supported Rust Version (MSRV)

This crate requires Rust 1.70 or later.

## Support

- **Documentation**: [docs.valyu.ai](https://docs.valyu.ai)
- **API Reference**: Full parameter documentation above
- **Examples**: Check the [examples/](./examples) directory in this repository
- **Issues**: [Report bugs on GitHub](https://github.com/valyu-network/valyu-rust/issues)
- **Platform**: [platform.valyu.ai](https://platform.valyu.ai)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

Before submitting a PR, please make sure to:

1. Run `cargo test` to ensure all tests pass
2. Run `cargo fmt` to format your code
3. Run `cargo clippy` to check for common mistakes
4. Add tests for any new functionality

## Feedback

This SDK is in alpha and we value your feedback! Please open an issue on GitHub to:

- Report bugs or issues
- Suggest new features
- Share your use cases
- Ask questions

Your input helps us improve the SDK for everyone.
