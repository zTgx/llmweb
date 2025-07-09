//! # llmweb.rs
//! **Powering the Web with Rust & LLMs**
//!
//! `llmweb` is a Rust library designed to seamlessly integrate Large Language Models (LLMs)
//! with web content. It allows you to fetch a webpage, extract its content, and then
//! use an LLM to get structured data from it based on a provided schema.
//!
//! ## Features
//! - 🚀 Seamless integration with major LLM APIs (currently Gemini).
//! - ✨ Automatic structured data extraction from web content.
//! - 🔧 Schema-first approach for precise data formatting using `serde_json::Value`.
//! - ⚡ Async-first design for high performance.
//!
//! ## Example
//!
//! Here's a quick example of how to use `llmweb` to extract stories from Hacker News:
//!
//! ```rust,no_run
//! use llmweb::{LlmWeb, error::LlmWebError};
//! use serde::{Deserialize, Serialize};
//! use serde_json::json;
//!
//! #[derive(Debug, Serialize, Deserialize)]
//! struct Story {
//!     title: String,
//!     points: f32,
//!     by: Option<String>,
//!     comments_url: Option<String>,
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), LlmWebError> {
//!     // 1. Define the schema for the data you want to extract.
//!     let schema_json = json!({
//!         "type": "array",
//!         "items": {
//!             "type": "object",
//!             "properties": {
//!                 "by": { "type": "string" },
//!                 "comments_url": { "type": "string" },
//!                 "points": { "type": "number" },
//!                 "title": { "type": "string" }
//!             },
//!             "required": ["by", "comments_url", "points", "title"]
//!         }
//!     });
//!
//!     // 2. Create an LlmWeb instance with the desired model.
//!     //    Make sure you have the GEMINI_API_KEY environment variable set.
//!     let llmweb = LlmWeb::new("gemini-1.5-flash");
//!
//!     // 3. Call completion with the URL and schema.
//!     let structured_value: Vec<Story> = llmweb
//!         .completion("https://news.ycombinator.com", schema_json)
//!         .await?;
//!
//!     // 4. Print the result.
//!     println!("{:#?}", structured_value);
//!
//!     Ok(())
//! }
//! ```
use {
    crate::{browser::LlmWebBrower, error::Result},
    serde::de::DeserializeOwned,
    std::fmt::Debug,
};

mod browser;
pub mod error;
mod models;

/// Represents the desired output format.
///
/// Note: This is currently not used but is planned for future versions.
#[derive(Debug, Clone)]
pub enum LlmWebFormat {
    /// JSON format.
    Json,
    /// YAML format.
    Yaml,
    /// Plain text format.
    Text,
}

/// The main struct for interacting with web pages and LLMs.
///
/// It holds the client for the LLM and provides methods to
/// perform completions on web content.
pub struct LlmWeb {
    client: models::LLMClient,
}

impl LlmWeb {
    /// Creates a new `LlmWeb` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the LLM model to use (e.g., "gemini-1.5-flash").
    pub fn new(name: &str) -> Self {
        Self {
            client: models::LLMClient::new(name),
        }
    }

    /// Fetches content from a URL, sends it to an LLM for processing based on a schema,
    /// and returns the structured data.
    ///
    /// This function performs the following steps:
    /// 1. Launches a headless browser.
    /// 2. Navigates to the specified URL.
    /// 3. Extracts the HTML content of the page.
    /// 4. Sends the content and a JSON schema to the configured LLM.
    /// 5. Parses the LLM's JSON response into the specified Rust type `R`.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the web page to process.
    /// * `scheme` - A serializable object representing the JSON schema for data extraction.
    ///   This is typically a `serde_json::Value`.
    ///
    /// # Errors
    ///
    /// This function can return an `LlmWebError` if any of the steps fail, such as
    /// browser errors, network issues, LLM API errors, or JSON deserialization errors.
    pub async fn completion<R>(&self, url: &str, scheme: serde_json::Value) -> Result<R>
    where
        R: DeserializeOwned + Debug,
    {
        let browser = LlmWebBrower::new().await?;
        let html = browser.run(url).await?;
        let response = self.client.completion(&html, scheme).await?;

        // The `?` operator is used here thanks to `#[from] serde_json::Error` on LlmWebError.
        let result: R = serde_json::from_str(&response)?;

        Ok(result)
    }

    /// A convenience method that accepts a schema as a string slice.
    ///
    /// This method is useful when loading a schema from a file. It parses the
    /// string into a `serde_json::Value` and then calls the main `completion` method.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the web page to process.
    /// * `schema_str` - A string slice containing the JSON schema.
    ///
    /// # Errors
    ///
    /// Returns an error if the `schema_str` is not valid JSON, or if any of the
    /// underlying operations in `completion` fail.
    pub async fn completion_from_schema_str<R>(&self, url: &str, schema_str: &str) -> Result<R>
    where
        R: DeserializeOwned + Debug,
    {
        let scheme: serde_json::Value = serde_json::from_str(schema_str)?;
        self.completion(url, scheme).await
    }
}
