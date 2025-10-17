//! Advanced example demonstrating the builder pattern with custom parameters

use std::env;
use valyu::{DeepSearchRequest, ValyuClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get API key from environment
    let api_key = env::var("VALYU_API_KEY")
        .expect("VALYU_API_KEY must be set in .env file");

    // Create the Valyu client
    let client = ValyuClient::new(api_key);

    // Build a custom search request with multiple parameters
    let request = DeepSearchRequest::new("artificial intelligence breakthroughs 2024")
        .with_max_results(10)
        .with_search_type("web")
        .with_fast_mode(false)
        .with_response_length("medium")
        .with_relevance_threshold(0.7)
        .with_date_range("2024-01-01", "2024-12-31");

    println!("🔍 Advanced Search");
    println!("Query: {}", request.query);
    println!("Max results: {:?}", request.max_num_results);
    println!("Search type: {:?}", request.search_type);
    println!("Response length: {:?}", request.response_length);
    println!("Relevance threshold: {:?}", request.relevance_threshold);
    println!("Date range: {} to {}\n",
        request.start_date.as_deref().unwrap_or("N/A"),
        request.end_date.as_deref().unwrap_or("N/A")
    );

    // Execute the search
    let response = client.deep_search(&request).await?;

    println!("✅ Search completed successfully!\n");

    // Display metadata
    if let Some(tx_id) = &response.tx_id {
        println!("📋 Transaction ID: {}", tx_id);
    }

    if let Some(total_chars) = response.total_characters {
        println!("📝 Total characters: {}", total_chars);
    }

    if let Some(cost) = response.total_deduction_dollars {
        println!("💰 Cost: ${:.4}", cost);
    }

    // Display results summary
    if let Some(results) = &response.results {
        println!("\n📊 Found {} results\n", results.len());

        for (idx, result) in results.iter().enumerate() {
            println!("[{}] {}", idx + 1, result.title.as_deref().unwrap_or("Untitled"));

            if let Some(url) = &result.url {
                println!("    URL: {}", url);
            }

            if let Some(pub_date) = &result.publication_date {
                println!("    Published: {}", pub_date);
            }

            if let Some(citation_count) = result.citation_count {
                println!("    Citations: {}", citation_count);
            }

            println!();
        }
    }

    Ok(())
}
