use std::env;
use std::fs;
use std::io;

mod decoder;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();
    let path = &args[1];
    let instructions = parser::parse(io::BufReader::new(fs::File::open(path)?))?;
    for ins in instructions {
        let decoded = decoder::decode(ins)?;
        println!("{}", decoded);
    }
    Ok(())
}
