<div align="center">

# llmweb   
**Empower the Web with Rust & AI**

[![Version](https://img.shields.io/crates/v/llmweb)](https://crates.io/crates/llmweb)
[![Downloads](https://img.shields.io/crates/d/llmweb?logo=rust)](https://crates.io/crates/llmweb)
[![License](https://img.shields.io/crates/l/llmweb)](LICENSE)
[![Documentation](https://img.shields.io/docsrs/llmweb)](https://docs.rs/llmweb)

</div>

> [!IMPORTANT]
> ***This project is under active development and APIs may change.***

## ✨ Key Features
 
- **🤖 Schema-Driven Extraction**
- **🌐 Multi-Provider LLM Support**
- **⚡ High-Performance & Async**
- **💻 Simple & Powerful CLI** 
- **🦀 Rust-Powered Reliability**
- **📄 Streaming**


## Installation
Add to your `Cargo.toml`:
```toml
[dependencies]
llmweb = "0.1"
```

1. Configure API Key(different providers choose one):
```bash
export OPENAI_API_KEY="sk-your-key-here"         # OpenAI
export ANTHROPIC_API_KEY="sk-ant-your-key"       # Claude
export GEMINI_API_KEY="your-google-key"          # Gemini
export COHERE_API_KEY="your-cohere-key"          # Cohere
export GROQ_API_KEY="gsk-your-key"               # Groq
export XAI_API_KEY="xai-your-key"               # xAI
export DEEPSEEK_API_KEY="your-deepseek-key"     # DeepSeek
# Ollama typically requires no API key for local usage
```

2. Pick the model you want to use:
```rust
let model = "gemini-2.0-flash";
```

3. Create `LlmWeb` instance with the model:
```rust
let llmweb = LlmWeb::new(model);
```

## Example - HN
```rust
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
```

## Example - V2EX
```rust
use llmweb::LlmWeb;
use serde::{Deserialize, Serialize};

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
    let schema_str = include_str!("../schemas/v2ex_schema.json");

    let llmweb = LlmWeb::new("gemini-2.0-flash");
    let structed_value: Vec<VXNA> = llmweb
        .completion_from_schema_str("https://v2ex.com/go/vxna", schema_str)
        .await
        .unwrap();
    println!("{:#?}", structed_value);
}
```

## Streaming
```rust
#[tokio::main]
async fn main() {
    // Load the schema from an external file as a string.
    let schema_str = include_str!("../schemas/v2ex_schema.json");
    let schema: Value = serde_json::from_str(schema_str).unwrap();

    let structed_value: Vec<VXNA> = LlmWeb::new("gemini-2.0-flash")
        .completion_stream("https://v2ex.com/go/vxna", schema)
        .await
        .unwrap();
    println!("{:#?}", structed_value);
}
```

## Cli
```bash
# Run the CLI
./target/debug/llmweb-cli --schema-file schemas/hn_schema.json https://news.ycombinator.com
```

## Examples
More examples can be found in the [Examples](./examples/) directory.

## Schemas
More schemas can be found in the [Schemas](./schemas/) directory.

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=zTgx/llmweb&type=Date)](https://www.star-history.com/#zTgx/llmweb&Date)

## Contributing

We welcome contributions! Please see our CONTRIBUTING.md for more details on how to get started.

## License

This project is licensed under the MIT License - see the `LICENSE` file for details.