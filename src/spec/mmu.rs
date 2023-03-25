#![allow(non_camel_case_types)]
use crate::spec::cartridge_header::{Cartridge, CartridgeType};
use crate::spec::mmu::Error::CreateError;
use crate::mbc::{ Mbc, mbc1::Mbc1, MbcError};
use crate::mbc::rom::Rom;

#[derive(Debug)]
pub enum Error {
    CreateError,
    ReadError,
    WriteError,
    MBCError(MbcError),
    UnusableWriteRegion
}

impl From<MbcError> for Error {
    fn from(e: MbcError) -> Self {
        Error::MBCError(e)
    }
}

pub enum MbcType {
    Rom,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc4,
    Mbc5,
    Mbc5Rumble,
    Mmm,
}

impl From<&CartridgeType> for MbcType {
    fn from(cart_type: &CartridgeType) -> Self {
        match cart_type {
            CartridgeType::ROM | CartridgeType::ROM_RAM | CartridgeType::ROM_RAM_BAT => MbcType::Rom,
            CartridgeType::MBC1 | CartridgeType::MBC1_RAM | CartridgeType::MBC1_RAM_BAT => {
                MbcType::Mbc1
            }
            CartridgeType::MBC2 | CartridgeType::MBC2_BAT => MbcType::Mbc2,
            CartridgeType::MMM_01 | CartridgeType::MMM_01_RAM | CartridgeType::MMM_01_RAM_BAT => {
                MbcType::Mmm
            }
            CartridgeType::MBC3
            | CartridgeType::MBC3_TIMER_BAT
            | CartridgeType::MBC3_RAM_TIMER_BAT
            | CartridgeType::MBC3_RAM
            | CartridgeType::MBC3_RAM_BAT => MbcType::Mbc3,
            CartridgeType::MBC4 | CartridgeType::MBC4_RAM | CartridgeType::MBC4_RAM_BAT => {
                MbcType::Mbc4
            }
            CartridgeType::MBC5 | CartridgeType::MBC5_RAM | CartridgeType::MBC5_RAM_BAT => {
                MbcType::Mbc5
            }
        }
    }
}


pub struct MMU {
    mbc: Box<dyn Mbc>,
    enable_interrupts: u8,
}

impl MMU {
    pub fn new(game_data: &[u8], cart_type: &CartridgeType) -> Result<MMU, Error> {
        Ok(MMU {
            mbc: Self::create_mbc_from_type(cart_type, game_data),
            enable_interrupts: 0
        })
    }

    pub fn read_byte(&self, address: u16) -> Result<u8, Error> {
        match address {
            0x8000..=0x9FFF => {
                // VIDEO RAM
                unimplemented!("video ram")
            },
            0xC000..=0xFDFF => {
                // Internal work ram
                // Note 0xE000-0xFDFF is mirror ram
                unimplemented!("internal work ram")
            },
            0xFEA0..=0xFEFF => {
                Err(Error::UnusableWriteRegion)
            },
            0xFF00..=0xFF7F => {
                unimplemented!("io reg")
            },
            0xFF80..=0xFFFE => {
                unimplemented!("hiram")
            },
            0xFFFF => {
                Ok(self.enable_interrupts)
            },
            _ => self.mbc.map_read(address).map_err(Error::MBCError)
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) -> Result<(), Error> {
        match address {
            0x8000..=0x9FFF => {
                // VIDEO RAM
                unimplemented!("video ram")
            },
            0xC000..=0xFDFF => {
                // Internal work ram
                // Note 0xE000-0xFDFF is mirror ram
                unimplemented!("internal work ram")
            },
            0xFEA0..=0xFEFF => {
                Err(Error::UnusableWriteRegion)
            },
            0xFF00..=0xFF7F => {
                unimplemented!("io reg")
            },
            0xFF80..=0xFFFE => {
                unimplemented!("hiram")
            },
            0xFFFF => {
                self.enable_interrupts = value;
                Ok(())
            },
            _ => {
                self.mbc.map_write(address, value).map_err(Error::MBCError)
            }
        }
    }

    pub fn read_word(&self, address: u16) -> Result<u16, Error> {
        let rhs = self.read_byte(address)? as u16;
        let lhs = self.read_byte(address + 1)? as u16;

        Ok((lhs << 0xFF) | rhs)
    }

    pub fn write_word(&mut self, address: u16, value: u16) -> Result<(), Error> {
        let rhs = value & 0xFF;
        let lhs = (value & 0xFF00) >> 8;

        self.write_byte(address, rhs as u8)?;
        self.write_byte(address + 1, lhs as u8)
    }

    pub fn write_interrupt_enable_reg(&mut self, value: bool) -> Result<(), Error> {
        self.write_byte(0xFFFF, value as u8)
    }

    fn create_mbc_from_type(cart_type: &CartridgeType, data: &[u8]) -> Box<dyn Mbc> {
        match MbcType::from(cart_type) {
            MbcType::Rom => Box::new(Rom::new(data)),
            MbcType::Mbc1 => Box::new(Mbc1::new(data)),
            MbcType::Mbc2 => unimplemented!(),
            MbcType::Mbc3 => unimplemented!(),
            MbcType::Mbc4 => unimplemented!(),
            MbcType::Mbc5 => unimplemented!(),
            MbcType::Mbc5Rumble => unimplemented!(),
            MbcType::Mmm => unimplemented!(),
        }
    }
}
