mod context;
mod output;
mod parser;

use crate::context::Context;
use crate::output::output_http;
use crate::parser::parse_http_file;

use clap::Parser;
use reqwest::blocking::Client;

#[derive(Clone, Debug)]
struct KeyValue(String, String);

/// A small and fast CLI for testing REST APIs
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to a HTTP request file
    #[arg(short, long, name = "FILE")]
    file: String,

    /// Parameters to be passed to the request
    /// in the form of key=value
    #[arg(short, long, name = "key=value", value_parser = parse_param)]
    param: Vec<KeyValue>,

    /// Show debug information
    #[arg(short, long)]
    debug: bool,
}

fn parse_param(s: &str) -> anyhow::Result<KeyValue> {
    let parts: Vec<&str> = s.split('=').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid key=value pair: {}", s);
    }
    Ok(KeyValue(parts[0].to_string(), parts[1].to_string()))
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.debug {
        // Print backtrace when a panic occurs
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    let mut context = Context::from_args(args.param);

    let client = Client::new();
    let request = parse_http_file(&mut context, &client, &args.file)?;
    let response = request.send()?;

    output_http(response)
}
