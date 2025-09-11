use std::str::FromStr;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct CliArgs {
    pub root_url: String,
    #[arg(short, long = "input_prompt", default_value_t = String::from_str(">").unwrap())]
    pub input_prompt: String,
    #[arg(short, long = "output_prompt", default_value_t = String::from_str("").unwrap())]
    pub output_prompt: String,
    #[arg(short, long, default_value_t = 1024)]
    pub buffer_size: u16
}
