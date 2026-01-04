mod assembler;
mod cpu;
mod instructions;
mod memory;

use cpu::CPU;
use memory::Memory;
use clap::Parser;
use crate::assembler::assembler;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,
}


fn main() {
    let args = Args::parse();

    let mut cpu = CPU::default();
    let mut mem = Memory::new();

    let asm = std::fs::read_to_string(args.filename).unwrap();
    let program = assembler(&asm);

    for (i, byte) in program.iter().enumerate() {
        mem.write(i as u16, *byte);
    }

    while !cpu.halted {
        cpu.step(&mut mem);
        println!("{:?}", cpu);
    }
}
