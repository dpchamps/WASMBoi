#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
mod dasm;
mod mbc;
mod spec;
mod util;
#[macro_use]
extern crate impl_ops;
use crate::spec::cpu::CPU;
use std::env;
use std::fs;
use crate::spec::gameboy::Peripheral;

fn main() {
    let rom_location = env::var("ROM").unwrap();
    println!("Loading ${}", rom_location);
    let rom = fs::read(rom_location).unwrap();

    let mut gameboy = spec::gameboy::GameBoy::new(&rom).unwrap_or_else(|e| {
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
