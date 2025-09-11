use std::str::FromStr;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct CliArgs {
    root_url: String,
    #[arg(short, long = "input-prompt", default_value_t = String::from_str(">").unwrap())]
    input_prompt: String,
    #[arg(short, long = "output-prompt", default_value_t = String::from_str("").unwrap())]
    output_prompt: String,
}
