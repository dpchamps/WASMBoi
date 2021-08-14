#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
mod dasm;
mod spec;
mod util;

use std::fs;
use std::env;

fn main() {
    let rom_location = env::var("ROM").unwrap();
    println!("Loading ${}", rom_location);
    let rom = fs::read(rom_location).unwrap();
    let disassembly = match dasm::disassemble(&rom) {
        Ok(d) => d,
        Err(error) => {
            println!("{:?}", error);
            panic!();
        }
    };

    fs::write("testbed/output", disassembly).unwrap_or_else(|_| panic!("Couldnt write file"));
}
