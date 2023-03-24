use crate::mbc::{Mbc, MbcError};

#[derive(Default)]
pub struct Rom {
    rom: Box<[u8]>,
    ram: Box<[u8]>
}

impl Rom {
    pub fn new(data: &[u8]) -> Self {
        Self {
            rom: Box::from(data),
            ram: Box::from([0; 8191])
        }
    }

}

impl Mbc for Rom {
    fn map_read(&self, address: u16) -> Result<u8, MbcError> {
        match address {
            0x0000..=0x7FFF => {
                Ok(self.rom[address as usize])
            },
            0xA000..=0xBFFF => {
                Ok(self.ram[(address - 0xA000) as usize])
            },
            _ => Err(MbcError::Read)
        }
    }

    fn map_write(&mut self, address: u16, value: u8) -> Result<(), MbcError> {
        match address {
            0x0000..=0x7FFF => Ok(()),
            0xA000..=0xBFFF => {
                self.ram[(address - 0xA000) as usize] = value;
                Ok(())
            }
            _ => Err(MbcError::Write)
        }
    }
}