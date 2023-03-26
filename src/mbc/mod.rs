use std::ops::RangeInclusive;

pub mod mbc1;
pub mod rom;

#[derive(Debug)]
pub enum MbcError {
    Read(u16),
    Write(u16, u8),
}

pub trait Mbc {
    fn map_read(&self, address: u16) -> Result<u8, MbcError>;
    fn map_write(&mut self, address: u16, data: u8) -> Result<(), MbcError>;
}
