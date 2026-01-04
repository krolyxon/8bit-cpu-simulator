mod cpu;
mod instructions;
mod memory;

use cpu::CPU;
use memory::Memory;
use instructions::Instruction;

fn main() {
    let mut cpu = CPU::default();
    let mut mem = Memory::new();



    // a = 10
    mem.write(0x0000, Instruction::MOV as u8);
    mem.write(0x0001, 0);
    mem.write(0x0002, 5);

    // b = 2
    mem.write(0x0003, Instruction::MOV as u8);
    mem.write(0x0004, 1);
    mem.write(0x0005, 3);

    // a = a + b
    mem.write(0x0006, Instruction::SUB as u8);
    mem.write(0x0007, 0);
    mem.write(0x0008, 1);

    // JMP to halt
    mem.write(0x0009, Instruction::JNZ as u8);
    mem.write(0x000a, 0x0f); // Low
    mem.write(0x000b, 0x00); // High

    // set b = 0
    mem.write(0x000c, Instruction::MOV as u8);
    mem.write(0x000d, 1);
    mem.write(0x000e, 0);

    // halt
    mem.write(0x000f, Instruction::HLT as u8);

    while !cpu.halted {
        let opcode = mem.read(cpu.pc);
        cpu.inc_pc();

        match opcode {
            x if x == Instruction::MOV as u8 =>  cpu.mov(&mut mem),
            x if x == Instruction::ADD as u8 => cpu.add(&mut mem),
            x if x == Instruction::SUB as u8 => cpu.sub(&mut mem),
            x if x == Instruction::JMP as u8 => cpu.jmp(&mut mem),
            x if x == Instruction::JZ  as u8 => cpu.jz(&mut mem),
            x if x == Instruction::JNZ  as u8 => cpu.jnz(&mut mem),
            x if x == Instruction::HLT as u8 => cpu.halt(),
            _ => panic!("Unknown opcode {:02X}", opcode),
        }
    }

    println!("{:#?}", cpu);

}
