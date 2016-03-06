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
        // 128 internal RAM (stack)
        if *addr >= 0xFF80 {
            self.stack[(*addr - 0xFF80) as usize]
        } else
        // Empty
        if addr.in_range(0xFF4C, 0xFF80) {
            0
        } else
        // I/O ports
        if addr.in_range(0xFF00, 0xFF4C) {
            println!("STUB: I/O port access: 0x{:02X}", *addr);
            0
        } else
        // OAM
        if addr.in_range(0xFE00, 0xFF00) {
            println!("STUB: OAM access: 0x{:02X}", *addr);
            0
        } else
        // Echo of 8K internal RAM
        if addr.in_range(0xE000, 0xFE00) {
            self.ram[(*addr - 0xE000) as usize]
        } else
        // 8K internal RAM
        if addr.in_range(0xC000, 0xE000) {
            self.ram[(*addr - 0xC000) as usize]
        } else
        // Video RAM
        if addr.in_range(0x8000, 0xA000) {
            println!("STUB: Video RAM read: 0x{:02X}", *addr);
            0
        } else
        // ROM bank #0
        if addr.in_range(0x0000, 0x4000) {
            debug_assert!(self.rom.data.len() >= 0x4000);
            self.rom.data[addr.0 as usize]
        } else {
            panic!("read addr stub: 0x{:02X}", *addr);
        }
    }

    pub fn write_u8(&mut self, addr: Addr, value: u8) {
        // 128 internal RAM (stack)
        if *addr >= 0xFF80 {
            self.stack[(*addr - 0xFF80) as usize] = value;
        } else
        // Empty
        if addr.in_range(0xFF4C, 0xFF80) {
        } else
        // I/O ports
        if addr.in_range(0xFF00, 0xFF4C) {
            println!("STUB: I/O port write: 0x{:02X}", *addr);
        } else
        // OAM
        if addr.in_range(0xFE00, 0xFF00) {
            println!("STUB: OAM write: 0x{:02X}", *addr);
        } else
        // Echo of 8K internal RAM
        if addr.in_range(0xE000, 0xFE00) {
            self.ram[(*addr - 0xE000) as usize] = value;
        } else
        // 8K internal RAM
        if addr.in_range(0xC000, 0xE000) {
            self.ram[(*addr - 0xC000) as usize] = value;
        } else
        // Video RAM
        if addr.in_range(0x8000, 0xA000) {
            println!("STUB: Video RAM write: 0x{:02X}", *addr);
        } else
        // ROM bank #0
        if addr.in_range(0x0000, 0x4000) {
            println!("STUB: ROM bank #0 write: 0x{:02X}", *addr);
        } else {
            panic!("write addr stub: 0x{:02X}", *addr);
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
