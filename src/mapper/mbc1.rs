use super::Mapper;
use memory::Addr;
use self::Mode::*;

pub struct Mbc1 {
    mode: Mode,
    rom_bank: u8,
}

impl Mbc1 {
    pub fn new() -> Mbc1 {
        Mbc1 {
            mode: Mode16MbitRom8KbyteRam,
            rom_bank: 1,
        }
    }

    fn select_memory_model(&mut self, value: u8) {
        match value & 0b1 {
            0 => self.mode = Mode16MbitRom8KbyteRam,
            1 => self.mode = Mode4MbitRom32KbyteRam,
            _ => unreachable!()
        }
    }

    fn select_rom_bank(&mut self, value: u8) {
        let bank = value & 0b11;
        if bank == 0 {
            self.rom_bank = 1;
        } else {
            self.rom_bank = bank;
        }
    }
}

impl Mapper for Mbc1 {
    fn read_u8(&mut self, rom: &[u8], addr: Addr) -> u8 {
        match *addr {
            0xA000 ... 0xBFFF => {
                println!("MCB1_STUB: read at RAM 0x{:04X}", *addr);
                0
            },
            0x4000 ... 0x7FFF => rom[*addr as usize],
            _ => panic!("MBC1_STUB: read at 0x{:04X}", *addr)
        }
    }

    fn write_u8(&mut self, rom: &[u8], addr: Addr, value: u8) {
        match self.mode {
            Mode16MbitRom8KbyteRam => match *addr {
                0x6000 ... 0x7FFF => self.select_memory_model(value),
                0x4000 ... 0x5FFF => self.select_rom_bank(value),
                _ => panic!("MBC1_STUB: {:?} write at 0x{:04X} ← 0b{:08b}", self.mode, *addr, value)
            },
            Mode4MbitRom32KbyteRam => panic!("MBC1_STUB: {:?} write at 0x{:04X} ← 0b{:08b}", self.mode, *addr, value)
        }
    }
}

#[derive(Debug)]
enum Mode {
    Mode16MbitRom8KbyteRam,
    Mode4MbitRom32KbyteRam,
}
