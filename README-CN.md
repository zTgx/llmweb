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
