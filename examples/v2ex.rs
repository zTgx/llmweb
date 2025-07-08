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

// Example Output
// [
//     VXNA {
//         username: "MrBrand",
//         avatar_url: "https://cdn.v2ex.com/gravatar/d8065bea49aa2877ce13686772727711?s=48&d=retro",
//         profile_url: "/member/MrBrand",
//         title: "申请收录个人博客： https://www.brandhuang.com",
//         topic_url: "/t/1142704#reply2",
//         topic_id: 1142704,
//         relative_time: "4 天前",
//         reply_count: 2,
//         last_replier: Some(
//             "Livid",
//         ),
//     },
//     VXNA {
//         username: "iwukong",
//         avatar_url: "https://cdn.v2ex.com/avatar/419a/98e0/203596_large.png?m=1610208325",
//         profile_url: "/member/iwukong",
//         title: "申请收录 https://www.rizuo.com",
//         topic_url: "/t/1142679#reply8",
//         topic_id: 1142679,
//         relative_time: "5 天前",
//         reply_count: 8,
//         last_replier: Some(
//             "Livid",
//         ),
//     },
//     VXNA {
//         username: "SwaggyMacro",
//         avatar_url: "https://cdn.v2ex.com/gravatar/b71f5c2c6e2bc86b7e81299b2e8257c6?s=48&d=retro",
//         profile_url: "/member/SwaggyMacro",
//         title: "申请收录个人博客： https://b.ncii.cn",
//         topic_url: "/t/1142655#reply4",
//         topic_id: 1142655,
//         relative_time: "5 天前",
//         reply_count: 4,
//         last_replier: Some(
//             "SwaggyMacro",
//         ),
//     },
//     VXNA {
//         username: "tongnixcv",
//         avatar_url: "https://cdn.v2ex.com/gravatar/8885bbed83f07878204416810cdd94de?s=48&d=retro",
//         profile_url: "/member/tongnixcv",
//         title: "申请收录个人博客 iyeslogo.com",
//         topic_url: "/t/1142584#reply2",
//         topic_id: 1142584,
//         relative_time: "5 天前",
//         reply_count: 2,
//         last_replier: Some(
//             "tongnixcv",
//         ),
//     },
//     VXNA {
//         username: "cscnk52",
//         avatar_url: "https://cdn.v2ex.com/gravatar/72a6f7e522a2e8e887cdcb282e1d71b6?s=48&d=retro",
//         profile_url: "/member/cscnk52",
//         title: "申请收录个人博客： Cason Kervis's blog",
//         topic_url: "/t/1142152#reply2",
//         topic_id: 1142152,
//         relative_time: "7 天前",
//         reply_count: 2,
//         last_replier: Some(
//             "Livid",
//         ),
//     },
//     VXNA {
//         username: "FlyingClive",
//         avatar_url: "https://cdn.v2ex.com/avatar/b943/1a51/101218_xlarge.png?m=1728089859",
//         profile_url: "/member/FlyingClive",
//         title: "申请收录个人网站： https://www.linggandianbo.com",
//         topic_url: "/t/1141954#reply1",
//         topic_id: 1141954,
//         relative_time: "5 天前",
//         reply_count: 1,
//         last_replier: Some(
//             "Livid",
//         ),
//     },
// ]
