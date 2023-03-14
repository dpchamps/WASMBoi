#![allow(non_camel_case_types)]
use crate::spec::cartridge_header::{Cartridge, CartridgeType};
use crate::spec::mmu::Error::CreateError;

pub enum MBC {
    Rom,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc4,
    Mbc5,
    Mbc5Rumble,
    Mmm,
}

impl From<&CartridgeType> for MBC {
    fn from(cart_type: &CartridgeType) -> Self {
        match cart_type {
            CartridgeType::ROM | CartridgeType::ROM_RAM | CartridgeType::ROM_RAM_BAT => MBC::Rom,
            CartridgeType::MBC1 | CartridgeType::MBC1_RAM | CartridgeType::MBC1_RAM_BAT => {
                MBC::Mbc1
            }
            CartridgeType::MBC2 | CartridgeType::MBC2_BAT => MBC::Mbc2,
            CartridgeType::MMM_01 | CartridgeType::MMM_01_RAM | CartridgeType::MMM_01_RAM_BAT => {
                MBC::Mmm
            }
            CartridgeType::MBC3
            | CartridgeType::MBC3_TIMER_BAT
            | CartridgeType::MBC3_RAM_TIMER_BAT
            | CartridgeType::MBC3_RAM
            | CartridgeType::MBC3_RAM_BAT => MBC::Mbc3,
            CartridgeType::MBC4 | CartridgeType::MBC4_RAM | CartridgeType::MBC4_RAM_BAT => {
                MBC::Mbc4
            }
            CartridgeType::MBC5 | CartridgeType::MBC5_RAM | CartridgeType::MBC5_RAM_BAT => {
                MBC::Mbc5
            }
        }
    }
}

pub struct MMU {
    rom: Box<[u8]>,
    mbc: MBC,
}

#[derive(Debug)]
pub enum Error {
    CreateError,
    ReadError,
    WriteError,
}

impl MMU {
    pub fn new(game_data: &[u8], cart_type: &CartridgeType) -> Result<MMU, Error> {
        Ok(MMU {
            rom: Box::from(game_data),
            mbc: cart_type.into(),
        })
    }

    pub fn read_byte(&self, address: u16) -> Result<u8, Error> {
        self.rom.get(address as usize).map(|x|*x).ok_or(Error::ReadError)
    }

    pub fn read_word(&self, address: u16) -> Result<u16, Error> {
        unimplemented!()
    }

    pub fn write_byte(&mut self, address: u16, value: u8) -> Result<(), Error> {
        unimplemented!()
    }

    pub fn write_word(&mut self, address: u16, value: u8) -> Result<(), Error> {
        unimplemented!()
    }
}
