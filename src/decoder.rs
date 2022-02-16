#[macro_use]
mod decode_error;
use super::symbol_table;

const ADDRESS_MAX: u16 = 65535;

#[allow(dead_code)]
pub enum Instruction {
    A(InsA),
    C(InsC),
}

pub struct InsA(pub String);

pub struct InsC {
    pub comp: String,
    pub dest: Option<String>,
    pub jump: Option<String>,
}

pub fn decode(
    ins: Instruction,
    symbol_table: &mut symbol_table::SymbolTable,
) -> Result<String, decode_error::DecodeError> {
    match ins {
        Instruction::A(ins) => decode_a(ins, symbol_table),
        Instruction::C(ins) => decode_c(ins),
    }
}

fn decode_a(
    ins: InsA,
    symbol_table: &mut symbol_table::SymbolTable,
) -> Result<String, decode_error::DecodeError> {
    let InsA(raw) = ins;
    let addr = {
        let tmp = raw.parse::<u16>();
        if let Err(_) = tmp {
            let addr = symbol_table.get_address(&raw);
            if addr == None {
                symbol_table.add_label(raw, symbol_table.next_address);
                let tmp = symbol_table.next_address;
                symbol_table.next_address += 1;
                tmp
            } else {
                *addr.unwrap()
            }
        } else {
            tmp.unwrap()
        }
    };
    if addr > ADDRESS_MAX {
        return Err(decode_err!("memory address exceed".to_string()));
    }
    return Ok(format!("{:0>16b}", addr));
}

fn decode_c(ins: InsC) -> Result<String, decode_error::DecodeError> {
    let InsC { comp, dest, jump } = ins;
    let decoded_comp = decode_comp(&comp);
    if decoded_comp == "" {
        return Err(decode_err!(format!("invalid dest: {}", comp)));
    }
    let decoded_dest = decode_dest(&dest);
    if decoded_dest == "" {
        return Err(decode_err!(format!("invalid dest: {:?}", dest)));
    }
    let decoded_jump = decode_jump(&jump);
    if decoded_jump == "" {
        return Err(decode_err!(format!("invalid jump: {:?}", jump)));
    }
    // C 命令の上位 3bit はすべて 1
    let ret = "111".to_string() + decoded_comp + decoded_dest + decoded_jump;
    return Ok(ret);
}

fn decode_comp(comp: &String) -> &'static str {
    match &comp[..] {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "D+A" => "0000010",
        "D-A" => "0010011",
        "A-D" => "0000111",
        "D&A" => "0000000",
        "D|A" => "0010101",
        "M" => "1110000",
        "!M" => "1110001",
        "-M" => "1110011",
        "M+1" => "1110111",
        "M-1" => "1110010",
        "D+M" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" => "1000000",
        "D|M" => "1010101",
        _ => "",
    }
}

fn decode_dest(dest: &Option<String>) -> &'static str {
    if dest == &None {
        return "000";
    }
    let dest = dest.as_ref().unwrap();
    match &dest[..] {
        "M" => "001",
        "D" => "010",
        "MD" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "AMD" => "111",
        _ => "",
    }
}

fn decode_jump(jump: &Option<String>) -> &'static str {
    if jump == &None {
        return "000";
    }
    let jump = jump.as_ref().unwrap();
    match &jump[..] {
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => "",
    }
}
