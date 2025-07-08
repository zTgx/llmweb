<div align="center">

# llmweb.rs   
**Powering the Web with Rust & LLMs**

[![Version](https://img.shields.io/crates/v/llmweb)](https://crates.io/crates/llmweb)
[![Downloads](https://img.shields.io/crates/d/llmweb?logo=rust)](https://crates.io/crates/llmweb)
[![License](https://img.shields.io/crates/l/llmweb)](LICENSE)
[![Documentation](https://img.shields.io/docsrs/llmweb)](https://docs.rs/llmweb)

</div>

## Features
- 🚀 Seamless integration with major LLM APIs (Gemini, OpenAI, etc.)
- ✨ Automatic structured data extraction from web content
- 🔧 Schema-first approach for precise data formatting
- ⚡ Async-first design for high performance
- 🌐 Web content extraction and summarization
- 💬 Multi-modal input support (text, images, etc.)
- 🛡️ Robust error handling and retry mechanisms  
- 💻 CLI for quick and easy interaction
- 📚 Comprehensive documentation and examples

## Installation
Add to your `Cargo.toml`:
```toml
[dependencies]
llmweb = "0.1"
tokio = { version = "1.0", features = ["full"] }
```

1. Configure API Key(GEMINI_API_KEY for example)
```bash
export GEMINI_API_KEY="your_api_key_here"
```

2. Create `LlmWeb` instance with corresponding model name
```rust
let llmweb = LlmWeb::new("gemini-2.0-flash");
```

## Example
```rust
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
```

## CLI
```bash
# Make sure your GEMINI_API_KEY is set
export GEMINI_API_KEY="your_api_key_here"

# Build the project
cargo build

# Run the CLI
./target/debug/llmweb-cli --schema-file schemas/hn_schema.json https://news.ycombinator.com
```

## Output
```bash
[
    Story {
        title: "François Chollet: The Arc Prize and How We Get to AGI [video]",
        points: 43.0,
        by: Some(
            "sandslash",
        ),
        comments_url: Some(
            "item?id=44455175",
        ),
    },
    Story {
        title: "When Figma starts designing us",
        points: 24.0,
        by: Some(
            "bravomartin",
        ),
        comments_url: Some(
            "item?id=44479502",
        ),
    },
    ...
]
```

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=zTgx/llmweb&type=Date)](https://www.star-history.com/#zTgx/llmweb&Date)

## Contributing

We welcome contributions! Please see our CONTRIBUTING.md for more details on how to get started.

## License

This project is licensed under the MIT License - see the `LICENSE` file for details.