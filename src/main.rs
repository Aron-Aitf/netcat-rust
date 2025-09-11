mod cli;
use clap::Parser;
use cli::CliArgs;
fn main() {
    let args = CliArgs::parse();
    println!("{:#?}", args);
}
// del netcat.exe && move ./target/debug/netcat.exe netcat.exe