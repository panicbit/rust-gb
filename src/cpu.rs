use std::num::Wrapping;
use instructions::Instruction;
use memory::*;
use util::IntoWrapping;

macro_rules! reg8 {
    ($R:ident => $r:ident) => (
        pub enum $R {}
        impl Reg8 for $R {
            fn get(cpu: &Cpu) -> Wrapping<u8> {
                cpu.$r
            }
            fn set<V: IntoWrapping<u8>>(cpu: &mut Cpu, value: V) {
                cpu.$r = value.into_wrapping();
            }
        }
    )
}

pub mod registers {
    use std::num::Wrapping;
    use util::IntoWrapping;
    use super::Cpu;
    use super::Reg8;
    reg8!(A => a);
    reg8!(B => b);
    reg8!(C => c);
    reg8!(D => d);
    reg8!(E => e);
    reg8!(H => h);
    reg8!(L => l);
}
use self::registers::*;

trait Reg8 {
    fn get(cpu: &Cpu) -> Wrapping<u8>;
    fn set<V: IntoWrapping<u8>>(cpu: &mut Cpu, value: V);
}

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
    pub is_stalling: bool,
    pub at_breakpoint: bool,
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
            is_stalling: false,
            at_breakpoint: false,
        }
    }

    pub fn get<R: Reg8>(&self) -> Wrapping<u8> { R::get(self) }
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

    pub fn set<R: Reg8>(&mut self, n: Wrapping<u8>) {
        R::set(self, n);
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

    pub fn set_d(&mut self, n: u8) {
        self.d = Wrapping(n);
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

    pub fn set_l(&mut self, n: u8) {
        self.l = Wrapping(n);
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

    pub fn load<R1: Reg8, R2: Reg8>(&mut self) {
        let r2 = self.get::<R2>();
        self.set::<R1>(r2);
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
        let last_pc = self.pc();
        let inst = Instruction::decode(mem, Addr(self.pc()));

        // if self.pc.0 == 0xC2C2 {
        //     self.at_breakpoint = true;
        //     self.print_registers();
        // }

        println!("{:04X} | {:?}", self.pc(), inst);
        if self.at_breakpoint {
            let mut cmd = String::new();
            ::std::io::stdin().read_line(&mut cmd);
            let cmd = cmd.trim();
            if cmd == "c" {
                self.at_breakpoint = false;
            }
        }

        self.pc += Wrapping(inst.len());
        let cycles = inst.cycles();
        inst.execute(self, mem);

        if self.pc() == last_pc {
            self.is_stalling = true;
        }

        // if self.at_breakpoint {
            self.print_registers();
        // }
    }

    pub fn add(&mut self, amount: u8) {
        let a = self.a();
        self.a += Wrapping(amount);

        unborrow!(self.set_flag_z(self.a() == 0));
        self.set_flag_n(false);
        self.set_flag_h(0xFF - (a << 4) < (amount << 4));
        self.set_flag_c(0xFF - a < amount);
    }

    pub fn add_hl(&mut self, amount: u16) {
        let hl = self.hl();
        self.set_hl(hl.wrapping_add(amount));

        // Z is not affected
        self.set_flag_n(false);
        self.set_flag_h(0xFFFF - (hl << 4) < (amount << 4));
        self.set_flag_c(0xFFFF - hl < amount);
    }

    pub fn add_carry(&mut self, amount: u8) {
        let carry = self.f() >> 4 & 0b1;
        self.add(amount);
        let f = self.f();
        self.add(carry);
        unborrow!(self.set_f(f | self.f()));
        // check if final result is zero
        unborrow!(self.set_flag_z(self.a() == 0));
    }

    pub fn sub(&mut self, amount: u8) {
        let a = self.a();
        self.a -= Wrapping(amount);

        unborrow!(self.set_flag_z(self.a() == 0));
        self.set_flag_n(true);
        self.set_flag_h((a << 4) < (amount << 4));
        self.set_flag_c(a < amount);
    }

    pub fn sub_carry(&mut self, amount: u8) {
        let carry = self.f() >> 4 & 0b1;
        self.sub(amount);
        let f = self.f();
        self.sub(carry);
        unborrow!(self.set_f(f | self.f()));
        // check if final result is zero
        unborrow!(self.set_flag_z(self.a() == 0));
    }

    pub fn compare(&mut self, value: u8) {
        let a = self.a();
        unborrow!(self.set_flag_z(a == value));
        self.set_flag_n(true);
        self.set_flag_h((a << 4) < (value << 4));
        self.set_flag_c(a < value);
    }

    pub fn increment<R: Reg8>(&mut self) {
        let mut r = self.get::<R>();
        r += Wrapping(1);
        self.set::<R>(r);

        // Update flags
        let r = r.0;
        self.set_flag_z(r == 0);
        self.set_flag_n(false);
        self.set_flag_h(r & 0x0F == 0);
    }

    pub fn increment_bc(&mut self) {
        unborrow!(self.set_bc(self.bc().wrapping_add(1)));
        unborrow!(self.increment_affect_flags(self.bc()));
    }

    pub fn increment_de(&mut self) {
        unborrow!(self.set_de(self.de().wrapping_add(1)));
        unborrow!(self.increment_affect_flags(self.de()));
    }

    pub fn increment_hl(&mut self) {
        unborrow!(self.set_hl(self.hl().wrapping_add(1)));
        unborrow!(self.increment_affect_flags(self.hl()));
    }

    pub fn increment_hl_without_affecting_flags(&mut self) {
        unborrow!(self.set_hl(self.hl().wrapping_add(1)));
    }

    pub fn decrement_hl_without_affecting_flags(&mut self) {
        unborrow!(self.set_hl(self.hl().wrapping_sub(1)));
    }

    pub fn increment_mhl(&mut self, mem: &mut Memory) {
        let addr = Addr(self.hl());
        let value = mem.read_u8(addr);
        let value = value.wrapping_add(1);
        unborrow!(self.increment_affect_flags(value as u16));
        mem.write_u8(addr, value);
    }

    fn increment_affect_flags(&mut self, value: u16) {
        self.set_flag_z(value == 0);
        self.set_flag_n(false);
        self.set_flag_h(value & 0xF == 0);
    }

    pub fn decrement<R: Reg8>(&mut self) {
        let mut r = self.get::<R>();
        r -= Wrapping(1);
        self.set::<R>(r);

        // Update flags
        let r = r.0;
        self.set_flag_z(r == 0);
        self.set_flag_n(true);
        self.set_flag_h(r & 0xF == 0xF);
    }

    pub fn decrement_hl(&mut self) {
        unborrow!(self.set_hl(self.hl().wrapping_sub(1)));
        unborrow!(self.decrement_affect_flags(self.hl()));
    }

    pub fn decrement_mhl(&mut self, mem: &mut Memory) {
        let addr = Addr(self.hl());
        let value = mem.read_u8(addr);
        let value = value.wrapping_sub(1);
        unborrow!(self.decrement_affect_flags(value as u16));
        mem.write_u8(addr, value);
    }

    fn decrement_affect_flags(&mut self, value: u16) {
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

    pub fn shift_right_logical_b(&mut self) {
        let msb = self.b.0 & 1;
        self.b >>= 1;

        unborrow!(self.set_flag_z(self.l() == 0));
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_c(msb == 1);
    }

    pub fn rotate_right_a(&mut self) {
        let new_carry = self.a.0 & 1;
        self.a >>= 1;
        self.a.0 |= (self.flag_c() as u8) << 7;

        unborrow!(self.rotate_affect_flags(
            self.a.0 == 0, // Z
            new_carry == 1 // C
        ));
    }

    pub fn rotate_right_c(&mut self) {
        let new_carry = self.c.0 & 1;
        self.c >>= 1;
        self.c.0 |= (self.flag_c() as u8) << 7;

        unborrow!(self.rotate_affect_flags(
            self.c.0 == 0, // Z
            new_carry == 1 // C
        ));
    }

    pub fn rotate_right_d(&mut self) {
        let new_carry = self.d.0 & 1;
        self.d >>= 1;
        self.d.0 |= (self.flag_c() as u8) << 7;

        unborrow!(self.rotate_affect_flags(
            self.d.0 == 0, // Z
            new_carry == 1 // C
        ));
    }

    pub fn rotate_right_e(&mut self) {
        let new_carry = self.e.0 & 1;
        self.e >>= 1;
        self.e.0 |= (self.flag_c() as u8) << 7;

        unborrow!(self.rotate_affect_flags(
            self.e.0 == 0, // Z
            new_carry == 1 // C
        ));
    }

    pub fn rotate_left_a(&mut self) {
        let new_carry = (self.a.0 >> 7) & 1;
        self.a <<= 1;
        self.a.0 |= (self.flag_c() as u8) >> 7;

        unborrow!(self.rotate_affect_flags(
            self.a.0 == 0, // Z
            new_carry == 1 // C
        ));
    }

    fn rotate_affect_flags(&mut self, z: bool, c: bool) {
        self.set_flag_z(z);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_c(c);
    }

    pub fn rotate_left_carry<R: Reg8>(&mut self) {
        let r = self.get::<R>();
        let new_carry = (r.0 >> 7) & 1;
        R::set(self, r.0.rotate_left(1));

        // update flags
        let r = r.0;
        unborrow!(self.rotate_affect_flags(
            r == 0,        // Z
            new_carry == 1 // C
        ));
    }

    pub fn rotate_right_carry<R: Reg8>(&mut self) {
        let r = self.get::<R>();
        let new_carry = r.0 & 1;
        R::set(self, r.0.rotate_right(1));

        // update flags
        let r = r.0;
        unborrow!(self.rotate_affect_flags(
            r == 0,        // Z
            new_carry == 1 // C
        ));
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
        self.f >> 7 == 1
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
        (self.f >> 6) & 1 == 1
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
        (self.f >> 5) & 1 == 1
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
        (self.f >> 4) & 1 == 1
    }

    pub fn set_flag_c(&mut self, set: bool) {
        //              76543210
        if set {
            self.f |= 0b00010000;
        } else {
            self.f &= 0b11101111;
        }
    }

    pub fn swap_nibbles_a(&mut self) {
        unborrow!(self.set_a(swap_nibbles(self.a())));
        unborrow!(self.swap_nibbles_affect_flags(self.a()));
    }

    pub fn swap_nibbles_b(&mut self) {
        unborrow!(self.set_b(swap_nibbles(self.b())));
        unborrow!(self.swap_nibbles_affect_flags(self.b()));
    }

    pub fn swap_nibbles_c(&mut self) {
        unborrow!(self.set_c(swap_nibbles(self.c())));
        unborrow!(self.swap_nibbles_affect_flags(self.c()));
    }

    pub fn swap_nibbles_d(&mut self) {
        unborrow!(self.set_d(swap_nibbles(self.d())));
        unborrow!(self.swap_nibbles_affect_flags(self.d()));
    }

    pub fn swap_nibbles_e(&mut self) {
        unborrow!(self.set_e(swap_nibbles(self.e())));
        unborrow!(self.swap_nibbles_affect_flags(self.e()));
    }

    pub fn swap_nibbles_h(&mut self) {
        unborrow!(self.set_h(swap_nibbles(self.h())));
        unborrow!(self.swap_nibbles_affect_flags(self.h()));
    }

    pub fn swap_nibbles_l(&mut self) {
        unborrow!(self.set_l(swap_nibbles(self.l())));
        unborrow!(self.swap_nibbles_affect_flags(self.l()));
    }

    pub fn swap_nibbles_affect_flags(&mut self, value: u8) {
        self.set_flag_z(value == 0);
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_c(false);
    }

    pub fn set_carry_flag(&mut self) {
        self.set_flag_n(false);
        self.set_flag_h(false);
        self.set_flag_c(true);
    }

    pub fn complement_carry_flag(&mut self) {
        self.set_flag_n(false);
        self.set_flag_h(false);
        unborrow!(self.set_flag_c(self.flag_c()));
    }
}

fn swap_nibbles(value: u8) -> u8 {
    (value << 4) | (value >> 4)
}
