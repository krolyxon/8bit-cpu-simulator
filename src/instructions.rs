#[repr(u8)]
pub enum Instruction {
    MOV_RI = 0x01,
    MOV_RR = 0x08,
    ADD = 0x02,
    SUB = 0x03,
    JMP = 0x04,
    JZ = 0x05,
    JNZ = 0x06,
    CMP_RI = 0x07,
    CMP_RR = 0x09,
    HLT = 0xFF,
}

impl Instruction {
    pub  fn opcode_name(op: u8) -> &'static str{
        match op {
            0x01 | 0x08 => "MOV",
            0x02 => "ADD",
            0x03 => "SUB",
            0x04 => "JMP",
            0x05 => "JZ",
            0x06 => "JNZ",
            0x07 | 0x09 => "CMP",
            0xFF => "HLT",
            _ => "???",
        }
    }
}
