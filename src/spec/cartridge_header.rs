#![allow(non_camel_case_types)]
use crate::util::byte_ops::*;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str;

#[derive(Debug)]
pub enum CartridgeError {
    InvalidCartridgeCode,
    InvalidCartridgeRomSize,
    InvalidCartridgeRamSize,
    BadRomData,
}

pub mod cartridge_header_address {
    pub const CHECKSUM: usize = 0x14E;
    pub const COMPLEMENT_CHECKSUM: usize = 0x14D;
    pub const MASK_ROM_VERSION: usize = 0x14C;
    pub const OLD_LICENSE_CODE: usize = 0x14B;
    pub const DESTINATION_CODE: usize = 0x14A;
    pub const CARTRIDGE_RAM_SIZE: usize = 0x149;
    pub const CARTRIDGE_ROM_SIZE: usize = 0x148;
    pub const CARTRIDGE_TYPE: usize = 0x147;
    pub const GAME_TITLE: usize = 0x134;
    pub const ENTRY: usize = 0x102;
}

pub const GAME_TITLE_LENGTH: usize = 0xF;

#[derive(Debug, PartialEq, Eq)]
pub enum CartridgeType {
    ROM,
    MBC1,
    MBC1_RAM,
    MBC1_RAM_BAT,
    MBC2,
    MBC2_BAT,
    ROM_RAM,
    ROM_RAM_BAT,
    MMM_01,
    MMM_01_RAM,
    MMM_01_RAM_BAT,
    MBC3_TIMER_BAT,
    MBC3_RAM_TIMER_BAT,
    MBC3,
    MBC3_RAM,
    MBC3_RAM_BAT,
    MBC4,
    MBC4_RAM,
    MBC4_RAM_BAT,
    MBC5,
    MBC5_RAM,
    MBC5_RAM_BAT,
}

impl Display for CartridgeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let str = match self {
            CartridgeType::ROM => "ROM ONLY",
            CartridgeType::MBC1 => "MBC1",
            CartridgeType::MBC1_RAM => "MBC1_RAM",
            CartridgeType::MBC1_RAM_BAT => "MBC1_RAM_BAT",
            CartridgeType::MBC2 => "MBC2",
            CartridgeType::MBC2_BAT => "MBC2_BAT",
            CartridgeType::ROM_RAM => "ROM_RAM",
            CartridgeType::ROM_RAM_BAT => "ROM_RAM_BAT",
            CartridgeType::MMM_01 => "MMM_01",
            CartridgeType::MMM_01_RAM => "MMM_01_RAM",
            CartridgeType::MMM_01_RAM_BAT => "MMM_01_RAM_BAT",
            CartridgeType::MBC3_TIMER_BAT => "MBC3_TIMER_BAT",
            CartridgeType::MBC3_RAM_TIMER_BAT => "MBC3_RAM_TIMER_BAT",
            CartridgeType::MBC3 => "MBC3",
            CartridgeType::MBC3_RAM => "MBC3_RAM",
            CartridgeType::MBC3_RAM_BAT => "MBC3_RAM_BAT",
            CartridgeType::MBC4 => "MBC4",
            CartridgeType::MBC4_RAM => "MBC4_RAM",
            CartridgeType::MBC4_RAM_BAT => "MBC4_RAM_BAT",
            CartridgeType::MBC5 => "MBC5",
            CartridgeType::MBC5_RAM => "MBC5_RAM",
            CartridgeType::MBC5_RAM_BAT => "MBC5_RAM_BAT",
        };

        write!(f, "{}", str)
    }
}

pub fn lookup_cartridge_type(input: u8) -> Result<CartridgeType, CartridgeError> {
    match input {
        0x0 => Ok(CartridgeType::ROM),
        0x1 => Ok(CartridgeType::MBC1),
        0x2 => Ok(CartridgeType::MBC1_RAM),
        0x3 => Ok(CartridgeType::MBC1_RAM_BAT),
        0x5 => Ok(CartridgeType::MBC2),
        0x6 => Ok(CartridgeType::MBC2_BAT),
        0x8 => Ok(CartridgeType::ROM_RAM),
        0x9 => Ok(CartridgeType::ROM_RAM_BAT),
        0xB => Ok(CartridgeType::MMM_01),
        0xC => Ok(CartridgeType::MMM_01_RAM),
        0xD => Ok(CartridgeType::MMM_01_RAM_BAT),
        0xF => Ok(CartridgeType::MBC3_TIMER_BAT),
        0x10 => Ok(CartridgeType::MBC3_RAM_TIMER_BAT),
        0x11 => Ok(CartridgeType::MBC3),
        0x12 => Ok(CartridgeType::MBC3_RAM),
        0x13 => Ok(CartridgeType::MBC3_RAM_BAT),
        0x19 => Ok(CartridgeType::MBC5),
        0x1A => Ok(CartridgeType::MBC5_RAM),
        0x1B => Ok(CartridgeType::MBC5_RAM_BAT),
        _ => Err(CartridgeError::InvalidCartridgeCode),
    }
}

pub fn lookup_cartridge_ram_size(input: u8) -> Result<usize, CartridgeError> {
    match input {
        0x0 => Ok(0),
        0x1 => Ok(2),
        0x2 => Ok(8),
        0x3 => Ok(32),
        _ => Err(CartridgeError::InvalidCartridgeRamSize),
    }
}

pub fn lookup_cartridge_rom_size(input: u8) -> Result<usize, CartridgeError> {
    match input {
        0x0 => Ok(32),
        0x1 => Ok(64),
        0x2 => Ok(128),
        0x3 => Ok(256),
        0x4 => Ok(512),
        0x5 => Ok(1000),
        0x6 => Ok(2000),
        0x7 => Ok(4000),
        0x8 => Ok(1100),
        0x9 => Ok(1200),
        0xA => Ok(1500),
        _ => Err(CartridgeError::InvalidCartridgeRomSize),
    }
}

#[derive(Debug)]
pub struct Cartridge {
    pub start_address: u16,
    pub cartridge_type: CartridgeType,
    pub game_title: String,
    pub rom_size: usize,
    pub ram_size: usize,
}

impl fmt::Display for Cartridge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Title: {}\nCartridge Type: {}\nStart Address: {:X}\nRom Size: {}kB\nRam Size: {}kB\n",
            self.game_title, self.cartridge_type, self.start_address, self.rom_size, self.ram_size,
        )
    }
}

impl Cartridge {
    pub fn new(buffer: &[u8]) -> Result<Self, CartridgeError> {
        let game_title = match str::from_utf8(
            &buffer[cartridge_header_address::GAME_TITLE
                ..cartridge_header_address::GAME_TITLE + GAME_TITLE_LENGTH],
        ) {
            Ok(v) => v,
            Err(_x) => return Err(CartridgeError::BadRomData),
        };

        let rom_size =
            lookup_cartridge_rom_size(buffer[cartridge_header_address::CARTRIDGE_ROM_SIZE])?;

        let ram_size =
            lookup_cartridge_ram_size(buffer[cartridge_header_address::CARTRIDGE_RAM_SIZE])?;

        Ok(Self {
            start_address: hi_lo_combine(
                buffer[cartridge_header_address::ENTRY + 1],
                buffer[cartridge_header_address::ENTRY],
            ),
            cartridge_type: lookup_cartridge_type(
                buffer[cartridge_header_address::CARTRIDGE_TYPE],
            )?,
            game_title: game_title.to_string(),
            rom_size,
            ram_size,
        })
    }

    pub fn header_info(&self) -> String {
        format!(
            "{} \n\
            Cartridge Type: {} \n\
            Ram Size: {}kB \n\
            Rom Size: {}kB \n\
            Entry Address: {:X}",
            self.game_title, self.cartridge_type, self.ram_size, self.rom_size, self.start_address
        )
    }
}

#[cfg(test)]
mod cartridge_header_test {
    use crate::spec::cartridge_header::{Cartridge, CartridgeType};

    fn get_header_fixture() -> Vec<u8> {
        vec![
            255, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 255, 0,
            0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0,
            0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 195, 229, 29, 0, 0, 0, 0, 0, 195, 172, 21, 0, 0, 0,
            0, 0, 195, 106, 33, 0, 0, 0, 0, 0, 195, 121, 31, 0, 0, 0, 0, 0, 217, 175, 224, 15, 240,
            255, 71, 203, 135, 224, 255, 240, 68, 254, 145, 32, 250, 240, 64, 230, 127, 224, 64,
            120, 224, 255, 201, 240, 64, 203, 255, 224, 64, 201, 175, 33, 0, 195, 6, 160, 34, 5,
            32, 252, 201, 62, 160, 33, 0, 195, 17, 4, 0, 6, 40, 119, 25, 5, 32, 251, 201, 234, 34,
            209, 240, 184, 245, 250, 34, 209, 205, 126, 62, 205, 177, 0, 241, 205, 126, 62, 201,
            120, 167, 40, 12, 121, 167, 40, 1, 4, 205, 193, 0, 5, 32, 250, 201, 42, 18, 19, 13, 32,
            250, 201, 240, 64, 203, 127, 194, 254, 21, 229, 98, 107, 209, 120, 245, 203, 49, 62,
            15, 161, 71, 62, 240, 161, 79, 241, 195, 157, 0, 240, 64, 203, 127, 194, 54, 22, 213,
            84, 93, 120, 245, 38, 0, 105, 41, 41, 41, 68, 77, 241, 225, 195, 212, 21, 0, 0, 0, 0,
            0, 195, 171, 1, 206, 237, 102, 102, 204, 13, 0, 11, 3, 115, 0, 131, 0, 12, 0, 13, 0, 8,
            17, 31, 136, 137, 0, 14, 220, 204, 110, 230, 221, 221, 217, 153, 187, 187, 103, 99,
            110, 14, 236, 204, 221, 220, 153, 159, 187, 185, 51, 62, 80, 79, 75, 69, 77, 79, 78,
            32, 89, 69, 76, 76, 79, 87, 0, 128, 48, 49, 3, 27, 5, 3, 1, 51, 0, 151, 4, 124, 240,
            184, 245, 120, 205, 126, 62, 42, 79, 42, 71, 42, 87, 62, 3, 61, 32, 253, 205, 153, 1,
            205, 165, 1, 205, 153, 1, 205, 165, 1, 205, 153, 1, 205, 165, 1, 205, 153, 1, 205, 165,
            1, 205, 153, 1, 205, 165, 1, 205, 153, 1, 205, 165, 1, 205, 153, 1, 205, 165, 1, 205,
            153, 1, 11, 121, 176, 32, 199, 241, 205, 126, 62, 201, 122, 230, 128, 203, 63, 203, 63,
            224, 28, 203, 34, 201, 62, 3, 61, 32, 253, 201, 254, 17, 40, 3, 175, 24, 2, 62, 1, 224,
            254, 195, 16, 29, 240, 184, 245, 62, 3, 205, 126, 62, 205, 45, 64, 241, 195, 126, 62,
            240, 184, 245, 62, 3, 205, 126, 62, 205, 0, 64, 241, 195, 126, 62, 62, 255, 234, 107,
            205, 205, 203, 14, 6, 3, 33, 124, 64, 205, 132, 62, 33, 43, 215, 203, 70, 40, 5, 62, 3,
            234, 59, 209, 33, 45, 215, 203, 110, 203, 174, 204, 208, 15, 196, 80, 7,
        ]
    }

    fn get_cartridge() -> Cartridge {
        let fixture = get_header_fixture();

        Cartridge::new(&fixture).ok().unwrap()
    }

    #[test]
    fn game_title() {
        let cartridge = get_cartridge();

        assert_eq!(cartridge.game_title, "POKEMON YELLOW ")
    }

    #[test]
    fn rom_size() {
        let cartridge = get_cartridge();

        assert_eq!(cartridge.rom_size, 1000);
    }

    #[test]
    fn ram_size() {
        let cartridge = get_cartridge();

        assert_eq!(cartridge.ram_size, 32)
    }

    #[test]
    fn cartridge_type() {
        let cartridge = get_cartridge();

        assert_eq!(cartridge.cartridge_type, CartridgeType::MBC5_RAM_BAT)
    }

    #[test]
    fn entry() {
        let cartridge = get_cartridge();

        assert_eq!(cartridge.start_address, 427)
    }
}
