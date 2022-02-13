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
        _ => Ok(format!("TODO")),
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
    let mut ret = 1 << 16;
    ret += int;
    return Ok(format!("{:b}", ret));
}
