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

impl MemoryRegion for HardwareRegister {
    type Error = HardwareRegisterError;

    fn map_read(&self, address: u16) -> Result<u8, Self::Error> {
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