use std::env;
use std::fs;
use std::io;

mod decoder;
mod parser;
mod symbol_table;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();
    let path = args.get(1);
    if path == None {
        eprintln!("need source file path");
        return Ok(());
    }
    let path = path.unwrap();
    let mut symbol_table = symbol_table::new();
    println!("{:?}", symbol_table);
    // パース
    let instructions = parser::parse(io::BufReader::new(fs::File::open(path)?), &mut symbol_table)?;
    for ins in instructions {
        let decoded = decoder::decode(ins, &mut symbol_table);
        if let Err(e) = decoded {
            eprintln!("{}", e);
            return Ok(());
        }
        let decoded = decoded.unwrap();
        println!("{}", decoded);
    }
    Ok(())
}
