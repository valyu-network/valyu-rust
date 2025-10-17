//! Basic example demonstrating simple search usage

use std::env;
use valyu::ValyuClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get API key from environment
    let api_key = env::var("VALYU_API_KEY")
        .expect("VALYU_API_KEY must be set in .env file");

    // Create the Valyu client
    let client = ValyuClient::new(api_key);

    println!("🔍 Searching: latest developments in quantum computing\n");

    // Perform a simple search
    let response = client
        .search("latest developments in quantum computing")
        .await?;

    // Display results
    println!("✅ Success!");

    if let Some(tx_id) = &response.tx_id {
        println!("📋 Transaction ID: {}", tx_id);
    }

    if let Some(results_by_source) = &response.results_by_source {
        println!("\n📊 Results by source:");
        if let Some(web) = results_by_source.web {
            println!("   Web: {}", web);
        }
        if let Some(proprietary) = results_by_source.proprietary {
            println!("   Proprietary: {}", proprietary);
        }
    }

    if let Some(cost) = response.total_deduction_dollars {
        println!("💰 Cost: ${:.4}", cost);
    }

    // Display search results
    if let Some(results) = &response.results {
        println!("\n📚 Search Results ({} found):", results.len());
        println!("{}", "=".repeat(80));

        for (idx, result) in results.iter().enumerate() {
            println!("\n[{}] {}", idx + 1, result.title.as_deref().unwrap_or("Untitled"));

            if let Some(url) = &result.url {
                println!("🔗 URL: {}", url);
            }

            if let Some(source) = &result.source {
                println!("📍 Source: {}", source);
            }

            if let Some(pub_date) = &result.publication_date {
                println!("📅 Published: {}", pub_date);
            }

            if let Some(authors) = &result.authors {
                if !authors.is_empty() {
                    println!("✍️  Authors: {}", authors.join(", "));
                }
            }

            if let Some(content) = &result.content {
                let preview = if content.len() > 200 {
                    format!("{}...", &content[..200])
                } else {
                    content.clone()
                };
                println!("\n{}", preview);
            }

            println!("{}", "-".repeat(80));
        }
    }

    Ok(())
}
