use std::convert::TryFrom;
use crate::spec::memory_region::MemoryRegion;

pub struct HardwareRegister {
    registers: [u8; 0x7F]
}

impl Default for HardwareRegister {
    fn default() -> Self {
        HardwareRegister {
            registers: [0; 0x7F]
        }
    }
}

#[derive(Debug)]
pub enum HardwareRegisterError {

}

#[derive(Debug)]
pub enum Interrupt {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    Joypad
}

impl TryFrom<u8> for Interrupt {
    type Error = ();

    /// This function returns the highest priority interrupt from the given
    /// byte.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b if (b & 0b1) >= 1 => Ok(Interrupt::VBlank),
            b if (b & 0b10) >= 1 => Ok(Interrupt::LCDStat),
            b if (b & 0b100) >= 1 => Ok(Interrupt::Timer),
            b if (b & 0b1000) >= 1 => Ok(Interrupt::Serial),
            b if (b & 0b10000) >= 1 => Ok(Interrupt::Joypad),
            _ => Err(())
        }
    }
}

impl Interrupt {
    pub fn get_isr_location(&self) -> u16 {
        match *self {
            Interrupt::VBlank => 0x40,
            Interrupt::LCDStat => 0x48,
            Interrupt::Timer => 0x50,
            Interrupt::Serial => 0x58,
            Interrupt::Joypad => 0x60
        }
    }

    pub fn get_position(&self) -> u8 {
        match *self {
            Interrupt::VBlank =>  0b1,
            Interrupt::LCDStat => 0b10,
            Interrupt::Timer =>   0b100,
            Interrupt::Serial =>  0b1000,
            Interrupt::Joypad =>  0b10000
        }
    }
}

impl MemoryRegion for HardwareRegister {
    type Error = HardwareRegisterError;

    fn map_read(&self, address: u16) -> Result<u8, Self::Error> {
        // GBDEBUG, todo: add flag here
        if address == 0xFF44 {
            return Ok(0x90);
        }
        Ok(self.registers[(address-0xFF00) as usize])
    }

    fn map_write(&mut self, address: u16, value: u8) -> Result<(), Self::Error> {
        let offset = (address-0xFF00) as usize;
        match address {
            0xFF04 => {
                self.registers[offset] = 0;
                Ok(())
            },
            _ => {
                self.registers[(address-0xFF00) as usize] = value;

                Ok(())
            }
        }
    }
}