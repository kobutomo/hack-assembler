use std::env;
use std::fs;
use std::io;
use std::io::Write;

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
    // パース
    let instructions = parser::parse(io::BufReader::new(fs::File::open(path)?), &mut symbol_table)?;
    let mut writer = io::BufWriter::new(fs::File::create("out.hack")?);
    for ins in instructions {
        let decoded = decoder::decode(ins, &mut symbol_table);
        if let Err(e) = decoded {
            eprintln!("{}", e);
            return Ok(());
        }
        let decoded = decoded.unwrap();
        writer.write((decoded + "\n").as_bytes())?;
    }
    writer.flush()?;
    Ok(())
}
