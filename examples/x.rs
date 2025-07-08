use llmweb::LlmWeb;
use serde_json::{Value, json};

// TODO:
// https://github.com/zTgx/llmweb/issues/2
#[tokio::main]
async fn main() {
    let schema_json = json!({
        "type": "object",
        "properties": {
            "tweet_text": {
            "type": "string",
            },
        },
        "required": ["tweet_text"]
    });

    let llmweb = LlmWeb::new("gemini-2.0-flash");
    let structed_value: Value = llmweb
        .completion(
            "https://x.com/ztgx5/status/1942242787317133452",
            schema_json,
        )
        .await
        .unwrap();
    println!("{:#?}", structed_value);
}
