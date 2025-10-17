//! Example demonstrating the Contents API for extracting content from URLs

use std::env;
use valyu::{ContentsRequest, ValyuClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get API key from environment
    let api_key = env::var("VALYU_API_KEY")
        .expect("VALYU_API_KEY must be set in .env file");

    // Create the Valyu client
    let client = ValyuClient::new(api_key);

    // Example URLs to process
    let urls = vec![
        "https://www.nature.com/articles/s41586-024-07998-6".to_string(),
        "https://arxiv.org/abs/2401.00001".to_string(),
    ];

    println!("🔄 Processing {} URLs...\n", urls.len());
    for (idx, url) in urls.iter().enumerate() {
        println!("[{}] {}", idx + 1, url);
    }
    println!();

    // Build the request with custom parameters
    let request = ContentsRequest::new(urls)
        .with_response_length("medium")
        .with_extract_effort("high")
        .with_summary(true);

    // Execute the request
    let response = client.contents(&request).await?;

    // Display results
    println!("✅ Processing complete!\n");

    if let Some(tx_id) = &response.tx_id {
        println!("📋 Transaction ID: {}", tx_id);
    }

    if let Some(urls_processed) = response.urls_processed {
        println!("✓ URLs processed: {}", urls_processed);
    }

    if let Some(urls_failed) = response.urls_failed {
        if urls_failed > 0 {
            println!("✗ URLs failed: {}", urls_failed);
        }
    }

    if let Some(total_cost) = response.total_cost_dollars {
        println!("💰 Total cost: ${:.4}", total_cost);
    }

    if let Some(total_chars) = response.total_characters {
        println!("📝 Total characters: {}", total_chars);
    }

    // Display extracted content
    if let Some(results) = &response.results {
        println!("\n📄 Extracted Content ({} results):", results.len());
        println!("{}", "=".repeat(80));

        for (idx, result) in results.iter().enumerate() {
            println!("\n[{}] {}", idx + 1, result.title.as_deref().unwrap_or("Untitled"));

            if let Some(url) = &result.url {
                println!("🔗 URL: {}", url);
            }

            if let Some(pub_date) = &result.publication_date {
                println!("📅 Published: {}", pub_date);
            }

            if let Some(desc) = &result.description {
                println!("\n📖 Description:");
                println!("{}", desc);
            }

            if let Some(content) = &result.content {
                println!("\n📋 Content:");
                let content_str = match content {
                    serde_json::Value::String(s) => s.clone(),
                    _ => serde_json::to_string_pretty(&content)?,
                };

                let preview = if content_str.len() > 500 {
                    format!("{}...", &content_str[..500])
                } else {
                    content_str
                };
                println!("{}", preview);
            }

            if let Some(images) = &result.images {
                if !images.is_empty() {
                    println!("\n🖼️  Images found: {}", images.len());
                    for (i, img) in images.iter().take(3).enumerate() {
                        println!("   [{}] {}", i + 1, img);
                    }
                }
            }

            if let Some(cost) = result.cost_dollars {
                println!("\n💵 Cost for this URL: ${:.4}", cost);
            }

            println!("{}", "-".repeat(80));
        }
    }

    Ok(())
}
