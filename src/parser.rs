use super::decoder;

use std::{
    fs,
    io::{self, BufRead},
};

#[derive(PartialEq)]
#[allow(dead_code)]
enum InstructionTypes {
    A = 0,
    C = 1,
    L = 2,
}

pub fn parse(file_reader: io::BufReader<fs::File>) -> Result<Vec<decoder::Instruction>, io::Error> {
    let mut ret: Vec<decoder::Instruction> = Vec::new();
    for line in file_reader.lines() {
        let res = match line {
            Ok(l) => l,
            Err(e) => return Err(e),
        };
        let i_type = match get_type(&res) {
            Some(t) => t,
            None => continue,
        };

        match i_type {
            InstructionTypes::A => {
                ret.push(decoder::Instruction::A(decoder::InsA(
                    (&res[1..]).to_string(),
                )));
            }
            InstructionTypes::C => {
                let insc = destruct_insc(res);
                ret.push(decoder::Instruction::C(insc));
            }
            InstructionTypes::L => { /* TODO: symbol table 作成 */ }
        }
    }
    Ok(ret)
}

fn get_type(row: &String) -> Option<InstructionTypes> {
    match row.chars().nth(0) {
        Some('@') => Some(InstructionTypes::A),
        Some('(') => Some(InstructionTypes::L),
        Some(_) => Some(InstructionTypes::C),
        None => None,
    }
}

fn destruct_insc(mut line: String) -> decoder::InsC {
    let dest = {
        if line.find("=") != None {
            let dest = line.split("=").nth(0).unwrap().to_string();
            line = line.split("=").nth(1).unwrap().to_string();
            Some(dest)
        } else {
            None
        }
    };
    let (comp, jump) = {
        if line.find(";") != None {
            let mut sp = line.split(";");
            (
                sp.nth(0).unwrap().to_string(),
                Some(sp.nth(0).unwrap().to_string()),
            )
        } else {
            (line, None)
        }
    };
    decoder::InsC { dest, comp, jump }
}
