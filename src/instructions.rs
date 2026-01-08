#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    MOV_RI = 0x01,
    MOV_RR = 0x08,
    ADD_RR = 0x02,
    ADD_RI = 0x0A,
    SUB_RR = 0x03,
    SUB_RI = 0x0B,
    JMP = 0x04,
    JZ = 0x05,
    JNZ = 0x06,
    CMP_RI = 0x07,
    CMP_RR = 0x09,
    MUL = 0x0C,
    DIV = 0x0D,
    CALL = 0x0E,
    RET = 0x0F,
    HLT = 0xFF,
}

impl Instruction {
    pub  fn opcode_name(op: u8) -> &'static str{
        match op {
            0x01 | 0x08 => "MOV",
            0x02 | 0x0A => "ADD",
            0x03 | 0x0B => "SUB",
            0x04 => "JMP",
            0x05 => "JZ",
            0x06 => "JNZ",
            0x07 | 0x09 => "CMP",
            0x0C => "MUL",
            0x0D => "DIV",
            0x0E => "CALL",
            0x0F => "RET",
            0xFF => "HLT",

            _ => "???",
        }
    }
}
