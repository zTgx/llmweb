use llmweb::LlmWeb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Story {
    title: String,
    points: f32,
    by: Option<String>,
    comments_url: Option<String>,
}

#[tokio::main]
async fn main() {
    // Load the schema from an external file as a string.
    let schema_str = include_str!("../schemas/hn_schema.json");

    let llmweb = LlmWeb::new("gemini-2.0-flash");
    eprintln!("Fetching from Hacker News and extracting stories...");

    // Use the convenience method `completion_from_schema_str` which handles
    // parsing the schema string internally.
    let structed_value: Vec<Story> = llmweb
        .completion_from_schema_str("https://news.ycombinator.com", schema_str)
        .await
        .unwrap();
    println!("{:#?}", structed_value);
}
