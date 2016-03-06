use byteorder::{ByteOrder, LittleEndian};
use cpu::Cpu;
use bit_range::BitRange;
use memory::*;

type LE = LittleEndian;

macro_rules! instructions {
    (|$cpu:ident, $mem:ident, $addr:ident|
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
        pub enum Instruction {
            $(
                $name$({ $( $p_name : $p_ty ),+ })*
            ),*
        }

        impl Instruction {
            pub fn decode($mem: &mut Memory, $addr: Addr) -> Instruction {
                use self::Instruction::*;

                let op = $mem.read_u8($addr);
                let mut $addr = $addr + 1;

                match op {
                    $(
                        $op => $name$({ $( $p_name : Param::get($mem, &mut $addr) ),+ })*
                    ),*,
                    op => panic!("opcode 0x{:02X}", op)
                }
            }

            pub fn len(&self) -> u16 {
                use self::Instruction::*;
                match *self {
                    $(
                        $name$({ $( $p_name ),+ })* => $len
                    ),*,
                }
            }

            pub fn cycles(&self) -> u16 {
                use self::Instruction::*;
                match *self {
                    $(
                        $name$({ $( $p_name ),+ })* => $cycles
                    ),*,
                }
            }

            pub fn execute(&self, $cpu: &mut Cpu, $mem: &mut Memory) {
                use self::Instruction::*;
                println!("OP: {:?}", self);
                match *self {
                    $(
                        $name$({ $( $p_name ),+ })* => {$exec;}
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

impl Param for u16 {
    fn get(mem: &mut Memory, addr: &mut Addr) -> Self {
        *addr = *addr + 2;
        mem.read_u16(*addr - 2)
    }
}

instructions! {
    |cpu, mem, addr|
    // op, len, cycles
    0x00, 1, 4, NOP => {};
    0xF3, 1, 4, DI => cpu.disable_interrupts();
    0xC3, 3, 12, JP_nn(pc: u16) => cpu.set_pc(pc);
    0x78, 1, 4, LD_A_B => { let value = cpu.b(); cpu.set_a(value) };
    0x7C, 1, 4, LD_A_H => { let value = cpu.h(); cpu.set_a(value) };
    0x7D, 1, 4, LD_A_L => { let value = cpu.l(); cpu.set_a(value) };
    0x3E, 2, 8, LD_A_n(value: u8) => cpu.set_a(value);
    0x01, 3, 8, LD_BC_nn(value: u16) => cpu.set_bc(value);
    0x21, 3, 12, LD_HL_nn(value: u16) => cpu.set_hl(value);
    0x31, 3, 12, LD_SP_nn(value: u16) => cpu.set_sp(value);
    0xEA, 3, 16, LD_Mnn_A(p: u16) => mem.write_u16(Addr(p), cpu.a() as u16);
    0xE0, 2, 12, LD_Mn_A(p: u8) => mem.write_u8(Addr(0xFF00) + p as u16, cpu.a());
    0x2A, 1, 8, LDI_A_MHL => {
        let hl = cpu.hl();
        cpu.set_a(mem.read_u8(Addr(hl)));
    };
    0xCD, 3, 12, CALL_nn(addr: u16) => {
        let pc = cpu.pc();
        cpu.push_u16(mem, pc);
        cpu.set_pc(addr);
    };
    0x18, 2, 8, JR_n(offset: u8) => {
        let pc = cpu.pc();
        cpu.set_pc(pc + offset as u16);
    };
    0xC9, 1, 8, RET => { let pc = cpu.pop_u16(mem); cpu.set_pc(pc) };
    0xF5, 1, 16, PUSH_AF => { let value = cpu.af(); cpu.push_u16(mem, value) };
    0xC5, 1, 16, PUSH_BC => { let value = cpu.bc(); cpu.push_u16(mem, value) };
    0xE5, 1, 16, PUSH_HL => { let value = cpu.hl(); cpu.push_u16(mem, value) };
    0xF1, 1, 12, POP_AF => { let value = cpu.pop_u16(mem); cpu.set_af(value) };
    0xE1, 1, 12, POP_HL => { let value = cpu.pop_u16(mem); cpu.set_hl(value) };
    0x04, 1, 4, INC_B => cpu.incr_b();
    0x03, 1, 8, INC_BC => cpu.incr_bc();
    0x23, 1, 8, INC_HL => cpu.incr_hl();
    0xB1, 1, 4, OR_B => {
        let value = cpu.a() | cpu.b();
        cpu.set_a(value);
    };
    0x28, 2, 8, JR_Z(offset: u8) => if cpu.flag_z() {
        let pc = cpu.pc;
        cpu.set_pc(pc + offset as u16)
    };
}
