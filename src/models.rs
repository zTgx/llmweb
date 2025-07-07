use {
    crate::error::{LlmWebError, Result},
    genai::{
        Client,
        chat::{ChatMessage, ChatOptions, ChatRequest, ChatResponseFormat, JsonSpec},
    },
    serde_json::{Value, json},
};

pub const SYSTEM_PROMPT: &str = "You are a structured information extraction assistant. Please extract JSON from the HTML page.\nStrictly output the JSON structure as specified above. Use null for missing fields.";

#[macro_export]
macro_rules! strip_markdown_backticks {
    ($text:expr) => {{
        let trimmed = $text.trim();
        let re_leading = regex::Regex::new(r"(?i)^```[\w]*\s*").unwrap();
        let re_trailing = regex::Regex::new(r"(?i)\s*```$").unwrap();
        let without_leading = re_leading.replace(trimmed, "");
        let without_trailing = re_trailing.replace(&without_leading, "");
        without_trailing.to_string()
    }};
}

pub struct LLMClient {
    client: Client,
    pub model: String,
}

impl LLMClient {
    pub fn new(model: &str) -> Self {
        Self {
            client: Client::default(),
            model: model.to_string(),
        }
    }

    pub async fn completion(&self, user_content: &str, scheme: Value) -> Result<String> {
        let op = ChatOptions::default().with_response_format(ChatResponseFormat::JsonSpec(
            JsonSpec::new("LlmWeb", json!(scheme)),
        ));
        let chat_req = ChatRequest::new(vec![
            ChatMessage::system(SYSTEM_PROMPT),
            ChatMessage::user(user_content),
        ]);

        let response = self
            .client
            .exec_chat(&self.model, chat_req, Some(&op))
            .await
            .map_err(|e| LlmWebError::ModelClient(format!("{e}")))?;

        let json_str = response
            .content
            .ok_or_else(|| LlmWebError::ModelClient("No content in response".to_string()))?
            .text_into_string();

        if let Some(json_str) = json_str {
            let text = strip_markdown_backticks!(json_str);
            return Ok(text);
        }

        Err(LlmWebError::ModelClient(
            "Content to string error".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use regex;

    #[test]
    fn test_strip_markdown_backticks() {
        // No backticks
        let s1 = "hello";
        assert_eq!(strip_markdown_backticks!(s1), "hello");

        // Simple fenced code block
        let s2 = "```json\n{\"a\":1}\n```";
        assert_eq!(strip_markdown_backticks!(s2), "{\"a\":1}");

        // Fenced code block with language
        let s3 = "```rust\nlet x = 1;\n```";
        assert_eq!(strip_markdown_backticks!(s3), "let x = 1;");

        // Leading/trailing whitespace
        let s4 = "   ```json\n{\"b\":2}\n```   ";
        assert_eq!(strip_markdown_backticks!(s4), "{\"b\":2}");

        // Only backticks
        let s5 = "```";
        assert_eq!(strip_markdown_backticks!(s5), "");

        // No code block, just text with backticks inside
        let s6 = "some `inline` code";
        assert_eq!(strip_markdown_backticks!(s6), "some `inline` code");
    }

    #[test]
    fn test_strip_markdown_backticks_large_json() {
        let json_str = r###"
```json
{
  "top": [
    {
      "title": "Ask HN: Any resources for finding non-smart appliances?",
      "points": 65,
      "by": "everyone",
      "comments_url": "item?id=44488810"
    },
    {
      "title": "What every programmer should know about how CPUs work [video]",
      "points": 76,
      "by": "bschne",
      "comments_url": "item?id=44458896"
    },
    {
      "title": "Bitchat - A decentralized messaging app that works over Bluetooth mesh networks",
      "points": 0,
      "by": null,
      "comments_url": null
    }
  ]
}
```
"###;

        let expected = r#"{
  "top": [
    {
      "title": "Ask HN: Any resources for finding non-smart appliances?",
      "points": 65,
      "by": "everyone",
      "comments_url": "item?id=44488810"
    },
    {
      "title": "What every programmer should know about how CPUs work [video]",
      "points": 76,
      "by": "bschne",
      "comments_url": "item?id=44458896"
    },
    {
      "title": "Bitchat - A decentralized messaging app that works over Bluetooth mesh networks",
      "points": 0,
      "by": null,
      "comments_url": null
    }
  ]
}"#;

        assert_eq!(strip_markdown_backticks!(json_str), expected);
    }
}
