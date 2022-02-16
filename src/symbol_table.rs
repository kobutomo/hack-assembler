use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    pub next_address: u16,
    labels: HashMap<String, u16>,
}

pub fn new() -> SymbolTable {
    let mut labels: HashMap<String, u16> = HashMap::new();
    labels.insert("SP".to_string(), 0x0000);
    labels.insert("LCL".to_string(), 0x0001);
    labels.insert("ARG".to_string(), 0x0002);
    labels.insert("THIS".to_string(), 0x0003);
    labels.insert("THAT".to_string(), 0x0004);
    labels.insert("R0".to_string(), 0x0000);
    labels.insert("R1".to_string(), 0x0001);
    labels.insert("R2".to_string(), 0x0002);
    labels.insert("R3".to_string(), 0x0003);
    labels.insert("R4".to_string(), 0x0004);
    labels.insert("R5".to_string(), 0x0005);
    labels.insert("R6".to_string(), 0x0006);
    labels.insert("R7".to_string(), 0x0007);
    labels.insert("R8".to_string(), 0x0008);
    labels.insert("R9".to_string(), 0x0009);
    labels.insert("R10".to_string(), 0x000a);
    labels.insert("R11".to_string(), 0x000b);
    labels.insert("R12".to_string(), 0x000c);
    labels.insert("R13".to_string(), 0x000d);
    labels.insert("R14".to_string(), 0x000e);
    labels.insert("R15".to_string(), 0x000f);
    labels.insert("SCREEN".to_string(), 0x4000);
    labels.insert("KBD".to_string(), 0x6000);
    SymbolTable {
        next_address: 0,
        labels,
    }
}

impl SymbolTable {
    pub fn add_label(&mut self, label: String, addr: u16) {
        self.labels.insert(label, addr);
    }

    pub fn get_address(&self, label: &String) -> Option<&u16> {
        self.labels.get(label)
    }

    pub fn address_increment(&mut self) {
        self.next_address += 1;
    }
}
