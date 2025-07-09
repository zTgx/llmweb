use llmweb::LlmWeb;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    let schema_json = json!({
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "username": {
                    "type": "string",
                    "description": "发布者用户名"
                },
                "avatar_url": {
                    "type": "string",
                    "description": "头像URL"
                },
                "profile_url": {
                    "type": "string",
                    "description": "个人主页URL"
                },
                "title": {
                    "type": "string",
                    "description": "帖子标题"
                },
                "topic_url": {
                    "type": "string",
                    "description": "帖子链接"
                },
                "topic_id": {
                    "type": "integer",
                    "description": "帖子ID"
                },
                "relative_time": {
                    "type": "string",
                    "description": "相对时间描述"
                },
                "reply_count": {
                    "type": "integer",
                    "description": "回复数"
                },
                "last_replier": {
                    "type": "string",
                    "description": "最后回复者"
                }
            },
            "required": [
            "username",
            "avatar_url",
            "title",
            "topic_url",
            ]
        }
    });

    let llmweb = LlmWeb::new("gemini-2.0-flash");
    let structed_value: Vec<VXNA> = llmweb
        .completion("https://v2ex.com/go/vxna", schema_json)
        .await
        .unwrap();
    println!("{:#?}", structed_value);
}
