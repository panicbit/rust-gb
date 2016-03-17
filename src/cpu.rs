use std::num::Wrapping;
use instructions::Instruction;
use memory::*;

pub struct Cpu {
    pub pc: Wrapping<u16>,
    pub sp: Wrapping<u16>,
    pub a: Wrapping<u8>,
    pub b: Wrapping<u8>,
    pub c: Wrapping<u8>,
    pub d: Wrapping<u8>,
    pub e: Wrapping<u8>,
    // 7 6 5 4 3 2 1 0
    // Z N H C _ _ _ _
    pub f: u8,
    pub h: Wrapping<u8>,
    pub l: Wrapping<u8>,
    interrupts_enabled: bool,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: Wrapping(0x0100),
            sp: Wrapping(0xFFFE),
            a: Wrapping(0),
            b: Wrapping(0),
            c: Wrapping(0),
            d: Wrapping(0),
            e: Wrapping(0),
            f: 0b10110000,
            h: Wrapping(0),
            l: Wrapping(0),
            interrupts_enabled: true,
        }
    }

    pub fn a(&self) -> u8 { self.a.0 }
    pub fn b(&self) -> u8 { self.b.0 }
    pub fn c(&self) -> u8 { self.c.0 }
    pub fn d(&self) -> u8 { self.d.0 }
    pub fn e(&self) -> u8 { self.e.0 }
    pub fn f(&self) -> u8 { self.f   }
    pub fn l(&self) -> u8 { self.l.0 }
    pub fn h(&self) -> u8 { self.h.0 }

    pub fn af(&self) -> u16 {
        self.f as u16 | (self.a() as u16) << 8
    }

    pub fn bc(&self) -> u16 {
        self.c() as u16 | (self.b() as u16) << 8
    }

    pub fn de(&self) -> u16 {
        self.e() as u16 | (self.d() as u16) << 8
    }

    pub fn hl(&self) -> u16 {
        self.l() as u16 | (self.h() as u16) << 8
    }

    pub fn pc(&self) -> u16 {
        self.pc.0
    }

    pub fn sp(&self) -> u16 {
        self.sp.0
    }

    fn set_16(high: &mut u8, low: &mut u8, n: u16) {
        *high = (n >> 8) as u8;
        *low = n as u8;
    }

    pub fn set_a(&mut self, n: u8) {
        self.a = Wrapping(n);
    }

    pub fn set_b(&mut self, n: u8) {
        self.b = Wrapping(n);
    }

    pub fn set_c(&mut self, n: u8) {
        self.c = Wrapping(n);
    }

    pub fn set_e(&mut self, n: u8) {
        self.e = Wrapping(n);
    }

    pub fn set_f(&mut self, n: u8) {
        self.f = n;
    }

    pub fn set_h(&mut self, n: u8) {
        self.h = Wrapping(n);
    }

    pub fn set_af(&mut self, n: u16) {
        Self::set_16(&mut self.a.0, &mut self.f, n);
    }

    pub fn set_bc(&mut self, n: u16) {
        Self::set_16(&mut self.b.0, &mut self.c.0, n);
    }

    pub fn set_de(&mut self, n: u16) {
        Self::set_16(&mut self.d.0, &mut self.e.0, n);
    }

    pub fn set_hl(&mut self, n: u16) {
        Self::set_16(&mut self.h.0, &mut self.l.0, n);
    }

    pub fn set_pc(&mut self, pc: u16) {
        self.pc = Wrapping(pc);
    }

    pub fn set_sp(&mut self, sp: u16) {
        self.sp = Wrapping(sp);
    }

    pub fn push_u8(&mut self, mem: &mut Memory, value: u8) {
        self.sp -= Wrapping(1);
        mem.write_u8(Addr(self.sp()), value);
    }

    pub fn push_u16(&mut self, mem: &mut Memory, value: u16) {
        self.sp -= Wrapping(2);
        mem.write_u16(Addr(self.sp()), value);
    }

    pub fn pop_u8(&mut self, mem: &mut Memory) -> u8 {
        let result = mem.read_u8(Addr(self.sp()));
        self.sp += Wrapping(1);
        result
    }

    pub fn pop_u16(&mut self, mem: &mut Memory) -> u16 {
        let result = mem.read_u16(Addr(self.sp()));
        self.sp += Wrapping(2);
        result
    }

    pub fn step(&mut self, mem: &mut Memory) {
        let inst = Instruction::decode(mem, Addr(self.pc()));
        println!("{:04X} | {:?}", self.pc(), inst);
        self.pc += Wrapping(inst.len());
        let cycles = inst.cycles();
        inst.execute(self, mem);
    }

    pub fn add(&mut self, amount: u8) {
        let a = self.a();
        self.a += Wrapping(amount);

        unborrow!(self.set_flag_z(self.a() == 0));
        self.set_flag_n(false);
        self.set_flag_h(0xFF - (a << 4) < (amount << 4));
        self.set_flag_c(0xFF - a < amount);
    }

    pub fn add_carry(&mut self, amount: u8) {
        let carry = self.f() >> 4 & 0b1;
        self.add(amount);
        let f = self.f();
        self.add(carry);
        unborrow!(self.set_f(f | self.f()));
    }

    pub fn sub(&mut self, amount: u8) {
        let a = self.a();
        self.a -= Wrapping(amount);

        unborrow!(self.set_flag_z(self.a() == 0));
        self.set_flag_n(true);
        self.set_flag_h((a << 4) < (amount << 4));
        self.set_flag_c(a < amount);
    }

    pub fn compare(&mut self, value: u8) {
        let a = self.a();
        unborrow!(self.set_flag_z(a == value));
        self.set_flag_n(true);
        self.set_flag_h((a << 4) < (value << 4));
        self.set_flag_c(a < value);
    }

    pub fn incr_a(&mut self) {
        self.a += Wrapping(1);
        unborrow!(self.incr_affect_flags(self.a() as u16));
    }

    pub fn incr_b(&mut self) {
        self.b += Wrapping(1);
        unborrow!(self.incr_affect_flags(self.b() as u16));
    }

    pub fn incr_c(&mut self) {
        self.c += Wrapping(1);
        unborrow!(self.incr_affect_flags(self.c() as u16));
    }

    pub fn incr_d(&mut self) {
        self.d += Wrapping(1);
        unborrow!(self.incr_affect_flags(self.d() as u16));
    }

    pub fn incr_e(&mut self) {
        self.e += Wrapping(1);
        unborrow!(self.incr_affect_flags(self.e() as u16));
    }

    pub fn incr_h(&mut self) {
        self.h += Wrapping(1);
        unborrow!(self.incr_affect_flags(self.h() as u16));
    }

    pub fn incr_l(&mut self) {
        self.l += Wrapping(1);
        unborrow!(self.incr_affect_flags(self.l() as u16));
    }

    pub fn incr_bc(&mut self) {
        unborrow!(self.set_bc(self.bc().wrapping_add(1)));
        unborrow!(self.incr_affect_flags(self.bc()));
    }

    pub fn incr_de(&mut self) {
        unborrow!(self.set_de(self.de().wrapping_add(1)));
        unborrow!(self.incr_affect_flags(self.de()));
    }

    pub fn incr_hl(&mut self) {
        unborrow!(self.set_hl(self.hl().wrapping_add(1)));
        unborrow!(self.incr_affect_flags(self.hl()));
    }

    pub fn incr_hl_without_affecting_flags(&mut self) {
        unborrow!(self.set_hl(self.hl().wrapping_add(1)));
    }

    pub fn decr_hl_without_affecting_flags(&mut self) {
        unborrow!(self.set_hl(self.hl().wrapping_sub(1)));
    }

    pub fn incr_mhl(&mut self, mem: &mut Memory) {
        let addr = Addr(self.hl());
        let value = mem.read_u8(addr);
        let value = value.wrapping_add(1);
        unborrow!(self.incr_affect_flags(self.hl()));
        mem.write_u8(addr, value);
    }

    fn incr_affect_flags(&mut self, value: u16) {
        self.set_flag_z(value == 0);
        self.set_flag_n(false);
        self.set_flag_h(value & 0xF == 0);
    }

    pub fn decr_b(&mut self) {
        self.b -= Wrapping(1);
        unborrow!(self.decr_affect_flags(self.b() as u16));
    }

    pub fn decr_c(&mut self) {
        self.c -= Wrapping(1);
        unborrow!(self.decr_affect_flags(self.c() as u16));
    }

    pub fn decr_d(&mut self) {
        self.d -= Wrapping(1);
        unborrow!(self.decr_affect_flags(self.d() as u16));
    }

    pub fn decr_l(&mut self) {
        self.l -= Wrapping(1);
        unborrow!(self.decr_affect_flags(self.l() as u16));
    }

    pub fn decr_hl(&mut self) {
        unborrow!(self.set_hl(self.hl().wrapping_sub(1)));
        unborrow!(self.decr_affect_flags(self.hl()));
    }

    fn decr_affect_flags(&mut self, value: u16) {
        self.set_flag_z(value == 0);
        self.set_flag_n(true);
        self.set_flag_h(value & 0xF == 0xF);
    }

    pub fn or(&mut self, value: u8) {
        self.a |= Wrapping(value);
        unborrow!(self.set_flag_z(self.a() == 0));
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_c(false);
    }

    pub fn and(&mut self, value: u8) {
        self.a &= Wrapping(value);
        unborrow!(self.set_flag_z(self.a() == 0));
        self.set_flag_n(false);
        self.set_flag_h(true);
        self.set_flag_c(false);
    }

    pub fn xor(&mut self, value: u8) {
        self.a ^= Wrapping(value);
        unborrow!(self.set_flag_z(self.a() == 0));
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_c(false);
    }

    pub fn call(&mut self, mem: &mut Memory, addr: u16) {
        unborrow!(self.push_u16(mem, self.pc()));
        self.set_pc(addr);
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

    pub fn jump_routine(&mut self, offset: i8) {
        if offset >= 0 {
            unborrow!(self.set_pc(self.pc().wrapping_add( offset as u16)))
        } else {
            unborrow!(self.set_pc(self.pc().wrapping_sub(offset.abs() as u16)))
        }
    }

    pub fn print_registers(&self) {
        println!(r"--------------");
        println!(r"| af: {:02X}{:02X}", self.a(), self.f());
        println!(r"| bc: {:02X}{:02X}", self.b(), self.c());
        println!(r"| de: {:02X}{:02X}", self.d(), self.e());
        println!(r"| hl: {:02X}{:02X}", self.h(), self.l());
        println!(r"| sp: {:04X}", self.sp());
        println!(r"| pc: {:04X}", self.pc());
        println!(r"| z?: {}", self.flag_z());
        println!("");

    }

    pub fn flag_z(&self)  -> bool {
        //          76543210
        self.f >= 0b10000000
    }

    pub fn set_flag_z(&mut self, set: bool) {
        //              76543210
        if set {
            self.f |= 0b10000000;
        } else {
            self.f &= 0b01111111;
        }
    }

    pub fn flag_n(&self)  -> bool {
        //          76543210
        self.f >= 0b01000000
    }

    pub fn set_flag_n(&mut self, set: bool) {
        //              76543210
        if set {
            self.f |= 0b01000000;
        } else {
            self.f &= 0b10111111;
        }
    }

    pub fn flag_h(&self)  -> bool {
        //          76543210
        self.f >= 0b00100000
    }

    pub fn set_flag_h(&mut self, set: bool) {
        //              76543210
        if set {
            self.f |= 0b00100000;
        } else {
            self.f &= 0b11011111;
        }
    }

    pub fn flag_c(&self)  -> bool {
        //          76543210
        self.f >= 0b00010000
    }

    pub fn set_flag_c(&mut self, set: bool) {
        //              76543210
        if set {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
    }
}
