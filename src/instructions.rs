use byteorder::{ByteOrder, LittleEndian};
use cpu::Cpu;
use bit_range::BitRange;
use memory::*;

type LE = LittleEndian;

// https://danielkeep.github.io/tlborm/book/pat-repetition-replacement.html
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {$sub};
}

macro_rules! instructions {
    (
        $struct_name: ident
        |$cpu:ident, $mem:ident, $addr:ident|
        $(
            $op:expr,
            $len:expr,
            $cycles:expr,
            $name:ident$(( $( $p_name:ident : $p_ty:ty ),+ ))* =>
            $exec:expr
        );+
        $(;)*
    ) => (
        #[derive(Copy, Clone, Debug, Eq, PartialEq)]
        pub enum $struct_name {
            $(
                $name$(( $( $p_ty ),+ ))*
            ),*
        }

        impl $struct_name {
            pub fn decode($mem: &mut Memory, $addr: Addr) -> $struct_name {
                use self::$struct_name::*;

                let op = $mem.read_u8($addr);
                let mut $addr = $addr + 1;

                match op {
                    $(
                        $op => $name$(( $( replace_expr!($p_name Param::get($mem, &mut $addr)) ),+ ))*
                    ),*,
                    op => panic!("opcode 0x{:02X}", op)
                }
            }

            #[allow(unused_variables)]
            pub fn len(&self) -> u16 {
                use self::$struct_name::*;
                match *self {
                    $(
                        $name$(( $( $p_name ),+ ))* => $len
                    ),*,
                }
            }

            #[allow(unused_variables)]
            pub fn cycles(&self) -> u16 {
                use self::$struct_name::*;
                match *self {
                    $(
                        $name$(( $( $p_name ),+ ))* => $cycles
                    ),*,
                }
            }

            pub fn execute(&self, $cpu: &mut Cpu, $mem: &mut Memory) {
                use self::$struct_name::*;
                //println!("OP: {:?}", self);
                match *self {
                    $(
                        $name$(( $( $p_name ),+ ))* => {$exec;}
                    ),*
                }
            }
        }

    )
}

trait Param {
    fn get(mem: &mut Memory, addr: &mut Addr) -> Self;
}

impl Param for u8 {
    fn get(mem: &mut Memory, addr: &mut Addr) -> Self {
        *addr = *addr + 1;
        mem.read_u8(*addr - 1)
    }
}

impl Param for i8 {
    fn get(mem: &mut Memory, addr: &mut Addr) -> Self {
        *addr = *addr + 1;
        mem.read_u8(*addr - 1) as i8
    }
}

impl Param for u16 {
    fn get(mem: &mut Memory, addr: &mut Addr) -> Self {
        *addr = *addr + 2;
        mem.read_u16(*addr - 2)
    }
}

impl Param for ExtendedInstruction {
    fn get(mem: &mut Memory, addr: &mut Addr) -> Self {
        let instr = ExtendedInstruction::decode(mem, *addr);
        addr.0 += instr.len();
        instr
    }
}

instructions! {
    Instruction
    |cpu, mem, addr|
    // op, len, cycles
    0x00, 1,  4, NOP => {};
    0xFB, 1,  4, EI => cpu.enable_interrupts();
    0xF3, 1,  4, DI => cpu.disable_interrupts();
    0xC3, 3, 12, JP_nn(pc: u16) => cpu.set_pc(pc);
    0x78, 1,  4, LD_A_B => unborrow!(cpu.set_a(cpu.b()));
    0x7C, 1,  4, LD_A_H => unborrow!(cpu.set_a(cpu.h()));
    0x7D, 1,  4, LD_A_L => unborrow!(cpu.set_a(cpu.l()));
    0x1A, 1,  8, LD_A_DE => unborrow!(cpu.set_a(cpu.de() as u8));
    0x3E, 2,  8, LD_A_n(value: u8) => cpu.set_a(value);
    0xFA, 3, 16, LD_A_Mnn(p: u16) => cpu.set_a(mem.read_u8(Addr(p)));
    0x47, 1,  4, LD_B_A => unborrow!(cpu.set_b(cpu.a()));
    0x46, 1,  8, LD_B_MHL => unborrow!(cpu.set_b(mem.read_u8(Addr(cpu.hl()))));
    0x06, 2,  8, LD_B_n(value: u8) => cpu.set_b(value);
    0x4E, 1,  8, LD_C_MHL => unborrow!(cpu.set_c(mem.read_u8(Addr(cpu.hl()))));
    0x0E, 2,  8, LD_C_n(value: u8) => cpu.set_c(value);
    0x56, 1,  8, LD_D_MHL => unborrow!(cpu.set_d(mem.read_u8(Addr(cpu.hl()))));
    0x5A, 1,  4, LD_E_D => unborrow!(cpu.set_e(cpu.d()));
    0x26, 2,  8, LD_H_n(value: u8) => cpu.set_h(value);
    0x01, 3, 12, LD_BC_nn(value: u16) => cpu.set_bc(value);
    0x11, 3, 12, LD_DE_nn(value: u16) => cpu.set_de(value);
    0x21, 3, 12, LD_HL_nn(value: u16) => cpu.set_hl(value);
    0x31, 3, 12, LD_SP_nn(value: u16) => cpu.set_sp(value);
    0x12, 1,  8, LD_MDE_A => mem.write_u16(Addr(cpu.de()), cpu.a() as u16);
    0x77, 1,  8, LD_MHL_A => mem.write_u16(Addr(cpu.hl()), cpu.a() as u16);
    0xEA, 3, 16, LD_Mnn_A(p: u16) => mem.write_u16(Addr(p), cpu.a() as u16);
    0xE0, 2, 12, LD_Mn_A(p: u8) => mem.write_u8(Addr(0xFF00) + p as u16, cpu.a());
    0x2A, 1,  8, LDI_A_MHL => {
        let value = mem.read_u8(Addr(cpu.hl()));
        cpu.set_a(value);
        cpu.incr_hl_without_affecting_flags();
    };
    0x22, 1,  8, LDI_MHL_A => {
        mem.write_u8(Addr(cpu.hl()), cpu.a());
        cpu.incr_hl_without_affecting_flags();
    };
    0x32, 1,  8, LDD_MHL_A => {
        mem.write_u8(Addr(cpu.hl()), cpu.a());
        cpu.decr_hl_without_affecting_flags();
    };
    0xF0, 2, 12, LD_A_Mn(offset: u8) => unborrow!(cpu.set_a(mem.read_u8(Addr(0xFF00 + offset as u16))));
    0xC4, 3, 12, CALL_NZ_nn(addr: u16) => if !cpu.flag_z() { cpu.call(mem, addr) };
    0xCD, 3, 12, CALL_nn(addr: u16) => cpu.call(mem, addr);
    0xE9, 1,  4, JP_HL => unborrow!(cpu.set_pc(cpu.hl()));
    0xC2, 3, 12, JP_NZ_nn(addr: u16) => if !cpu.flag_z() { cpu.set_pc(addr) };
    0x18, 2,  8, JR_n(offset: i8) => cpu.jump_routine(offset);
    0x20, 2,  8, JR_NZ_n(offset: i8) => if !cpu.flag_z() { cpu.jump_routine(offset) };
    0x28, 2,  8, JR_Z(offset: i8) => if cpu.flag_z() { cpu.jump_routine(offset) };
    0xC9, 1,  8, RET => unborrow!(cpu.set_pc(cpu.pop_u16(mem)));
    0xC0, 1,  8, RET_NZ => if cpu.flag_z() { RET.execute(cpu, mem) };
    0xF5, 1, 16, PUSH_AF => unborrow!(cpu.push_u16(mem, cpu.af()));
    0xC5, 1, 16, PUSH_BC => unborrow!(cpu.push_u16(mem, cpu.bc()));
    0xD5, 1, 16, PUSH_DE => unborrow!(cpu.push_u16(mem, cpu.de()));
    0xE5, 1, 16, PUSH_HL => unborrow!(cpu.push_u16(mem, cpu.hl()));
    0xC1, 1, 12, POP_BC => unborrow!(cpu.set_bc(cpu.pop_u16(mem)));
    0xF1, 1, 12, POP_AF => unborrow!(cpu.set_af(cpu.pop_u16(mem)));
    0xE1, 1, 12, POP_HL => unborrow!(cpu.set_hl(cpu.pop_u16(mem)));
    0xC6, 2,  8, ADD_n(amount: u8) => cpu.add(amount);
    0xD6, 2,  8, SUB_n(amount: u8) => cpu.sub(amount);
    0x8A, 1,  4, ADC_D => unborrow!(cpu.add_carry(cpu.d()));
    0xCE, 1,  8, ADC_n(amount: u8) => cpu.add_carry(amount);
    0x3C, 1,  4, INC_A => cpu.incr_a();
    0x04, 1,  4, INC_B => cpu.incr_b();
    0x0C, 1,  4, INC_C => cpu.incr_c();
    0x14, 1,  4, INC_D => cpu.incr_d();
    0x1C, 1,  4, INC_E => cpu.incr_e();
    0x24, 1,  4, INC_H => cpu.incr_h();
    0x2C, 1,  4, INC_L => cpu.incr_l();
    0x03, 1,  8, INC_BC => cpu.incr_bc();
    0x13, 1,  8, INC_DE => cpu.incr_de();
    0x23, 1,  8, INC_HL => cpu.incr_hl();
    0x34, 1, 12, INC_MHL => cpu.incr_mhl(mem);
    0x05, 1,  4, DEC_B => cpu.decr_b();
    0x0D, 1,  4, DEC_C => cpu.decr_c();
    0x2D, 1,  4, DEC_L => cpu.decr_l();
    0xB7, 1,  4, OR_A => unborrow!(cpu.or(cpu.a()));
    0xB0, 1,  4, OR_B => unborrow!(cpu.or(cpu.b()));
    0xB1, 1,  4, OR_C => unborrow!(cpu.or(cpu.c()));
    0xE6, 2,  8, AND_n(value: u8) => cpu.and(value);
    0xAE, 1,  8, XOR_MHL => unborrow!(cpu.xor(mem.read_u8(Addr(cpu.hl()))));
    0xA9, 1,  4, XOR_C => unborrow!(cpu.xor(cpu.c()));
    0xB9, 1,  4, CP_C => unborrow!(cpu.compare(cpu.c()));
    0xFE, 2,  8, CP_n(value: u8) => unborrow!(cpu.compare(value));
    0x2F, 1,  4, CPL => unborrow!(cpu.set_a(cpu.a() ^ 0xFF));
    0x1F, 1,  4, RRA => cpu.rotate_right_a();
    0xCB, instr.len() + 1, instr.cycles(), Extended(instr: ExtendedInstruction) => instr.execute(cpu, mem);
}

instructions! {
    ExtendedInstruction
    |cpu, mem, addr|
    // op, len, cycles
    0x38, 1,  8, SRL_B => cpu.shift_right_logical_b();
    0x19, 1,  8, RR_C => cpu.rotate_right_c();
    0x1A, 1,  8, RR_D => cpu.rotate_right_d();
}
