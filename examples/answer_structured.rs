//! Example demonstrating structured output with the Answer API

use serde_json::json;
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

    // Define a JSON schema for structured output
    let schema = json!({
        "type": "object",
        "properties": {
            "summary": {
                "type": "string",
                "description": "A concise summary of the topic"
            },
            "key_points": {
                "type": "array",
                "items": {"type": "string"},
                "description": "List of key points or findings"
            },
            "timeline": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "date": {"type": "string"},
                        "event": {"type": "string"}
                    }
                },
                "description": "Timeline of important events"
            },
            "impact": {
                "type": "string",
                "description": "Overall impact and significance"
            }
        },
        "required": ["summary", "key_points"]
    });

    // Example query
    let query = "What is artificial intelligence?";

    println!("❓ Question: {}\n", query);

    // Build the request with structured output
    let request = AnswerRequest::new(query)
        .with_search_type("web")
        .with_structured_output(schema);

    println!("🔍 Generating structured answer...\n");

    // Execute the request
    let response = client.answer(&request).await?;

    // Display results
    println!("✅ Answer received!\n");

    if let Some(data_type) = &response.data_type {
        println!("📋 Data type: {}", data_type);
    }

    // Display the structured answer
    if let Some(contents) = &response.contents {
        println!("\n📊 Structured Answer:");
        println!("{}", "=".repeat(80));
        println!("{}", serde_json::to_string_pretty(contents)?);
        println!("{}", "=".repeat(80));
    }

    // Display sources
    if let Some(sources) = &response.search_results {
        println!("\n📚 Sources used: {}", sources.len());
    }

    // Display cost
    if let Some(cost) = &response.cost {
        if let Some(total) = cost.total_dollars {
            println!("\n💰 Total cost: ${:.4}", total);
        }
    }

    Ok(())
}
