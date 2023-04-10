pub trait MemoryRegion {
    type Error;
    fn map_read(&self, address: u16) -> Result<u8, Self::Error>;
    fn map_write(&mut self, address: u16, data: u8) -> Result<(), Self::Error>;
}