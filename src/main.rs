mod cli;
use std::{
    io::{Read as _, Write as _, stdin, stdout},
    net::TcpStream,
};

use clap::Parser;
use cli::CliArgs;
fn main() {
    let args = CliArgs::parse();
    let mut stream = TcpStream::connect(args.root_url).unwrap();
    loop {
        print!("{}", args.input_prompt);
        stdout().flush().unwrap();
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        stream.write_all(user_input.as_bytes()).unwrap();
        let mut url_output_buffer = Vec::new();
        stream.read_to_end(&mut url_output_buffer).unwrap();
        let url_output_buffer = String::from_utf8_lossy(&url_output_buffer);
        println!("{}{}", args.output_prompt, url_output_buffer);
    }
}
// del netcat.exe && move ./target/debug/netcat.exe netcat.exe
