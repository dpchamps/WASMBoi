

use wasmboi::spec::cpu::CPU;
use wasmboi::spec::gameboy::{Peripheral, GameBoy};
use std::env;
use std::fs;

fn main() {
    let rom_location = env::var("ROM").unwrap();
    // println!("Loading ${}", rom_location);
    let rom = fs::read(rom_location).unwrap();

    let mut gameboy = GameBoy::new(&rom).unwrap_or_else(|e| {
        panic!(
            "Failed to initialize GameBoy with the following error: {:?}",
            e
        )
    });

    gameboy.attach_peripheral(Peripheral::SerialPort(Box::new(|c| {
        if env::var("SERIAL_PORT_STDOUT").unwrap_or("false".into()) == "true" {
            if let Some(x) = c {
                print!("{}", x)
            }
        }
    })));

    gameboy.start().expect("Gameboy failed")
}
