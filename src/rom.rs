use std::path::Path;
use std::io::{self, Read};
use std::fs::File;

pub struct Rom {
    pub data: Vec<u8>
}

impl Rom {
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Rom> {
        let mut rom = Rom {
            data: Vec::new()
        };

        let mut file = try!(File::open(path));

        try!(file.read_to_end(&mut rom.data));

        Ok(rom)
    }
}

pub enum Type {
    ROM                       = 0x00,
    ROM_MBC1                  = 0x01,
    ROM_MBC1_RAM              = 0x02,
    ROM_MBC1_RAM_BATT         = 0x03,
    ROM_MBC2                  = 0x05,
    ROM_MBC2_BATTERY          = 0x06,
    ROM_RAM                   = 0x08,
    ROM_RAM_BATTERY           = 0x09,
    ROM_MMM01                 = 0x0B,
    ROM_MMM01_SRAM            = 0x0C,
    ROM_MMM01_SRAM_BATT       = 0x0D,
    ROM_MBC3_RAM              = 0x12,
    ROM_MBC3_RAM_BATT         = 0x13,
    ROM_MBC5                  = 0x19,
    ROM_MBC5_RAM              = 0x1A,
    ROM_MBC5_RAM_BATT         = 0x1B,
    ROM_MBC5_RUMBLE           = 0x1C,
    ROM_MBC5_RUMBLE_SRAM      = 0x1D,
    ROM_MBC5_RUMBLE_SRAM_BATT = 0x1E,
    PocketCamera              = 0x1F,
    BandaiTAMA5               = 0xFD,
    HudsonHuC3                = 0xFE,
}
