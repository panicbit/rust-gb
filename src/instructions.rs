use byteorder::{ByteOrder, LittleEndian};
use cpu::Cpu;
use bit_range::BitRange;
use memory::*;

type LE = LittleEndian;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    NOP,
    DI,
    // Load into A
    LD_A_A,
    LD_A_B,
    LD_A_C,
    LD_A_D,
    LD_A_E,
    LD_A_H,
    LD_A_L,
    LD_A_MC,
    LD_A_MBC,
    LD_A_MDE,
    LD_A_MHL,
    LD_A_Mn(u8),
    LD_A_n(u8),
    LD_A_nn(u16),
    // Load into B
    LD_B_A,
    LD_B_B,
    LD_B_C,
    LD_B_D,
    LD_B_E,
    LD_B_H,
    LD_B_L,
    LD_B_MHL,
    LD_B_n(u8),
    // Load into C
    LD_C_A,
    LD_C_B,
    LD_C_C,
    LD_C_D,
    LD_C_E,
    LD_C_H,
    LD_C_L,
    LD_C_MHL,
    LD_C_n(u8),
    // Load into D
    LD_D_A,
    LD_D_B,
    LD_D_C,
    LD_D_D,
    LD_D_E,
    LD_D_H,
    LD_D_L,
    LD_D_MHL,
    LD_D_n(u8),
    // Load into E
    LD_E_A,
    LD_E_B,
    LD_E_C,
    LD_E_D,
    LD_E_E,
    LD_E_H,
    LD_E_L,
    LD_E_MHL,
    LD_E_n(u8),
    // Load into H
    LD_H_A,
    LD_H_B,
    LD_H_C,
    LD_H_D,
    LD_H_E,
    LD_H_H,
    LD_H_L,
    LD_H_MHL,
    LD_H_n(u8),
    // Load into L
    LD_L_A,
    LD_L_B,
    LD_L_C,
    LD_L_D,
    LD_L_E,
    LD_L_H,
    LD_L_L,
    LD_L_MHL,
    LD_L_n(u8),
    // Load into BC
    LD_BC_nn(u16),
    // Load into DE
    LD_DE_nn(u16),
    // Load into HL
    LD_HL_nn(u16),
    // Load into SP
    LD_SP_nn(u16),
    LD_SP_HL,
    LD_MC_A,
    LD_MBC_A,
    LD_MDE_A,
    // Load into *HL
    LD_MHL_A,
    LD_MHL_B,
    LD_MHL_C,
    LD_MHL_D,
    LD_MHL_E,
    LD_MHL_H,
    LD_MHL_L,
    LD_MHL_n(u8),
    // Load into *(FF00+n)
    LD_Mn_A(u8),
    // Load into *nn
    LD_Mnn_A(u16),
    LD_Mnn_SP(u16),
    LDI_A_MHL,
    LDI_MHL_A,
    LDD_A_MHL,
    LDHL_SP_n(i8),
    // PUSH
    PUSH_AF,
    PUSH_BC,
    PUSH_DE,
    PUSH_HL,
    // POP
    POP_AF,
    POP_BC,
    POP_DE,
    POP_HL,
    // ADD to A
    ADD_A_A,
    ADD_A_B,
    ADD_A_C,
    ADD_A_D,
    ADD_A_E,
    ADD_A_H,
    ADD_A_L,
    ADD_A_MHL,
    ADD_A_n(u8),
    // ADD to HL
    ADD_HL_BC,
    ADD_HL_DE,
    ADD_HL_HL,
    ADD_HL_SP,
    // ADD to SP
    ADD_SP_n(i8),
    // ADD Carry + x to A
    ADC_A_A,
    ADC_A_B,
    ADC_A_C,
    ADC_A_D,
    ADC_A_E,
    ADC_A_H,
    ADC_A_L,
    ADC_A_MHL,
    ADC_A_n(u8),
    // SUB x from A
    SUB_A,
    SUB_B,
    SUB_C,
    SUB_D,
    SUB_E,
    SUB_H,
    SUB_L,
    SUB_MHL,
    SUB_n(u8),
    // SUB Carry + x from A
    SBC_A_A,
    SBC_A_B,
    SBC_A_C,
    SBC_A_D,
    SBC_A_E,
    SBC_A_H,
    SBC_A_L,
    SBC_A_MHL,
    SBC_A_n(u8),
    // AND x with A, result in A
    AND_A,
    AND_B,
    AND_C,
    AND_D,
    AND_E,
    AND_H,
    AND_L,
    AND_MHL,
    AND_n(u8),
    // OR x with A, result in A
    OR_A,
    OR_B,
    OR_C,
    OR_D,
    OR_E,
    OR_H,
    OR_L,
    OR_MHL,
    OR_n(u8),
    // XOR x with A, result in A
    XOR_A,
    XOR_B,
    XOR_C,
    XOR_D,
    XOR_E,
    XOR_H,
    XOR_L,
    XOR_MHL,
    XOR_n(u8),
    // Compare A with x
    CP_A,
    CP_B,
    CP_C,
    CP_D,
    CP_E,
    CP_H,
    CP_L,
    CP_MHL,
    CP_n(u8),
    // Increment x
    INC_A,
    INC_B,
    INC_C,
    INC_D,
    INC_E,
    INC_H,
    INC_L,
    INC_BC,
    INC_HL,
    INC_MHL,
    INC_nn(u16),
    // Decrement x
    DEC_A,
    DEC_B,
    DEC_C,
    DEC_D,
    DEC_E,
    DEC_H,
    DEC_L,
    DEC_MHL,
    DEC_nn(u16),
    // Jump
    JP_nn(u16),
    JR_n(u8),
    // CALL
    CALL_nn(u16),
    // RET
    RET,
    // RST
    RST_00,
    RST_08,
    RST_10,
    RST_18,
    RST_20,
    RST_28,
    RST_30,
    RST_38,
}

impl Instruction {
    pub fn decode(mem: &mut Memory, addr: Addr) -> Instruction {
        use self::Instruction::*;

        let op = mem.read_u8(addr);
        let addr = addr + 1;

        match op {
            0x00 => NOP,
            0xF3 => DI,
            0x78 => LD_A_B,
            0x7C => LD_A_H,
            0x7D => LD_A_L,
            0x01 => LD_BC_nn(mem.read_u16(addr)),
            0x3E => LD_A_n(mem.read_u8(addr)),
            0x06 => LD_B_n(mem.read_u8(addr)),
            0x11 => LD_DE_nn(mem.read_u16(addr)),
            0x21 => LD_HL_nn(mem.read_u16(addr)),
            0x31 => LD_SP_nn(mem.read_u16(addr)),
            0xE0 => LD_Mn_A(mem.read_u8(addr)),
            0xEA => LD_Mnn_A(mem.read_u16(addr)),
            0x2A => LDI_A_MHL,
            0xF5 => PUSH_AF,
            0xC5 => PUSH_BC,
            0xE5 => PUSH_HL,
            0xF1 => POP_AF,
            0xE1 => POP_HL,
            0x04 => INC_B,
            0x03 => INC_BC,
            0x23 => INC_HL,
            0xB1 => OR_B,
            0xC3 => JP_nn(mem.read_u16(addr)),
            0x18 => JR_n(mem.read_u8(addr)),
            0xCD => CALL_nn(mem.read_u16(addr)),
            0xC9 => RET,
            op => panic!("opcode 0x{:02X}", op)
        }
    }

    pub fn len(&self) -> u16 {
        use self::Instruction::*;
        match *self {
            NOP => 1,
            DI => 1,
            LD_A_B => 1,
            LD_A_H => 1,
            LD_A_L => 1,
            LD_A_n(_) => 2,
            LD_B_n(_) => 2,
            LD_BC_nn(_) => 3,
            LD_DE_nn(_) => 3,
            LD_HL_nn(_) => 3,
            LD_SP_nn(_) => 3,
            LD_Mn_A(_) => 2,
            LD_Mnn_A(_) => 3,
            LDI_A_MHL => 1,
            PUSH_AF => 1,
            PUSH_BC => 1,
            PUSH_HL => 1,
            POP_AF => 1,
            POP_HL => 1,
            INC_B => 1,
            INC_BC => 1,
            INC_HL => 1,
            OR_B => 1,
            JP_nn(_) => 3,
            JR_n(_) => 2,
            CALL_nn(_) => 3,
            RET => 1,
            inst => panic!("instruction len {:?}", inst)
        }   
    }

    pub fn cycles(&self) -> usize {
        use self::Instruction::*;
        match *self {
            NOP => 4,
            DI => 4,
            LD_A_B => 4,
            LD_A_H => 4,
            LD_A_L => 4,
            LD_A_n(_) => 8,
            LD_B_n(_) => 8,
            LD_BC_nn(_) => 12,
            LD_DE_nn(_) => 12,
            LD_HL_nn(_) => 12,
            LD_SP_nn(_) => 12,
            LD_Mn_A(_) => 12,
            LD_Mnn_A(_) => 16,
            LDI_A_MHL => 8,
            PUSH_AF => 16,
            PUSH_BC => 16,
            PUSH_HL => 16,
            POP_AF => 12,
            POP_HL => 12,
            INC_B => 4,
            INC_HL => 8,
            INC_BC => 8,
            OR_B => 4,
            JP_nn(_) => 12,
            JR_n(_) => 8,
            CALL_nn(_) => 12,
            RET => 8,
            inst => panic!("instruction cycles {:?}", inst)
        }
    }

    pub fn execute(&self, cpu: &mut Cpu, mem: &mut Memory) {
        use self::Instruction::*;
        println!("OP: {:?}", self);
        match *self {
            NOP => {},
            DI => cpu.disable_interrupts(),
            LD_A_B => { let value = cpu.b(); cpu.set_a(value); },
            LD_A_H => { let value = cpu.h(); cpu.set_a(value); },
            LD_A_L => { let value = cpu.l(); cpu.set_a(value); },
            LD_A_n(value) => cpu.set_a(value),
            LD_B_n(value) => cpu.set_b(value),
            LD_BC_nn(value) => cpu.set_bc(value),
            LD_DE_nn(value) => cpu.set_de(value),
            LD_HL_nn(value) => cpu.set_hl(value),
            LD_SP_nn(value) => cpu.set_sp(value),
            LD_Mnn_A(p) => mem.write_u16(Addr(p), cpu.a() as u16),
            LD_Mn_A(p) => mem.write_u8(Addr(0xFF00) + p as u16, cpu.a()),
            LDI_A_MHL => {
                let hl = cpu.hl();
                cpu.set_a(mem.read_u8(Addr(hl)));
            },
            PUSH_AF => { let value = cpu.af(); cpu.push_u16(mem, value); },
            PUSH_BC => { let value = cpu.bc(); cpu.push_u16(mem, value); },
            PUSH_HL => { let value = cpu.hl(); cpu.push_u16(mem, value); },
            POP_AF => { let value = cpu.pop_u16(mem); cpu.set_af(value); },
            POP_HL => { let value = cpu.pop_u16(mem); cpu.set_hl(value); },
            INC_B => cpu.incr_b(),
            INC_BC => cpu.incr_bc(),
            INC_HL => cpu.incr_hl(),
            OR_B => {
                let value = cpu.a() | cpu.b();
                cpu.set_a(value);
            }
            JP_nn(pc) => cpu.set_pc(pc),
            CALL_nn(addr) => {
                let pc = cpu.pc();
                cpu.push_u16(mem, pc);
                cpu.set_pc(addr);
            },
            JR_n(offset) => {
                let pc = cpu.pc();
                cpu.set_pc(pc + offset as u16);
            }
            RET => { let pc = cpu.pop_u16(mem); cpu.set_pc(pc); },
            inst => panic!("instruction exec {:?}", inst)
        }
    }
}