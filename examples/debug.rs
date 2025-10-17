//! Debug example to see raw API response

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let api_key = env::var("VALYU_API_KEY")?;

    // Make raw request
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.valyu.ai/v1/deepsearch")
        .header("Content-Type", "application/json")
        .header("x-api-key", &api_key)
        .json(&serde_json::json!({
            "query": "test query",
            "max_num_results": 2
        }))
        .send()
        .await?;

    println!("Status: {}", response.status());

    let text = response.text().await?;
    println!("\nRaw response:");
    println!("{}", text);

    // Try to parse as generic JSON
    let json: serde_json::Value = serde_json::from_str(&text)?;
    println!("\nParsed JSON (pretty):");
    println!("{}", serde_json::to_string_pretty(&json)?);

    Ok(())
}
