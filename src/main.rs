mod cli;
use std::{
    error::Error,
    io::{Read as _, Write as _, stdin, stdout},
    net::TcpStream,
};

use clap::Parser;
use cli::CliArgs;

fn main() -> Result<(), Box<dyn Error>> {
    let args = CliArgs::parse();
    let mut stream = TcpStream::connect(args.root_url)?;
    loop {
        print!("{}", args.input_prompt);
        stdout().flush()?;

        let mut user_input = String::new();
        stdin().read_line(&mut user_input)?;

        stream.write_all(user_input.as_bytes())?;

        let mut url_output_buffer = String::new();
        
        stream.read_to_string(&mut url_output_buffer)?;

        print!("{}{}", args.output_prompt, url_output_buffer);
        stdout().flush()?;
    }
}

// cargo build && del netcat.exe && move ./target/debug/netcat.exe netcat.exe
