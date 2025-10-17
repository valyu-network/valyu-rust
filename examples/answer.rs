//! Example demonstrating the Answer API for AI-powered question answering

use std::env;
use valyu::{AnswerRequest, ValyuClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get API key from environment
    let api_key = env::var("VALYU_API_KEY")
        .expect("VALYU_API_KEY must be set in .env file");

    // Create the Valyu client
    let client = ValyuClient::new(api_key);

    // Example query
    let query = "What is quantum computing?";

    println!("❓ Question: {}\n", query);

    // Build the request with custom parameters
    let request = AnswerRequest::new(query)
        .with_search_type("web");

    println!("🔍 Searching for sources and generating answer...\n");

    // Execute the request
    let response = client.answer(&request).await?;

    // Display results
    println!("✅ Answer received!\n");

    if let Some(ai_tx_id) = &response.ai_tx_id {
        println!("📋 Transaction ID: {}", ai_tx_id);
    }

    // Display the AI-generated answer
    if let Some(contents) = &response.contents {
        println!("\n💡 Answer:");
        println!("{}", "=".repeat(80));

        let answer_text = match contents {
            serde_json::Value::String(s) => s.clone(),
            _ => serde_json::to_string_pretty(&contents)?,
        };
        println!("{}", answer_text);
        println!("{}", "=".repeat(80));
    }

    // Display sources
    if let Some(sources) = &response.search_results {
        if !sources.is_empty() {
            println!("\n📚 Sources ({}):", sources.len());
            println!("{}", "-".repeat(80));

            for (idx, source) in sources.iter().enumerate() {
                println!("\n[{}] {}", idx + 1, source.title.as_deref().unwrap_or("Untitled"));

                if let Some(url) = &source.url {
                    println!("    🔗 {}", url);
                }

                if let Some(date) = &source.date {
                    println!("    📅 {}", date);
                }

                if let Some(snippet) = &source.snippet {
                    let preview = if snippet.len() > 150 {
                        format!("{}...", &snippet[..150])
                    } else {
                        snippet.clone()
                    };
                    println!("    📄 {}", preview);
                }
            }
        }
    }

    // Display metadata
    if let Some(metadata) = &response.search_metadata {
        println!("\n📊 Search Metadata:");

        if let Some(search_tx_id) = &metadata.search_tx_id {
            println!("   Search TX ID: {}", search_tx_id);
        }

        if let Some(result_count) = metadata.result_count {
            println!("   Results found: {}", result_count);
        }

        if let Some(total_chars) = metadata.total_characters {
            println!("   Total characters: {}", total_chars);
        }
    }

    // Display AI usage statistics
    if let Some(ai_usage) = &response.ai_usage {
        println!("\n🤖 AI Usage:");

        if let Some(input_tokens) = ai_usage.input_tokens {
            println!("   Input tokens: {}", input_tokens);
        }

        if let Some(output_tokens) = ai_usage.output_tokens {
            println!("   Output tokens: {}", output_tokens);
        }
    }

    // Display cost breakdown
    if let Some(cost) = &response.cost {
        println!("\n💰 Cost Breakdown:");

        if let Some(total) = cost.total_dollars {
            println!("   Total: ${:.4}", total);
        }

        if let Some(search_cost) = cost.search_dollars {
            println!("   Search: ${:.4}", search_cost);
        }

        if let Some(ai_cost) = cost.ai_dollars {
            println!("   AI: ${:.4}", ai_cost);
        }
    }

    println!();

    Ok(())
}
