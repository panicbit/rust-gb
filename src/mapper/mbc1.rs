use super::Mapper;
use memory::Addr;

pub struct Mbc1;

impl Mbc1 {
    pub fn new() -> Mbc1 {
        Mbc1
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
        panic!("MBC1_STUB: write at 0x{:04X}", *addr);
    }
}
