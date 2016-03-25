use std::path::Path;
use std::io::{self, Read};
use std::fs::File;
use conv::TryFrom;

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

    pub fn typ(&self) -> Type {
        Type::try_from(::header::typ(&self.data)).unwrap_or(Type::UNKNOWN)
    }

    pub fn rom_size(&self) -> RomSize {
        RomSize::try_from(::header::rom_size(&self.data)).unwrap_or(RomSize::UNKNOWN)
    }

    pub fn ram_size(&self) -> RamSize {
        RamSize::try_from(::header::ram_size(&self.data)).unwrap_or(RamSize::UNKNOWN)
    }
}

custom_derive! {
    #[allow(non_camel_case_types)]
    #[derive(TryFrom(u8),Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Type {
        Rom                   = 0x00,
        RomMbc1               = 0x01,
        RomMbc1_Ram           = 0x02,
        RomMbc1RamBatt        = 0x03,
        RomMbc2               = 0x05,
        RomMbc2Batt           = 0x06,
        RomRam                = 0x08,
        RomRamBatt            = 0x09,
        RomMmm01              = 0x0B,
        RomMmm01Sram          = 0x0C,
        RomMmm01SramBatt      = 0x0D,
        RomMbc3Ram            = 0x12,
        RomMbc3RamBatt        = 0x13,
        RomMbc5               = 0x19,
        RomMbc5Ram            = 0x1A,
        RomMbc5RamBatt        = 0x1B,
        RomMbc5Rumble         = 0x1C,
        RomMbc5RumbleSram     = 0x1D,
        RomMbc5RumbleSramBatt = 0x1E,
        PocketCamera          = 0x1F,
        BandaiTAMA5           = 0xFD,
        HudsonHuC3            = 0xFE,
        UNKNOWN
    }
}

custom_derive! {
    #[allow(non_camel_case_types)]
    #[derive(TryFrom(u8),Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum RomSize {
        Kb32  = 0x00,
        Kb64  = 0x01,
        Kb128 = 0x02,
        Kb256 = 0x03,
        Kb512 = 0x04,
        Mb1   = 0x05,
        Mb2   = 0x06,
        Mb4   = 0x07,
        Mb1_1 = 0x52,
        Mb1_2 = 0x53,
        Mb1_5 = 0x54,
        UNKNOWN
    }
}

custom_derive! {
    #[derive(TryFrom(u8),Debug, Eq, PartialEq, Ord, PartialOrd)]
    pub enum RamSize {
        None  = 0x00,
        Kb2   = 0x01,
        Kb8   = 0x02,
        Kb32  = 0x03,
        Kb128 = 0x04,
        Kb64  = 0x05,
        UNKNOWN
    }
}