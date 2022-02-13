use std::env;
use std::fs;
use std::io;

mod decoder;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();
    let path = args.get(1);
    if path == None {
        eprintln!("need source file path");
        return Ok(());
    }
    let path = path.unwrap();
    // パース
    let instructions = parser::parse(io::BufReader::new(fs::File::open(path)?))?;
    for ins in instructions {
        let decoded = decoder::decode(ins);
        if let Err(e) = decoded {
            eprintln!("{}", e);
            return Ok(());
        }
        let decoded = decoded.unwrap();
        println!("{}", decoded);
    }
    Ok(())
}
