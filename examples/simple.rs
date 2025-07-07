use llmweb::LlmWeb;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct Story {
    title: String,
    points: f32,
    by: Option<String>,
    comments_url: Option<String>,
}

#[tokio::main]
async fn main() {
    let schema_json = json!({
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "by": { "type": "string" },
                "comments_url": { "type": "string" },
                "points": { "type": "number" },
                "title": { "type": "string" }
            },
            "required": ["by", "comments_url", "points", "title"]
        }
    });

    let llmweb = LlmWeb::new("gemini-2.0-flash");
    let structed_value: Vec<Story> = llmweb
        .completion("https://news.ycombinator.com", schema_json)
        .await
        .unwrap();
    println!("{:#?}", structed_value);
}
