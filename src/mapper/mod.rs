use memory::Addr;
use rom::{Rom, Type};
mod mbc1;

pub trait Mapper {
    fn read_u8(&mut self, rom: &[u8], addr: Addr) -> u8;
    fn write_u8(&mut self, rom: &[u8], addr: Addr, value: u8);
}

pub fn from_rom(rom: &Rom) -> Box<Mapper> {
    match rom.typ() {
        Type::RomMbc1 => Box::new(mbc1::Mbc1::new()),
        typ => panic!("Mapper not implemented: {:?}", typ)
    }
}
