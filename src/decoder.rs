#[macro_use]
mod decode_error;

const ADDRESS_MAX: i32 = 65535;

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

pub fn decode(ins: Instruction) -> Result<String, decode_error::DecodeError> {
    match ins {
        Instruction::A(ins) => decode_a(ins),
        Instruction::C(ins) => decode_c(ins),
    }
}

fn decode_a(ins: InsA) -> Result<String, decode_error::DecodeError> {
    let InsA(raw) = ins;
    let int = raw.parse::<i32>();
    if let Err(_) = int {
        return Err(decode_err!("invalid instruction found"));
    }
    let int = int.unwrap();
    if int > ADDRESS_MAX {
        return Err(decode_err!("memory address exceed"));
    }
    let mut ret = 1 << 15;
    ret += int;
    return Ok(format!("{:b}", ret));
}

fn decode_c(ins: InsC) -> Result<String, decode_error::DecodeError> {
    let InsC { comp, dest, jump } = ins;
    let decoded_comp = decode_comp(comp);
    if decoded_comp == "" {
        return Err(decode_err!("invalid comp"));
    }
    let decoded_dest = decode_dest(dest);
    if decoded_dest == "" {
        return Err(decode_err!("invalid dest"));
    }
    let decoded_jump = decode_jump(jump);
    if decoded_jump == "" {
        return Err(decode_err!("invalid jump"));
    }
    // C 命令の上位 3bit はすべて 1
    let ret = "111".to_string() + decoded_comp + decoded_dest + decoded_jump;
    return Ok(ret);
}

fn decode_comp(comp: String) -> &'static str {
    match &comp[..] {
        "0" => "1101010",
        "1" => "1101010",
        "-1" => "1101010",
        "D" => "1101010",
        "A" => "1101010",
        "!D" => "1101010",
        "!A" => "1101010",
        "-D" => "1101010",
        "-A" => "1101010",
        "D+1" => "1101010",
        "A+1" => "1101010",
        "D-1" => "1101010",
        "A-1" => "1101010",
        "D+A" => "1101010",
        "D-A" => "1101010",
        "A-D" => "1101010",
        "D&A" => "1101010",
        "D|A" => "1101010",
        "M" => "0110000",
        "!M" => "0110001",
        "-M" => "0110011",
        "M+1" => "0110111",
        "M-1" => "0110010",
        "D+M" => "0000010",
        "D-M" => "0010011",
        "M-D" => "0000111",
        "D&M" => "0000000",
        "D|M" => "0010101",
        _ => "",
    }
}

fn decode_dest(dest: Option<String>) -> &'static str {
    if dest == None {
        return "000";
    }
    let dest = dest.unwrap();
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

fn decode_jump(jump: Option<String>) -> &'static str {
    if jump == None {
        return "000";
    }
    let jump = jump.unwrap();
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
