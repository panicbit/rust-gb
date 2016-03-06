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
