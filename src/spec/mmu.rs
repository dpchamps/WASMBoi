#![allow(non_camel_case_types)]

use crate::mbc::rom::Rom;
use crate::mbc::{mbc1::Mbc1, Mbc, MbcError};
use crate::spec::cartridge_header::CartridgeType;
use crate::spec::hardware_registers::{HardwareRegister, HardwareRegisterError, Interrupt};
use crate::spec::memory_region::MemoryRegion;
use std::convert::TryFrom;
use std::ops::Range;

#[derive(Debug)]
pub enum Error {
    CreateError,
    ReadError,
    WriteError,
    MBCError(MbcError),
    HWError(HardwareRegisterError),
    UnusableWriteRegion,
    InvalidInterruptFlagState,
}

impl From<MbcError> for Error {
    fn from(e: MbcError) -> Self {
        Error::MBCError(e)
    }
}

impl From<HardwareRegisterError> for Error {
    fn from(e: HardwareRegisterError) -> Self {
        Error::HWError(e)
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
            CartridgeType::ROM | CartridgeType::ROM_RAM | CartridgeType::ROM_RAM_BAT => {
                MbcType::Rom
            }
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
    pub enable_interrupts: bool,
    interrupt_enable: u8,
    pub internal_ram: Box<[u8]>,
    hi_ram: Box<[u8]>,
    // TODO: hw registers implemented as a solid block of mem. Pick these off into
    //  separate datastructures as needed.
    hw_registers: HardwareRegister,
}

impl MMU {
    pub fn new(game_data: &[u8], cart_type: &CartridgeType) -> Result<MMU, Error> {
        Ok(MMU {
            mbc: Self::create_mbc_from_type(cart_type, game_data),
            enable_interrupts: false,
            interrupt_enable: 0,
            internal_ram: Box::from([0; 0xE000 - 0xC000]),
            hi_ram: Box::from([0; 0xFFFF - 0xFF80]),
            hw_registers: HardwareRegister::default(),
        })
    }

    pub fn read_byte(&self, address: u16) -> Result<u8, Error> {
        match address {
            0x8000..=0x9FFF => {
                // VIDEO RAM
                Ok(0)
            }
            0xC000..=0xFDFF => {
                // Internal work ram
                // Note 0xE000-0xFDFF is mirror ram
                let mirrored_address = if address >= 0xE000 {
                    address - (0xE000 - 0xC000)
                } else {
                    address
                };

                Ok(self.internal_ram[(mirrored_address - 0xC000) as usize])
            }
            0xFEA0..=0xFEFF => Ok(0),
            0xFF00..=0xFF7F => Ok(self.hw_registers.map_read(address)?),
            0xFF80..=0xFFFE => Ok(self.hi_ram[(address - 0xFF80) as usize]),
            0xFFFF => Ok(self.interrupt_enable),
            _ => self
                .mbc
                .map_read(address)
                .or_else(|_| panic!("Attempt to read from an unknown address {:X}", address)),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) -> Result<(), Error> {
        match address {
            0x8000..=0x9FFF => {
                // VIDEO RAM unimplemented
                Ok(())
            }
            0xC000..=0xFDFF => {
                // Internal work ram
                // Note 0xE000-0xFDFF is mirror ram
                let mirrored_address = if address >= 0xE000 {
                    address - (0xE000 - 0xC000)
                } else {
                    address
                };

                self.internal_ram[(mirrored_address - 0xC000) as usize] = value;
                Ok(())
            }
            0xFEA0..=0xFEFF => Ok(()),
            0xFF00..=0xFF7F => Ok(self.hw_registers.map_write(address, value)?),
            0xFF80..=0xFFFE => {
                self.hi_ram[(address - 0xFF80) as usize] = value;
                Ok(())
            }
            0xFFFF => {
                self.interrupt_enable = value;

                Ok(())
            }
            _ => self.mbc.map_write(address, value).or_else(|_| {
                panic!(
                    "Attempt to write to an unknown address {:X} <- {:X}",
                    address, value
                )
            }),
        }
    }

    pub fn read_word(&self, address: u16) -> Result<u16, Error> {
        let rhs = self.read_byte(address)? as u16;
        let lhs = self.read_byte(address + 1)? as u16;
        let value = (lhs << 8) | rhs;

        Ok(value)
    }

    pub fn write_word(&mut self, address: u16, value: u16) -> Result<(), Error> {
        let rhs = value & 0xFF;
        let lhs = (value & 0xFF00) >> 8;

        self.write_byte(address, rhs as u8)?;
        self.write_byte(address + 1, lhs as u8)
    }

    pub fn write_interrupt_enable_reg(&mut self, value: bool) {
        self.enable_interrupts = value;
    }

    pub fn interrupts_enabled(&self) -> Result<Option<Interrupt>, Error> {
        let interrupt_enable = self.read_byte(0xFFFF)?;
        let interrupt_flag = self.read_byte(0xFF0F)?;

        if self.enable_interrupts && (interrupt_enable & interrupt_flag) != 0 {
            return Ok(Interrupt::try_from(interrupt_enable & interrupt_flag).ok());
        }

        Ok(None)
    }

    pub fn interrupts_scheduled(&self) -> Result<bool, Error> {
        let interrupt_enable = self.read_byte(0xFFFF)?;
        let interrupt_flag = self.read_byte(0xFF0F)?;

        Ok((interrupt_enable & interrupt_flag) != 0)
    }

    pub fn set_interrupt_bit(&mut self, int: Interrupt, state: bool) -> Result<(), Error> {
        let interrupt_flag = self.read_byte(0xFF0F)?;
        let bit = int.get_position();
        let next_value = if state {
            interrupt_flag | bit
        } else {
            interrupt_flag & !bit
        };

        self.write_byte(0xFF0F, next_value)
    }

    fn create_mbc_from_type(cart_type: &CartridgeType, data: &[u8]) -> Box<dyn Mbc> {
        match MbcType::from(cart_type) {
            MbcType::Rom => Box::new(Rom::new(data)),
            MbcType::Mbc1 => Box::new(Mbc1::new(data)),
            MbcType::Mbc2 => unimplemented!("MBC2"),
            MbcType::Mbc3 => unimplemented!("MBC3"),
            MbcType::Mbc4 => unimplemented!("MBC4"),
            MbcType::Mbc5 => unimplemented!("MBC5"),
            MbcType::Mbc5Rumble => unimplemented!("MBC5Rumble"),
            MbcType::Mmm => unimplemented!("MMM"),
        }
    }

    #[cfg(debug_assertions)]
    pub fn debug_print_range(&self, range: Range<u16>) {
        let mut chunk = vec![];
        let x = range.clone();

        for address in range {
            chunk.push(format!(
                "{:X}: {:X}",
                address,
                self.read_byte(address).unwrap()
            ))
        }

        println!("Chunk between {:X?}: {:?}", x, chunk);
    }
}
