use llmweb::LlmWeb;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VXNA {
    pub username: String,

    pub avatar_url: String,

    pub profile_url: String,

    pub title: String,

    pub topic_url: String,

    pub topic_id: u64,

    pub relative_time: String,

    pub reply_count: u32,

    pub last_replier: Option<String>,
}

#[tokio::main]
async fn main() {
    // Load the schema from an external file as a string.
    let schema_str = include_str!("../schemas/v2ex_schema.json");
    let schema: Value = serde_json::from_str(schema_str).unwrap();

    let structed_value: Vec<VXNA> = LlmWeb::new("gemini-2.0-flash")
        .stream("https://v2ex.com/go/vxna", schema)
        .await
        .unwrap();
    println!("{:#?}", structed_value);
}
