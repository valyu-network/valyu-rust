//! Example demonstrating custom HTTP client configuration

use std::env;
use std::time::Duration;
use valyu::ValyuClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get API key from environment
    let api_key = env::var("VALYU_API_KEY")
        .expect("VALYU_API_KEY must be set in .env file");

    // Create a custom HTTP client with specific timeout and retry settings
    let http_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("valyu-rust-sdk-example/0.1.0")
        .build()?;

    // Create the Valyu client with custom HTTP client
    let client = ValyuClient::with_client(api_key, http_client);

    println!("🔍 Searching with custom HTTP client configuration...\n");

    // Perform a search
    let response = client
        .search("machine learning frameworks")
        .await?;

    println!("✅ Success!");

    if let Some(results) = &response.results {
        println!("📚 Found {} results", results.len());

        for (idx, result) in results.iter().take(3).enumerate() {
            println!("\n[{}] {}", idx + 1, result.title.as_deref().unwrap_or("Untitled"));
            if let Some(url) = &result.url {
                println!("    {}", url);
            }
        }
    }

    Ok(())
}
