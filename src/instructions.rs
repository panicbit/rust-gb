use byteorder::{ByteOrder, LittleEndian};
use cpu::Cpu;
use cpu::registers::*;
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
        #[allow(non_camel_case_types)]
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
                    op => panic!(concat!(stringify!($struct_name), " 0x{:02X}"), op)
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
    0x7F, 1,  4, LD_A_A => cpu.load::<A,A>();
    0x78, 1,  4, LD_A_B => cpu.load::<A,B>();
    0x79, 1,  4, LD_A_C => cpu.load::<A,C>();
    0x7A, 1,  4, LD_A_D => cpu.load::<A,D>();
    0x7B, 1,  4, LD_A_E => cpu.load::<A,E>();
    0x7C, 1,  4, LD_A_H => cpu.load::<A,H>();
    0x7D, 1,  4, LD_A_L => cpu.load::<A,L>();
    0x1A, 1,  8, LD_A_MDE => unborrow!(cpu.set_a(mem.read_u8(Addr(cpu.de()))));
    0x7E, 1,  8, LD_A_MHL => unborrow!(cpu.set_a(mem.read_u8(Addr(cpu.hl()))));
    0x3E, 2,  8, LD_A_n(value: u8) => cpu.set_a(value);
    0xFA, 3, 16, LD_A_Mnn(p: u16) => cpu.set_a(mem.read_u8(Addr(p)));
    0x47, 1,  4, LD_B_A => cpu.load::<B,A>();
    0x40, 1,  4, LD_B_B => cpu.load::<B,B>();
    0x41, 1,  4, LD_B_C => cpu.load::<B,C>();
    0x42, 1,  4, LD_B_D => cpu.load::<B,D>();
    0x43, 1,  4, LD_B_E => cpu.load::<B,E>();
    0x44, 1,  4, LD_B_H => cpu.load::<B,H>();
    0x45, 1,  4, LD_B_L => cpu.load::<B,L>();
    0x46, 1,  8, LD_B_MHL => unborrow!(cpu.set_b(mem.read_u8(Addr(cpu.hl()))));
    0x06, 2,  8, LD_B_n(value: u8) => cpu.set_b(value);
    0x4F, 1,  4, LD_C_A => cpu.load::<C,A>();
    0x48, 1,  4, LD_C_B => cpu.load::<C,B>();
    0x49, 1,  4, LD_C_C => cpu.load::<C,C>();
    0x4A, 1,  4, LD_C_D => cpu.load::<C,D>();
    0x4B, 1,  4, LD_C_E => cpu.load::<C,E>();
    0x4C, 1,  4, LD_C_H => cpu.load::<C,H>();
    0x4D, 1,  4, LD_C_L => cpu.load::<C,L>();
    0x4E, 1,  8, LD_C_MHL => unborrow!(cpu.set_c(mem.read_u8(Addr(cpu.hl()))));
    0x57, 1,  4, LD_D_A => cpu.load::<D,A>();
    0x50, 1,  4, LD_D_B => cpu.load::<D,B>();
    0x51, 1,  4, LD_D_C => cpu.load::<D,C>();
    0x52, 1,  4, LD_D_D => cpu.load::<D,D>();
    0x53, 1,  4, LD_D_E => cpu.load::<D,E>();
    0x54, 1,  4, LD_D_H => cpu.load::<D,H>();
    0x55, 1,  4, LD_D_L => cpu.load::<D,L>();
    0x0E, 2,  8, LD_C_n(value: u8) => cpu.set_c(value);
    0x56, 1,  8, LD_D_MHL => unborrow!(cpu.set_d(mem.read_u8(Addr(cpu.hl()))));
    0x5F, 1,  4, LD_E_A => cpu.load::<E,A>();
    0x58, 1,  4, LD_E_B => cpu.load::<E,B>();
    0x59, 1,  4, LD_E_C => cpu.load::<E,C>();
    0x5A, 1,  4, LD_E_D => cpu.load::<E,D>();
    0x5B, 1,  4, LD_E_E => cpu.load::<E,E>();
    0x5C, 1,  4, LD_E_H => cpu.load::<E,H>();
    0x5D, 1,  4, LD_E_L => cpu.load::<E,L>();
    0x5E, 1,  8, LD_E_MHL => unborrow!(cpu.set_e(mem.read_u8(Addr(cpu.hl()))));
    0x67, 1,  4, LD_H_A => cpu.load::<H,A>();
    0x60, 1,  4, LD_H_B => cpu.load::<H,B>();
    0x61, 1,  4, LD_H_C => cpu.load::<H,C>();
    0x62, 1,  4, LD_H_D => cpu.load::<H,D>();
    0x63, 1,  4, LD_H_E => cpu.load::<H,E>();
    0x64, 1,  4, LD_H_H => cpu.load::<H,H>();
    0x65, 1,  4, LD_H_L => cpu.load::<H,L>();
    0x66, 1,  8, LD_H_MHL => unborrow!(cpu.set_h(mem.read_u8(Addr(cpu.hl()))));
    0x26, 2,  8, LD_H_n(value: u8) => cpu.set_h(value);
    0x6F, 1,  4, LD_L_A => cpu.load::<L,A>();
    0x68, 1,  4, LD_L_B => cpu.load::<L,B>();
    0x69, 1,  4, LD_L_C => cpu.load::<L,C>();
    0x6A, 1,  4, LD_L_D => cpu.load::<L,D>();
    0x6B, 1,  4, LD_L_E => cpu.load::<L,E>();
    0x6C, 1,  4, LD_L_H => cpu.load::<L,H>();
    0x6D, 1,  4, LD_L_L => cpu.load::<L,L>();
    0x6E, 1,  8, LD_L_MHL => unborrow!(cpu.set_l(mem.read_u8(Addr(cpu.hl()))));
    0x2E, 2,  8, LD_L_n(value: u8) => cpu.set_l(value);
    0x01, 3, 12, LD_BC_nn(value: u16) => cpu.set_bc(value);
    0x11, 3, 12, LD_DE_nn(value: u16) => cpu.set_de(value);
    0x21, 3, 12, LD_HL_nn(value: u16) => cpu.set_hl(value);
    0x31, 3, 12, LD_SP_nn(value: u16) => cpu.set_sp(value);
    0x12, 1,  8, LD_MDE_A => mem.write_u8(Addr(cpu.de()), cpu.a());
    0x77, 1,  8, LD_MHL_A => mem.write_u8(Addr(cpu.hl()), cpu.a());
    0x70, 1,  8, LD_MHL_B => mem.write_u8(Addr(cpu.hl()), cpu.b());
    0x71, 1,  8, LD_MHL_C => mem.write_u8(Addr(cpu.hl()), cpu.c());
    0x72, 1,  8, LD_MHL_D => mem.write_u8(Addr(cpu.hl()), cpu.d());
    0x73, 1,  8, LD_MHL_E => mem.write_u8(Addr(cpu.hl()), cpu.e());
    0x74, 1,  8, LD_MHL_H => mem.write_u8(Addr(cpu.hl()), cpu.h());
    0x75, 1,  8, LD_MHL_L => mem.write_u8(Addr(cpu.hl()), cpu.l());
    0xEA, 3, 16, LD_Mnn_A(p: u16) => mem.write_u8(Addr(p), cpu.a());
    0xE0, 2, 12, LD_Mn_A(p: u8) => mem.write_u8(Addr(0xFF00 + p as u16), cpu.a());
    0x2A, 1,  8, LDI_A_MHL => {
        let value = mem.read_u8(Addr(cpu.hl()));
        cpu.set_a(value);
        cpu.increment_hl_without_affecting_flags();
    };
    0x22, 1,  8, LDI_MHL_A => {
        mem.write_u8(Addr(cpu.hl()), cpu.a());
        cpu.increment_hl_without_affecting_flags();
    };
    0x32, 1,  8, LDD_MHL_A => {
        mem.write_u8(Addr(cpu.hl()), cpu.a());
        cpu.decrement_hl_without_affecting_flags();
    };
    0xF0, 2, 12, LD_A_Mn(offset: u8) => unborrow!(cpu.set_a(mem.read_u8(Addr(0xFF00 + offset as u16))));
    0xC4, 3, 12, CALL_NZ_nn(addr: u16) => if !cpu.flag_z() { cpu.call(mem, addr) };
    0xCD, 3, 12, CALL_nn(addr: u16) => cpu.call(mem, addr);
    0xE9, 1,  4, JP_HL => unborrow!(cpu.set_pc(cpu.hl()));
    0xC2, 3, 12, JP_NZ_nn(addr: u16) => if !cpu.flag_z() { cpu.set_pc(addr) };
    0x18, 2,  8, JR_n(offset: i8) => cpu.jump_routine(offset);
    0x20, 2,  8, JR_NZ_n(offset: i8) => if !cpu.flag_z() { cpu.jump_routine(offset) };
    0x28, 2,  8, JR_Z_n(offset: i8) => if cpu.flag_z() { cpu.jump_routine(offset) };
    0x38, 2,  8, JR_C_n(offset: i8) => if cpu.flag_c() { cpu.jump_routine(offset) };
    0x30, 2,  8, JR_NC(offset: i8) => if !cpu.flag_c() { cpu.jump_routine(offset) };
    0xC9, 1,  8, RET => unborrow!(cpu.set_pc(cpu.pop_u16(mem)));
    0xC8, 1,  8, RET_Z => if cpu.flag_z() { RET.execute(cpu, mem) };
    0xC0, 1,  8, RET_NZ => if !cpu.flag_z() { RET.execute(cpu, mem) };
    0xD8, 1,  8, RET_C => if cpu.flag_c() { RET.execute(cpu, mem) };
    0xD0, 1,  8, RET_NC => if !cpu.flag_c() { RET.execute(cpu, mem) };
    0xF5, 1, 16, PUSH_AF => unborrow!(cpu.push_u16(mem, cpu.af()));
    0xC5, 1, 16, PUSH_BC => unborrow!(cpu.push_u16(mem, cpu.bc()));
    0xD5, 1, 16, PUSH_DE => unborrow!(cpu.push_u16(mem, cpu.de()));
    0xE5, 1, 16, PUSH_HL => unborrow!(cpu.push_u16(mem, cpu.hl()));
    0xF1, 1, 12, POP_AF => unborrow!(cpu.set_af(cpu.pop_u16(mem)));
    0xC1, 1, 12, POP_BC => unborrow!(cpu.set_bc(cpu.pop_u16(mem)));
    0xD1, 1, 12, POP_DE => unborrow!(cpu.set_de(cpu.pop_u16(mem)));
    0xE1, 1, 12, POP_HL => unborrow!(cpu.set_hl(cpu.pop_u16(mem)));
    0x87, 1,  4, ADD_A => unborrow!(cpu.add(cpu.a()));
    0x80, 1,  4, ADD_B => unborrow!(cpu.add(cpu.b()));
    0x81, 1,  4, ADD_C => unborrow!(cpu.add(cpu.c()));
    0x82, 1,  4, ADD_D => unborrow!(cpu.add(cpu.d()));
    0x83, 1,  4, ADD_E => unborrow!(cpu.add(cpu.e()));
    0x84, 1,  4, ADD_H => unborrow!(cpu.add(cpu.h()));
    0x85, 1,  4, ADD_L => unborrow!(cpu.add(cpu.l()));
    0x29, 1,  8, ADD_HL_HL => unborrow!(cpu.add_hl(cpu.hl()));
    0xC6, 2,  8, ADD_n(amount: u8) => cpu.add(amount);
    0x97, 1,  4, SUB_A => unborrow!(cpu.sub(cpu.a()));
    0x90, 1,  4, SUB_B => unborrow!(cpu.sub(cpu.b()));
    0x91, 1,  4, SUB_C => unborrow!(cpu.sub(cpu.c()));
    0x92, 1,  4, SUB_D => unborrow!(cpu.sub(cpu.d()));
    0x93, 1,  4, SUB_E => unborrow!(cpu.sub(cpu.e()));
    0x94, 1,  4, SUB_H => unborrow!(cpu.sub(cpu.h()));
    0x95, 1,  4, SUB_L => unborrow!(cpu.sub(cpu.l()));
    0xD6, 2,  8, SUB_n(amount: u8) => cpu.sub(amount);
    0x8F, 1,  4, ADC_A => unborrow!(cpu.add_carry(cpu.a()));
    0x88, 1,  4, ADC_B => unborrow!(cpu.add_carry(cpu.b()));
    0x89, 1,  4, ADC_C => unborrow!(cpu.add_carry(cpu.c()));
    0x8A, 1,  4, ADC_D => unborrow!(cpu.add_carry(cpu.d()));
    0x8B, 1,  4, ADC_E => unborrow!(cpu.add_carry(cpu.e()));
    0x8C, 1,  4, ADC_H => unborrow!(cpu.add_carry(cpu.h()));
    0x8D, 1,  4, ADC_L => unborrow!(cpu.add_carry(cpu.l()));
    0xCE, 2,  8, ADC_n(amount: u8) => cpu.add_carry(amount);
    0x9F, 1,  4, SBC_A => unborrow!(cpu.sub_carry(cpu.a()));
    0x98, 1,  4, SBC_B => unborrow!(cpu.sub_carry(cpu.b()));
    0x99, 1,  4, SBC_C => unborrow!(cpu.sub_carry(cpu.c()));
    0x9A, 1,  4, SBC_D => unborrow!(cpu.sub_carry(cpu.d()));
    0x9B, 1,  4, SBC_E => unborrow!(cpu.sub_carry(cpu.e()));
    0x9C, 1,  4, SBC_H => unborrow!(cpu.sub_carry(cpu.h()));
    0x9D, 1,  4, SBC_L => unborrow!(cpu.sub_carry(cpu.l()));
    0xDE, 2,  8, SBC_n(amount: u8) => cpu.sub_carry(amount);
    0x3C, 1,  4, INC_A => cpu.increment::<A>();
    0x04, 1,  4, INC_B => cpu.increment::<B>();
    0x0C, 1,  4, INC_C => cpu.increment::<C>();
    0x14, 1,  4, INC_D => cpu.increment::<D>();
    0x1C, 1,  4, INC_E => cpu.increment::<E>();
    0x24, 1,  4, INC_H => cpu.increment::<H>();
    0x2C, 1,  4, INC_L => cpu.increment::<L>();
    0x03, 1,  8, INC_BC => cpu.increment_bc();
    0x13, 1,  8, INC_DE => cpu.increment_de();
    0x23, 1,  8, INC_HL => cpu.increment_hl();
    0x34, 1, 12, INC_MHL => cpu.increment_mhl(mem);
    0x3D, 1,  4, DEC_A => cpu.decrement::<A>();
    0x05, 1,  4, DEC_B => cpu.decrement::<B>();
    0x0D, 1,  4, DEC_C => cpu.decrement::<C>();
    0x15, 1,  4, DEC_D => cpu.decrement::<D>();
    0x1D, 1,  4, DEC_E => cpu.decrement::<E>();
    0x25, 1,  4, DEC_H => cpu.decrement::<H>();
    0x2D, 1,  4, DEC_L => cpu.decrement::<L>();
    0x35, 1,  12, DEC_MHL => cpu.decrement_mhl(mem);
    0xB7, 1,  4, OR_A => unborrow!(cpu.or(cpu.a()));
    0xB0, 1,  4, OR_B => unborrow!(cpu.or(cpu.b()));
    0xB1, 1,  4, OR_C => unborrow!(cpu.or(cpu.c()));
    0xB2, 1,  4, OR_D => unborrow!(cpu.or(cpu.d()));
    0xB3, 1,  4, OR_E => unborrow!(cpu.or(cpu.e()));
    0xB4, 1,  4, OR_H => unborrow!(cpu.or(cpu.h()));
    0xB5, 1,  4, OR_L => unborrow!(cpu.or(cpu.l()));
    0xB6, 1,  8, OR_MHL => unborrow!(cpu.or(mem.read_u8(Addr(cpu.hl()))));
    0xA7, 1,  4, AND_A => unborrow!(cpu.and(cpu.a()));
    0xA0, 1,  4, AND_B => unborrow!(cpu.and(cpu.b()));
    0xA1, 1,  4, AND_C => unborrow!(cpu.and(cpu.c()));
    0xA2, 1,  4, AND_D => unborrow!(cpu.and(cpu.d()));
    0xA3, 1,  4, AND_E => unborrow!(cpu.and(cpu.e()));
    0xA4, 1,  4, AND_H => unborrow!(cpu.and(cpu.h()));
    0xA5, 1,  4, AND_L => unborrow!(cpu.and(cpu.l()));
    0xE6, 2,  8, AND_n(value: u8) => cpu.and(value);
    0xAE, 1,  8, XOR_MHL => unborrow!(cpu.xor(mem.read_u8(Addr(cpu.hl()))));
    0xAF, 1,  4, XOR_A => unborrow!(cpu.xor(cpu.a()));
    0xA8, 1,  4, XOR_B => unborrow!(cpu.xor(cpu.b()));
    0xA9, 1,  4, XOR_C => unborrow!(cpu.xor(cpu.c()));
    0xAA, 1,  4, XOR_D => unborrow!(cpu.xor(cpu.d()));
    0xAB, 1,  4, XOR_E => unborrow!(cpu.xor(cpu.e()));
    0xAC, 1,  4, XOR_H => unborrow!(cpu.xor(cpu.h()));
    0xAD, 1,  4, XOR_L => unborrow!(cpu.xor(cpu.l()));
    0xEE, 2,  8, XOR_n(value: u8) => cpu.xor(value);
    0xBF, 1,  4, CP_A => unborrow!(cpu.compare(cpu.a()));
    0xB8, 1,  4, CP_B => unborrow!(cpu.compare(cpu.b()));
    0xB9, 1,  4, CP_C => unborrow!(cpu.compare(cpu.c()));
    0xBA, 1,  4, CP_D => unborrow!(cpu.compare(cpu.d()));
    0xBB, 1,  4, CP_E => unborrow!(cpu.compare(cpu.e()));
    0xBC, 1,  4, CP_H => unborrow!(cpu.compare(cpu.h()));
    0xBD, 1,  4, CP_L => unborrow!(cpu.compare(cpu.l()));
    0xFE, 2,  8, CP_n(value: u8) => unborrow!(cpu.compare(value));
    0x2F, 1,  4, CPL => unborrow!(cpu.set_a(cpu.a() ^ 0xFF));
    0x07, 1,  4, RLC_A => cpu.rotate_left_carry::<A>();
    0x17, 1,  4, RL_A => cpu.rotate_left_a();
    0x0F, 1,  4, RRC_A =>cpu.rotate_right_carry::<A>();
    0x1F, 1,  4, RR_A => cpu.rotate_right_a();
    0x37, 1,  4, SCF => cpu.set_carry_flag();
    0x3F, 1,  4, CCF => cpu.complement_carry_flag();
    0xCB, instr.len() + 1, instr.cycles(), Extended(instr: ExtendedInstruction) => instr.execute(cpu, mem);
}

instructions! {
    ExtendedInstruction
    |cpu, mem, addr|
    // op, len, cycles
    0x07, 1,  8, RLC_A => cpu.rotate_left_carry::<A>();
    0x00, 1,  8, RLC_B => cpu.rotate_left_carry::<B>();
    0x01, 1,  8, RLC_C => cpu.rotate_left_carry::<C>();
    0x02, 1,  8, RLC_D => cpu.rotate_left_carry::<D>();
    0x03, 1,  8, RLC_E => cpu.rotate_left_carry::<E>();
    0x04, 1,  8, RLC_H => cpu.rotate_left_carry::<H>();
    0x05, 1,  8, RLC_L => cpu.rotate_left_carry::<L>();
    0x0F, 1,  8, RRC_A => cpu.rotate_right_carry::<A>();
    0x08, 1,  8, RRC_B => cpu.rotate_right_carry::<B>();
    0x09, 1,  8, RRC_C => cpu.rotate_right_carry::<C>();
    0x0A, 1,  8, RRC_D => cpu.rotate_right_carry::<D>();
    0x0B, 1,  8, RRC_E => cpu.rotate_right_carry::<E>();
    0x0C, 1,  8, RRC_H => cpu.rotate_right_carry::<H>();
    0x0D, 1,  8, RRC_L => cpu.rotate_right_carry::<L>();
    0x38, 1,  8, SRL_B => cpu.shift_right_logical_b();
    0x19, 1,  8, RR_C => cpu.rotate_right_c();
    0x1A, 1,  8, RR_D => cpu.rotate_right_d();
    0x1B, 1,  8, RR_E => cpu.rotate_right_e();
    0x37, 1,  8, SWAP_A => cpu.swap_nibbles_a();
    0x30, 1,  8, SWAP_B => cpu.swap_nibbles_b();
    0x31, 1,  8, SWAP_C => cpu.swap_nibbles_c();
    0x32, 1,  8, SWAP_D => cpu.swap_nibbles_d();
    0x33, 1,  8, SWAP_E => cpu.swap_nibbles_e();
    0x34, 1,  8, SWAP_H => cpu.swap_nibbles_h();
    0x35, 1,  8, SWAP_L => cpu.swap_nibbles_l();
}
