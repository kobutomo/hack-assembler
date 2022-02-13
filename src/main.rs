use std::env;
use std::fs;
use std::io;

mod decoder;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();
    let path = &args[0];
    let instructions = parser::parse(io::BufReader::new(fs::File::open(path)?))?;
    for ins in instructions {
        // let ml = decoder::decode(ins);
    }
    Ok(())
}
