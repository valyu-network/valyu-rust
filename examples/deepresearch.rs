//! DeepResearch API Example
//!
//! This example demonstrates how to use the DeepResearch API to perform
//! comprehensive async research tasks.
//!
//! Run with: cargo run --example deepresearch

use valyu::{DeepResearchCreateRequest, DeepResearchMode, ValyuClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Valyu DeepResearch SDK Example ===\n");

    // Get API key from environment
    let api_key = std::env::var("VALYU_API_KEY").expect("VALYU_API_KEY must be set");

    // Create a client
    let client = ValyuClient::new(api_key);

    // Example 1: Create a basic research task
    println!("1. Creating a basic research task...");
    let request =
        DeepResearchCreateRequest::new("What are the key differences between RAG and fine-tuning for LLMs?")
            .with_mode(DeepResearchMode::Lite)
            .with_output_formats(vec!["markdown".to_string()]);

    let task = client.deepresearch_create(&request).await?;

    println!("   Task created: {:?}", task.deepresearch_id);
    println!("   Status: {:?}", task.status);
    println!("   Model: {:?}\n", task.model);

    // Example 2: Wait for completion
    println!("2. Waiting for task completion...");
    let task_id = task
        .deepresearch_id
        .as_ref()
        .expect("Task ID should be present");

    let result = client
        .deepresearch_wait(
            task_id,
            5,   // Poll every 5 seconds
            900, // Timeout after 15 minutes
        )
        .await?;

    println!("\n   Research completed!");
    println!("   Status: {:?}", result.status);

    // Display output
    if let Some(output) = &result.output {
        println!("\n=== Research Output ===");
        let output_str = output.to_string();
        if output_str.len() > 500 {
            println!("{}...\n", &output_str[..500]);
        } else {
            println!("{}\n", output_str);
        }
    }

    // Display sources
    if let Some(sources) = &result.sources {
        println!("=== Sources ({}) ===", sources.len());
        for (i, source) in sources.iter().take(5).enumerate() {
            println!("{}. {}", i + 1, source.title);
            println!("   URL: {}", source.url);
        }
        if sources.len() > 5 {
            println!("   ... and {} more sources", sources.len() - 5);
        }
    }

    // Display usage and cost
    if let Some(usage) = &result.usage {
        println!("\n=== Cost Breakdown ===");
        println!("   Search cost: ${:.4}", usage.search_cost);
        println!("   Contents cost: ${:.4}", usage.contents_cost);
        println!("   AI cost: ${:.4}", usage.ai_cost);
        println!("   Compute cost: ${:.4}", usage.compute_cost);
        println!("   Total cost: ${:.4}", usage.total_cost);
    }

    // Display images if any
    if let Some(images) = &result.images {
        if !images.is_empty() {
            println!("\n=== Generated Images ({}) ===", images.len());
            for (i, img) in images.iter().enumerate() {
                println!("{}. {} ({})", i + 1, img.title, img.image_type);
                println!("   URL: {}", img.image_url);
            }
        }
    }

    // PDF URL if available
    if let Some(pdf_url) = &result.pdf_url {
        println!("\n=== PDF Report ===");
        println!("   Download: {}", pdf_url);
    }

    println!("\n=== Example completed successfully! ===");

    Ok(())
}
