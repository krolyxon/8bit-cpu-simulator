use crate::instructions::Instruction;
use crate::memory::Memory;

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct CPU {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,

    pub pc: u16,
    pub sp: u16,

    pub zero: bool,
    pub carry: bool,

    pub halted: bool,
}

impl CPU {
    fn push16(&mut self, mem: &mut Memory, val: u16) {
        self.dec_sp();
        mem.write(self.sp, (val >> 8) as u8); // high
        self.dec_sp();
        mem.write(self.sp, (val & 0xFF) as u8); // low
    }

    fn pop16(&mut self, mem: &mut Memory) -> u16 {
        let low = mem.read(self.sp) as u16;
        self.inc_sp();
        let high = mem.read(self.sp) as u16;
        self.inc_sp();

        (high << 8) | low
    }

    pub fn debug_instr(&self, mem: &Memory) {
        let opcode = mem.read(self.pc);

        println!(
            "PC={:04X} {:<3} | A={:02X} B={:02X} C={:02X} D={:02X} | Z={} C={}",
            self.pc,
            Instruction::opcode_name(opcode),
            self.a,
            self.b,
            self.c,
            self.d,
            self.zero as u8,
            self.carry as u8
        );
    }

    pub fn step(&mut self, mem: &mut Memory) {
        let opcode = mem.read(self.pc);
        self.inc_pc();

        match opcode {
            x if x == Instruction::MOV_RI as u8 => self.mov_ri(mem),
            x if x == Instruction::MOV_RR as u8 => self.mov_rr(mem),
            x if x == Instruction::ADD_RR as u8 => self.add_rr(mem),
            x if x == Instruction::ADD_RI as u8 => self.add_ri(mem),
            x if x == Instruction::SUB_RR as u8 => self.sub_rr(mem),
            x if x == Instruction::SUB_RI as u8 => self.sub_ri(mem),
            x if x == Instruction::JMP as u8 => self.jmp(mem),
            x if x == Instruction::JZ as u8 => self.jz(mem),
            x if x == Instruction::JNZ as u8 => self.jnz(mem),
            x if x == Instruction::CMP_RI as u8 => self.cmp_ri(mem),
            x if x == Instruction::CMP_RR as u8 => self.cmp_rr(mem),
            x if x == Instruction::MUL as u8 => self.mul(mem),
            x if x == Instruction::DIV as u8 => self.div(mem),
            x if x == Instruction::CALL as u8 => {
                let low  = mem.read(self.pc) as u16; self.inc_pc();
                let high = mem.read(self.pc) as u16; self.inc_pc();
                let addr = (high << 8) | low;

                let return_addr = self.pc;

                self.push16(mem, return_addr);
                self.pc = addr;

            },
            x if x == Instruction::RET as u8 => {
                let addr = self.pop16(mem);
                self.pc = addr;
            },
            x if x == Instruction::HLT as u8 => self.halt(),
            _ => panic!("Unknown opcode {:02X}", opcode),
        }
    }

    pub fn inc_pc(&mut self) {
        self.pc += 1;
    }

    pub fn inc_sp(&mut self) {
        // self.sp += 1;
        self.sp = self.sp.wrapping_add(1);
    }

    pub fn dec_sp(&mut self) {
        // self.sp -= 1;
        self.sp = self.sp.wrapping_sub(1);
    }


    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn mov_ri(&mut self, mem: &Memory) {
        let reg = mem.read(self.pc);
        self.inc_pc();
        let val = mem.read(self.pc);
        self.inc_pc();

        match reg {
            0 => self.a = val,
            1 => self.b = val,
            2 => self.c = val,
            3 => self.d = val,
            _ => {}
        }

        self.zero = val == 0;
    }

    pub fn mov_rr(&mut self, mem: &Memory) {
        let dest = mem.read(self.pc);
        self.inc_pc();
        let src= mem.read(self.pc);
        self.inc_pc();

        let val = self.get_reg(src);
        self.set_reg(dest, val);

        self.zero = val == 0;
    }


    pub fn add_rr(&mut self, mem: &Memory) {
        let dest = mem.read(self.pc);
        self.pc += 1;
        let src = mem.read(self.pc);
        self.pc += 1;

        let (result, carry) = match (dest, src) {
            (0, 0) => self.a.overflowing_add(self.a),
            (0, 1) => self.a.overflowing_add(self.b),
            (0, 2) => self.a.overflowing_add(self.c),
            (0, 3) => self.a.overflowing_add(self.d),

            (1, 0) => self.b.overflowing_add(self.a),
            (1, 1) => self.b.overflowing_add(self.b),
            (1, 2) => self.b.overflowing_add(self.c),
            (1, 3) => self.b.overflowing_add(self.d),

            (2, 0) => self.c.overflowing_add(self.a),
            (2, 1) => self.c.overflowing_add(self.b),
            (2, 2) => self.c.overflowing_add(self.c),
            (2, 3) => self.c.overflowing_add(self.d),

            (3, 0) => self.d.overflowing_add(self.a),
            (3, 1) => self.d.overflowing_add(self.b),
            (3, 2) => self.d.overflowing_add(self.c),
            (3, 3) => self.d.overflowing_add(self.d),

            _ => (0, false),
        };

        match dest {
            0 => self.a = result,
            1 => self.b = result,
            2 => self.c = result,
            3 => self.d = result,
            _ => {}
        }

        self.zero = result == 0;
        self.carry = carry;
    }
    pub fn add_ri(&mut self, mem: &Memory) {

        let dest = mem.read(self.pc);
        self.pc += 1;
        let imm = mem.read(self.pc);
        self.pc += 1;

        let (result, carry) = match dest {
            0 => self.a.overflowing_add(imm),
            1 => self.b.overflowing_add(imm),
            2 => self.c.overflowing_add(imm),
            3 => self.d.overflowing_add(imm),
            _ => (0, false),
        };

        match dest {
            0 => self.a = result,
            1 => self.b = result,
            2 => self.c = result,
            3 => self.d = result,
            _ => {}
        }

        self.zero = result == 0;
        self.carry = carry;
    }


    pub fn sub_rr(&mut self, mem: &Memory) {
        let dest = mem.read(self.pc);
        self.pc += 1;
        let src = mem.read(self.pc);
        self.pc += 1;

        let (result, borrow) = match (dest, src) {
            (0, 0) => self.a.overflowing_sub(self.a),
            (0, 1) => self.a.overflowing_sub(self.b),
            (0, 2) => self.a.overflowing_sub(self.c),
            (0, 3) => self.a.overflowing_sub(self.d),

            (1, 0) => self.b.overflowing_sub(self.a),
            (1, 1) => self.b.overflowing_sub(self.b),
            (1, 2) => self.b.overflowing_sub(self.c),
            (1, 3) => self.b.overflowing_sub(self.d),

            (2, 0) => self.c.overflowing_sub(self.a),
            (2, 1) => self.c.overflowing_sub(self.b),
            (2, 2) => self.c.overflowing_sub(self.c),
            (2, 3) => self.c.overflowing_sub(self.d),

            (3, 0) => self.d.overflowing_sub(self.a),
            (3, 1) => self.d.overflowing_sub(self.b),
            (3, 2) => self.d.overflowing_sub(self.c),
            (3, 3) => self.d.overflowing_sub(self.d),

            _ => (0, false),
        };

        match dest {
            0 => self.a = result,
            1 => self.b = result,
            2 => self.c = result,
            3 => self.d = result,
            _ => {}
        }

        self.zero = result == 0;
        self.carry = borrow;
    }

 pub fn sub_ri(&mut self, mem: &Memory) {
        let dest = mem.read(self.pc);
        self.pc += 1;
        let imm = mem.read(self.pc);
        self.pc += 1;

        let (result, borrow) = match dest {
            0 => self.a.overflowing_sub(imm),
            1 => self.b.overflowing_sub(imm),
            2 => self.c.overflowing_sub(imm),
            3 => self.d.overflowing_sub(imm),
            _ => (0, false),
        };

        match dest {
            0 => self.a = result,
            1 => self.b = result,
            2 => self.c = result,
            3 => self.d = result,
            _ => {}
        }

        self.zero = result == 0;
        self.carry = borrow;
    }


    pub fn jmp(&mut self, mem: &Memory) {
        let low = mem.read(self.pc) as u16;
        self.inc_pc();
        let high = mem.read(self.pc) as u16;
        self.inc_pc();

        let addrs = (high << 8) | low;

        self.pc = addrs;
    }

    pub fn jz(&mut self, mem: &Memory) {
        let low = mem.read(self.pc) as u16;
        self.inc_pc();
        let high = mem.read(self.pc) as u16;
        self.inc_pc();

        let addrs = (high << 8) | low;

        if self.zero {
            self.pc = addrs;
        }
    }

    pub fn jnz(&mut self, mem: &Memory) {
        let low = mem.read(self.pc) as u16;
        self.inc_pc();
        let high = mem.read(self.pc) as u16;
        self.inc_pc();

        let addrs = (high << 8) | low;

        if !self.zero {
            self.pc = addrs;
        }
    }

    pub fn cmp_rr(&mut self, mem: &Memory) {
        let r1 = mem.read(self.pc);
        self.inc_pc();
        let r2 = mem.read(self.pc);
        self.inc_pc();

        let v1 = self.get_reg(r1);
        let v2 = self.get_reg(r2);

        let (result, borrow) = v1.overflowing_sub(v2);

        self.zero = result == 0;
        self.carry = borrow;
    }

    pub fn cmp_ri(&mut self, mem: &Memory) {
        let r1 = mem.read(self.pc);
        self.inc_pc();
        let r2 = mem.read(self.pc);
        self.inc_pc();

        let v1 = self.get_reg(r1);

        let (result, borrow) = v1.overflowing_sub(r2);

        self.zero = result == 0;
        self.carry = borrow;
    }

    pub fn mul(&mut self, mem: &Memory) {
        let dest = mem.read(self.pc); self.inc_pc();
        let src  = mem.read(self.pc); self.inc_pc();

        let lhs = self.get_reg(dest);
        let rhs = self.get_reg(src);

        let result16 = (lhs as u16) * (rhs as u16);
        let result8  = (result16 & 0xFF) as u8;

        self.set_reg(dest, result8);

        self.zero  = result8 == 0;
        self.carry = result16 > 0xFF;
    }

    pub fn div(&mut self, mem: &Memory) {
        let dest = mem.read(self.pc); self.inc_pc();
        let src  = mem.read(self.pc); self.inc_pc();

        let lhs = self.get_reg(dest);
        let rhs = self.get_reg(src);

        if rhs == 0 {
            panic!("Division by zero");
            // OR: set halted = true
            // OR: define a syscall/exception later
        }

        let result = lhs / rhs;

        self.set_reg(dest, result);

        self.zero = result == 0;
        // carry unchanged
    }

    fn get_reg(&self, r: u8) -> u8 {
        match r {
            0 => self.a,
            1 => self.b,
            2 => self.c,
            3 => self.d,
            _ => panic!("Invalid Register"),
        }
    }

    fn set_reg(&mut self, dest: u8, val: u8) {
        match dest {
            0 => self.a = val,
            1 => self.b = val,
            2 => self.c = val,
            3 => self.d = val,
            _ => panic!("Invalid register"),
        }
    }

}
