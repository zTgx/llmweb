use {
    clap::Parser,
    llmweb::{LlmWeb, error::LlmWebError},
    serde_json::{Value, from_str},
    std::fs,
    std::path::PathBuf,
};

/// A CLI tool to extract structured data from webpages using LLMs.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The URL of the webpage to process.
    #[arg()]
    url: String,

    /// Path to the JSON file containing the schema for data extraction.
    #[arg(short, long)]
    schema_file: PathBuf,

    /// The name of the LLM model to use.
    #[arg(short, long, default_value = "gemini-1.5-flash")]
    model: String,
}

#[tokio::main]
async fn main() -> std::result::Result<(), LlmWebError> {
    let args = Args::parse();

    // 1. Read the schema from the specified file.
    let schema_str =
        fs::read_to_string(&args.schema_file).map_err(|e| LlmWebError::Io(e.to_string()))?;
    let schema: Value = from_str(&schema_str)?;

    // 2. Initialize the LlmWeb client.
    let llmweb = LlmWeb::new(&args.model);

    eprintln!("Processing URL: {}", &args.url);
    eprintln!("Using model: {}", &args.model);

    // 3. Perform the completion, deserializing into a generic serde_json::Value.
    let result: Value = llmweb.completion(&args.url, schema).await?;

    // 4. Print the result as pretty-printed JSON to standard output.
    let pretty_json = serde_json::to_string_pretty(&result)?;

    println!("{pretty_json}");

    Ok(())
}
