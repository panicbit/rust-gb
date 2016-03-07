use std::ops::{Deref, Add, Sub, Index};
use rom::Rom;
use monster::incubation::SplitInt;

pub struct Memory {
    stack: [u8; 128], // 0xFF = IF
    ram: [u8; 8*1024],
    rom: Rom,
}

impl Memory {
    pub fn new(rom: Rom) -> Self {
        Memory {
            stack: [0; 128],
            ram: [0; 8*1024],
            rom: rom,
        }
    }

    pub fn read_u8(&mut self, addr: Addr) -> u8 {
        fn read_stub(msg: &str, addr: u16, value: u8) -> u8 {
            println!("READ_STUB: 0x{:04X} {}", addr, msg);
            value
        }
        use self::Location::*;
        match Location::from_addr(*addr) {
            InterruptEnable => read_stub("IE register", *addr, 0),
            InternalRam128(offset) => self.stack[offset as usize],
            Empty => 0,
            SerialPort => read_stub("serial port", *addr, 0),
            IOStub => read_stub("I/O port", *addr, 0),
            OAM(_offset) => read_stub("OAM access", *addr, 0),
            InternalRam8k(offset) => self.ram[offset as usize],
            VRAM(_offset) => read_stub("Video RAM read", *addr, 0),
            ROM0(offset) => {
                debug_assert!(self.rom.data.len() >= 0x4000);
                self.rom.data[offset as usize]
            }
            Stub => panic!("READ_STUB: 0x{:02X}", *addr)
        }
    }

    pub fn write_u8(&mut self, addr: Addr, value: u8) {
        fn write_stub(msg: &str, addr: u16, value: u8) {
            println!("WRITE_STUB: 0x{:04X} ← 0x{:02X} {}", addr, value, msg);
        }
        use self::Location::*;
        match Location::from_addr(*addr) {
            InterruptEnable => write_stub("IE register", *addr, value),
            InternalRam128(offset) => self.stack[offset as usize] = value,
            Empty => {},
            SerialPort => panic!("STUB: Write to serial port: {}", value as char),
            IOStub => write_stub("I/O port write", *addr, value),
            OAM(_offset) => write_stub("OAM", *addr, value),
            InternalRam8k(offset) => self.ram[offset as usize] = value,
            VRAM(_offset) => write_stub("Video RAM", *addr, value),
            ROM0(offset) => write_stub("ROM bank #0", *addr, value),
            Stub => panic!("WRITE_STUB: 0x{:02X} ← 0x{:02X}", *addr, value)
        }
    }

    pub fn read_u16(&mut self, addr: Addr) -> u16 {
        let low  = self.read_u8(addr    );
        let high = self.read_u8(addr + 1);
        
        (high as u16) << 8 | low as u16
    }

    pub fn write_u16(&mut self, addr: Addr, value: u16) {
        let (high, low) = value.split();

        self.write_u8(addr    , low );
        self.write_u8(addr + 1, high);
    }

    pub fn size(&self) -> u16 {
        0xFFFF
    }
}

pub enum Location {
    InterruptEnable,
    InternalRam128(u16),
    SerialPort,
    Empty,
    IOStub,
    OAM(u16),
    InternalRam8k(u16),
    VRAM(u16),
    ROM0(u16),
    Stub,
}

impl Location {
    fn from_addr(addr: u16) -> Location {
        use self::Location::*;
        match addr {
            0xFFFF            => InterruptEnable,
            0xFF80 ... 0xFFFE => InternalRam128(addr - 0xFF80),
            0xFF4C ... 0xFF7F => Empty,
            0xFF01            => SerialPort,
            0xFF00 ... 0xFF4B => IOStub,
            0xFEA0 ... 0xFEFF => Empty,
            0xFE00 ... 0xFE9F => OAM(addr - 0xFE00),
            0xE000 ... 0xFDFF => InternalRam8k(addr - 0xE000),
            0xC000 ... 0xDFFF => InternalRam8k(addr - 0xC000),
            0x8000 ... 0x9FFF => VRAM(addr - 0x8000),
            0x0000 ... 0x3FFF => ROM0(addr),
            _                 => Stub
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Addr(pub u16);

impl Addr {
    pub fn in_range(self, start: u16, end: u16) -> bool {
        let addr = self.0;
        start <= addr && addr < end
    }
}

impl Deref for Addr {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add<u16> for Addr {
    type Output = Addr;
    fn add(self, rhs: u16) -> Addr {
        Addr(self.wrapping_add(rhs))
    }
}

impl Sub<u16> for Addr {
    type Output = Addr;
    fn sub(self, rhs: u16) -> Addr {
        Addr(self.wrapping_sub(rhs))
    }
}

impl From<u16> for Addr {
    fn from(addr: u16) -> Self {
        Addr(addr)
    }
}
