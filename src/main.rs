extern crate byteorder;
extern crate monster;
extern crate bit_range;
#[macro_use] extern crate unborrow;
use std::ascii::AsciiExt;
use std::str;
use monster::incubation::SliceTakeWhile;
use bit_range::BitRange;

mod header;
mod cpu;
mod instructions;
mod memory;

use self::header::*;
use self::cpu::*;
use self::rom::*;
use self::memory::*;

fn main() {
    let rom = Rom::load("./gb-tests/cpu_instrs/cpu_instrs.gb").expect("rom");

    print_logo(logo(&rom.data));

    let mut memory = Memory::new(rom);
    let mut cpu = Cpu::new();

    for i in 0 .. ::std::u64::MAX {
        println!("Step: {}", i+1);
        cpu.step(&mut memory);
        // cpu.print_registers();
    }
}

mod rom {
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

}

// Kudos to Pokechu22: http://stackoverflow.com/a/24630503
fn matrix_from_logo(logo: &[u8]) -> [[bool; 48]; 8] {
    debug_assert!(logo.len() >= 0x30);

    let mut matrix = [[false; 48]; 8];

    fn from_chunk(chunk: &[u8], matrix: &mut [[bool; 48]]) {
        for row in 0 .. 2 {
            for col in 0 .. 12 {
                let byte = chunk[2 * col + row];
                for bit_index in 0..4 {
                    matrix[2 * row    ][4 * col + bit_index] = [byte].get_bit(bit_index as u32);
                } 
                for bit_index in 4..8 {
                    matrix[2 * row + 1][4 * col + bit_index - 4] = [byte].get_bit(bit_index as u32);
                }
            }
        }
    }

    from_chunk(&logo[..24], &mut matrix[..4]);
    from_chunk(&logo[24..], &mut matrix[4..]);

    matrix
}

fn print_logo(logo: &[u8]) {
    for line in matrix_from_logo(logo).iter() {
        for dot in line.iter() {
            if *dot {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}