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

## 🚀 工作原理

1. **启动无头浏览器**  
   访问目标 URL，执行 JavaScript，获取最终呈现的 HTML 内容

2. **内容提取**  
   将页面的 HTML 文本提取出来

3. **LLM 处理**  
   将 HTML 内容连同你定义的 Schema 一起发送给大模型

4. **返回结构化数据**  
   大模型会根据你的要求，智能地从非结构化的 HTML 中提取信息，并返回严格符合 Schema 格式的 JSON 数据

## ✨ 核心特性

- **网页"API化"**  
  无需编写繁琐的 CSS 选择器或 XPath，用自然语言描述数据结构即可

- **模式优先 (Schema-First)**  
  通过 `serde_json` 定义输出格式，保证数据准确性和一致性

- **异步核心**  
  基于 `tokio` 构建，性能卓越，轻松应对高并发场景

- **Rust 赋能**  
  兼具内存安全和高性能的优点

- **命令行工具 (CLI)**  
  开箱即用的终端工具，适合集成到自动化脚本中

## 💡 应用场景

- 从新闻网站（如 Hacker News）提取头条新闻列表
- 监控电商网站的商品价格和库存
- 聚合来自不同博客或论坛的帖子
- 将网页内容转化为结构化数据源

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