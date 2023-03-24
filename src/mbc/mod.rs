use std::ops::{RangeInclusive};

pub mod rom;
pub mod mbc1;

#[derive(Debug)]
pub enum MbcError {
    Read,
    Write
}

pub trait Mbc {
    fn map_read(&self, address: u16) -> Result<u8, MbcError>;
    fn map_write(&mut self, address: u16, data: u8) -> Result<(), MbcError>;
}