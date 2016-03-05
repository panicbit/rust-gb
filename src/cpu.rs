use instructions::Instruction;
use memory::*;

pub struct Cpu {
    pub pc: u16,
    pub sp: u16,
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    // 7 6 5 4 3 2 1 0
    // Z N H C _ _ _ _
    pub f: u8,
    pub h: u8,
    pub l: u8,
    interrupts_enabled: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0x0100,
            sp: 0xFFFE,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            interrupts_enabled: true,
        }
    }

    pub fn a(&self) -> u8 { self.a }
    pub fn b(&self) -> u8 { self.b }
    pub fn c(&self) -> u8 { self.c }
    pub fn l(&self) -> u8 { self.l }
    pub fn h(&self) -> u8 { self.h }

    pub fn af(&self) -> u16 {
        self.a as u16 | (self.f as u16) << 8
    }

    pub fn bc(&self) -> u16 {
        self.b as u16 | (self.c as u16) << 8
    }

    pub fn de(&self) -> u16 {
        self.d as u16 | (self.e as u16) << 8
    }

    pub fn hl(&self) -> u16 {
        self.h as u16 | (self.l as u16) << 8
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    fn set_16(low: &mut u8, high: &mut u8, n: u16) {
        *high = (n >> 8) as u8;
        *low = n as u8;
    }

    pub fn set_a(&mut self, n: u8) {
        self.a = n;
    }

    pub fn set_b(&mut self, n: u8) {
        self.b = n;
    }

    pub fn set_af(&mut self, n: u16) {
        Self::set_16(&mut self.a, &mut self.f, n);
    }

    pub fn set_bc(&mut self, n: u16) {
        Self::set_16(&mut self.b, &mut self.c, n);
    }

    pub fn set_de(&mut self, n: u16) {
        Self::set_16(&mut self.d, &mut self.e, n);
    }

    pub fn set_hl(&mut self, n: u16) {
        Self::set_16(&mut self.h, &mut self.l, n);
    }

    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }

    pub fn set_sp(&mut self, sp: u16) {
        self.sp = sp;
    }

    pub fn push_u8(&mut self, mem: &mut Memory, value: u8) {
        self.sp -= 1;
        mem.write_u8(Addr(self.sp), value);
    }

    pub fn push_u16(&mut self, mem: &mut Memory, value: u16) {
        self.sp -= 2;
        mem.write_u16(Addr(self.sp), value);
    }

    pub fn pop_u8(&mut self, mem: &mut Memory) -> u8 {
        self.sp += 1;
        mem.read_u8(Addr(self.sp - 1))
    }

    pub fn pop_u16(&mut self, mem: &mut Memory) -> u16 {
        self.sp += 2;
        mem.read_u16(Addr(self.sp - 2))
    }

    pub fn step(&mut self, mem: &mut Memory) {
        let inst = Instruction::decode(mem, Addr(self.pc));
        self.pc += inst.len();
        let cycles = inst.cycles();
        inst.execute(self, mem);
    }

    pub fn incr_b(&mut self) {
        self.b += 1;
    }


    pub fn incr_bc(&mut self) {
        let value = self.bc();
        self.set_bc(value);
    }

    pub fn incr_hl(&mut self) {
        let value = self.hl();
        self.set_hl(value);
    }

    pub fn disable_interrupts(&mut self) {
        self.interrupts_enabled = false;
    }

    pub fn enable_interrupts(&mut self) {
        self.interrupts_enabled = true;
    }

    pub fn interrupts_enabled(&self) -> bool {
        self.interrupts_enabled
    }

    pub fn print_registers(&self) {
        println!(r"--------------");
        println!(r"| pc: {:02X}", self.pc);
        println!(r"| sp: {:02X}", self.sp);
        println!(r"| a: {:02X}", self.a);
        println!(r"| b: {:02X}", self.b);
        println!(r"| c: {:02X}", self.c);
        println!(r"| d: {:02X}", self.d);
        println!(r"| e: {:02X}", self.e);
        println!(r"| f: {:02X}", self.f);
        println!(r"| h: {:02X}", self.h);
        println!(r"| l: {:02X}", self.l);
        println!(r"| af: {:02X}", self.af());
        println!(r"| bc: {:02X}", self.bc());
        println!(r"| de: {:02X}", self.de());
        println!(r"| hl: {:02X}", self.hl());
        println!("");

    }

    pub fn set_flag_z(&mut self) {
        self.f |= 0x10000000;
    }

    pub fn reset_flag_z(&mut self) {
        self.f &= 0x01111111;
    }

    pub fn toggle_flag_z(&mut self) {
        self.f ^= 0x10000000;
    }
}