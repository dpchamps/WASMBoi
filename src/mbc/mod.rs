use std::ops::RangeInclusive;
use crate::spec::memory_region::MemoryRegion;

pub mod mbc1;
pub mod rom;

#[derive(Debug)]
pub enum MbcError {
    Read(u16),
    Write(u16, u8),
}

pub trait Mbc: MemoryRegion<Error = MbcError> {

}
