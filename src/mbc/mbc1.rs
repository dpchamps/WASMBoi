use crate::mbc::{Mbc, MbcError};
use std::num::Wrapping;

#[derive(Default)]
pub struct Mbc1 {
    rom: Box<[u8]>,
    ram: Box<[u8]>,

    ram_enable: bool,
    rom_bank: u8,
    rom_bank_offset: u16,
    ram_bank: u8,
    ram_bank_offset: u16,
    bank_mode: bool,
}

impl Mbc1 {
    pub fn new(data: &[u8]) -> Self {
        Self {
            rom: Box::from(data),
            ram: Box::from([0; 0x7FFF]),
            ram_enable: false,
            rom_bank: 0,
            rom_bank_offset: 0,
            ram_bank: 0,
            ram_bank_offset: 0,
            bank_mode: false,
        }
    }
}

impl Mbc for Mbc1 {
    fn map_read(&self, address: u16) -> Result<u8, MbcError> {
        match address {
            0..=0x3FFF => Ok(self.rom[address as usize]),
            0x4000..=0x7FFF => {
                // println!("\t\tReading from ({:X}){:X}", address, (self.rom_bank_offset));

                Ok(self.rom[(self.rom_bank_offset + (address)) as usize])
            }
            0xA000..=0xBFFF => {
                let address = if self.bank_mode {
                    address + (self.ram_bank_offset)
                } else {
                    address
                };

                Ok(self.ram[(address) as usize])
            }
            _ => Err(MbcError::Read(address)),
        }
    }

    fn map_write(&mut self, address: u16, data: u8) -> Result<(), MbcError> {
        // println!("\t\t[{:X}+{:X}]<-{:X}", address, self.rom_bank_offset, data);
        match address {
            0..=0x1FFF => {
                self.ram_enable = (data & 0xF) == 0xA;
                Ok(())
            }
            0x2000..=0x3FFF => {
                self.rom_bank = (self.rom_bank & 0x60) + ((data & 0x1f) + 1);
                self.rom_bank_offset = (Wrapping(self.rom_bank as u16) * Wrapping(0x4000)).0;
                // NOTE: Pan docs call out a caveat for mbc1m banking read more here: https://gbdev.io/pandocs/MBC1.html#MBC1M_banking

                Ok(())
            }
            0x4000..=0x5FFF => {
                if self.bank_mode {
                    // RAM Banking Mode
                    self.ram_bank = data & 0b11;
                    self.rom_bank_offset = (self.ram_bank as u16) & 0x2000;
                } else {
                    // Simple Banking Mode
                    self.rom_bank = ((data & 0x3) << 5) + (self.rom_bank & 0x1f);
                    println!("Setting bank offset with bank: {:X}", self.rom_bank);
                    self.rom_bank_offset = (Wrapping(self.rom_bank as u16) * Wrapping(0x4000)).0;
                }

                Ok(())
            }
            0x6000..=0x7FFF => {
                self.bank_mode = (data & 1) == 1;
                Ok(())
            }
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    self.ram[(self.ram_bank_offset + (address & 0x1FFF)) as usize] = data;
                }

                Ok(())
            }
            _ => Err(MbcError::Write(address, data)),
        }
    }
}
