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
