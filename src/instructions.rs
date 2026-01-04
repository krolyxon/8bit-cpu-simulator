#[repr(u8)]
pub enum Instruction {
    MOV = 0x01,
    ADD = 0x02,
    SUB = 0x03,
    JMP = 0x04,
    JZ = 0x05,
    JNZ = 0x06,
    HLT = 0xFF,
}
