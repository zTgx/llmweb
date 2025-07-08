<div align="center">

# llmweb   
<summary><a href="README.md">English</a> | 中文</summary>

**用 Rust 和 LLM，给任意网页套上一个 API**

[![Version](https://img.shields.io/crates/v/llmweb)](https://crates.io/crates/llmweb)
[![Downloads](https://img.shields.io/crates/d/llmweb?logo=rust)](https://crates.io/crates/llmweb)
[![License](https://img.shields.io/crates/l/llmweb)](LICENSE)
[![Documentation](https://img.shields.io/docsrs/llmweb)](https://docs.rs/llmweb)

</div>

`llmweb` 是一个 Rust 库，它将无头浏览器（Headless Chrome）、Rust 的高性能异步能力与大语言模型（LLM）的强大理解力结合在一起。你只需要提供一个网址和一个你想要的数据结构（JSON Schema），`llmweb` 就能自动访问网页，像人一样"阅读"页面内容，并为你返回结构化的、干净的数据。

## ✨ 核心特性

- **🤖 模式驱动提取 (Schema-Driven Extraction):** 使用 JSON Schema 定义你所需的数据结构。`llmweb` 会指示大语言模型（LLM）精确地提取和格式化数据以满足你的需求，确保输出干净、可预测且类型安全。 
- **🌐 多模型供应商支持 (Multi-Provider LLM Support):** 不再局限于单一的生态系统。在主流的 LLM 供应商（如 OpenAI, Google (Gemini), Anthropic (Claude), Cohere 等）之间无缝切换。只需设置相应的 API 密钥和模型名称。
- **📄 类人网页交互 (Human-Like Web Interaction):** 通过无头浏览器驱动，`llmweb` 能像用户一样“看到”网页。这使其能够理解动态加载的内容、复杂的布局以及传统抓取工具会遗漏的信息。
- **⚡ 高性能异步核心 (High-Performance & Async):** 基于 `tokio` 构建，`llmweb` 从核心上就是异步的，能够高效地并发处理多个网页抓取任务。
- **💻 简单强大的命令行工具 (Simple & Powerful CLI):** 内置开箱即用的命令行工具。轻松地将 `llmweb` 集成到你的 shell 脚本和数据管道中，只需传入一个 URL 和一个 schema 文件，即可在终端直接获取结构化数据。
- **🦀 Rust 赋能的可靠性 (Rust-Powered Reliability):** 享受 Rust 带来的内存安全、高性能和稳健性，让你的数据提取工具既可靠又高效。

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

## License

This project is licensed under the MIT License - see the `LICENSE` file for details.