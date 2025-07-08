# Contributing to llmweb.rs

First off, thank you for considering contributing to `llmweb`! It's people like you that make open source such a great community. We welcome any form of contribution, from reporting bugs and suggesting features to writing code and improving documentation.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## How Can I Contribute?

### Reporting Bugs

If you find a bug, please ensure the bug was not already reported by searching on GitHub under Issues. If you're unable to find an open issue addressing the problem, open a new one. Be sure to include a **title and clear description**, as much relevant information as possible, and a **code sample** or an **executable test case** demonstrating the expected behavior that is not occurring.

### Suggesting Enhancements

If you have an idea for an enhancement, feel free to open an issue to discuss it. This allows us to coordinate efforts and ensure the feature aligns with the project's goals.

### Pull Requests

We love pull requests! To submit one, please follow these steps:

1.  **Fork the repository** and create your branch from `main`.
2.  **Set up your development environment**. You'll need a recent version of Rust. You can install it via rustup.
3.  **Make your changes**. Please ensure your code adheres to the existing style.
4.  **Add or update tests** for your changes. We value well-tested code.
5.  **Ensure the test suite passes** by running `cargo test`.
6.  **Format your code** with `cargo fmt`.
7.  **Lint your code** with `cargo clippy -- -D warnings`.
8.  **Commit your changes** with a clear and descriptive commit message.
9.  **Push to your fork** and submit a pull request to the `main` branch of the `llmweb` repository.

## Development Setup

Here’s a quick guide to get your development environment up and running:

```bash
# 1. Clone your fork
git clone https://github.com/YOUR_USERNAME/llmweb.git
cd llmweb

# 2. Set up your API key for testing
# Make sure you have a .env file or export the variable
export GEMINI_API_KEY="your_api_key_here"

# 3. Build the project
cargo build

# 4. Run tests to ensure everything is working
cargo test
```

## Style Guide

We follow the standard Rust formatting guidelines. Please run `cargo fmt` before committing your changes to ensure your code is formatted correctly.

Thank you again for your contribution!

---

*Note: This is a living document. If you have suggestions for improving it, please open an issue or a pull request.*