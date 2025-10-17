//! # Valyu Rust SDK
//!
//! Official Rust SDK for the [Valyu AI API](https://valyu.ai).
//!
//! Valyu provides advanced AI-powered search capabilities, combining web search
//! with proprietary data sources to deliver comprehensive, relevant results.
//!
//! ## Quick Start
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! valyu = "0.1"
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! ## Basic Usage
//!
//! ```no_run
//! use valyu::ValyuClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client with your API key
//!     let client = ValyuClient::new("your-api-key");
//!
//!     // Perform a simple search
//!     let response = client.search("quantum computing").await?;
//!
//!     // Process results
//!     if let Some(results) = &response.results {
//!         for result in results {
//!             println!("{}: {}",
//!                 result.title.as_deref().unwrap_or("Untitled"),
//!                 result.url.as_deref().unwrap_or("No URL")
//!             );
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Advanced Usage
//!
//! Use the builder pattern for more control over search parameters:
//!
//! ```no_run
//! use valyu::{ValyuClient, DeepSearchRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ValyuClient::new("your-api-key");
//!
//!     let request = DeepSearchRequest::new("artificial intelligence")
//!         .with_max_results(10)
//!         .with_search_type("web")
//!         .with_fast_mode(true)
//!         .with_response_length("medium")
//!         .with_date_range("2024-01-01", "2024-12-31");
//!
//!     let response = client.deep_search(&request).await?;
//!
//!     println!("Transaction ID: {}", response.tx_id.as_deref().unwrap_or("N/A"));
//!     println!("Cost: ${:.4}", response.total_deduction_dollars.unwrap_or(0.0));
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! - **Type-safe API**: Full type coverage with serde support
//! - **Async/await**: Built on tokio and reqwest for async operations
//! - **Builder pattern**: Fluent interface for constructing requests
//! - **Comprehensive error handling**: Detailed error types for all failure cases
//! - **Well documented**: Extensive documentation and examples
//!
//! ## Error Handling
//!
//! The SDK uses a custom [`ValyuError`] type for all errors:
//!
//! ```no_run
//! use valyu::{ValyuClient, ValyuError};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = ValyuClient::new("your-api-key");
//!
//!     match client.search("test").await {
//!         Ok(response) => println!("Success!"),
//!         Err(ValyuError::InvalidApiKey) => eprintln!("Invalid API key"),
//!         Err(ValyuError::RateLimitExceeded) => eprintln!("Rate limit exceeded"),
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! ```

#![warn(missing_docs)]
#![warn(clippy::all)]

mod client;
mod error;
mod types;

// Re-export public API
pub use client::ValyuClient;
pub use error::{Result, ValyuError};
pub use types::{
    // DeepSearch API
    DeepSearchRequest, DeepSearchResponse, ResultsBySource, SearchResult,
    // Contents API
    AnswerCost, AnswerRequest, AnswerResponse, AnswerSearchMetadata, AnswerSearchResult, AiUsage,
    ContentResult, ContentsRequest, ContentsResponse, ResponseLength, SummaryOption,
};
