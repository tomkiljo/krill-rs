mod output;
mod parser;

use crate::output::output_http;
use crate::parser::parse_http_file;

use clap::Parser;
use reqwest::blocking::Client;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();

    let client = Client::new();
    let request = parse_http_file(&client, &args.path);
    let response = request.send();

    output_http(response);
}
