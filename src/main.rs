mod output;
mod parser;

use crate::output::output_http;
use crate::parser::parse_http_file;

use clap::Parser;
use reqwest::blocking::Client;

/// A small and fast CLI for testing REST APIs
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to a HTTP request file
    #[arg(short, long, name = "FILE")]
    path: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let client = Client::new();
    let request = parse_http_file(&client, &args.path)?;
    let response = request.send();

    output_http(response)
}
