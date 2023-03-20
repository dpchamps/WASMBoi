use std::ffi::c_void;
use std::num::Wrapping;
use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, Deref};
use std::{fmt, u16};

use crate::spec::cpu::Error::Default;
use num::traits::{WrappingAdd, WrappingSub};
use num::{cast, PrimInt as Integer};
use std::ops;

struct Flags {
    z: u8,
    n: u8,
    h: u8,
    c: u8,
}

impl From<&FlagRegister> for Flags {
    fn from(fr: &FlagRegister) -> Self {
        Flags {
            z: (fr.0 & 0b1000) >> 3,
            n: (fr.0 & 0b0100) >> 2,
            h: (fr.0 & 0b0010) >> 1,
            c: (fr.0 & 0b0001),
        }
    }
}

#[derive(Default, PartialEq, Debug)]
pub struct FlagRegister(pub u8);

impl FlagRegister {
    pub fn get_value(&self) -> u8 {
        self.0
    }

    pub fn new(z: bool, n: bool, h: bool, c: bool) -> Self {
        FlagRegister(((z as u8) << 3) | ((h as u8) << 2) | ((n as u8) << 1) | (c as u8))
    }
}

pub trait CarryFlags {
    fn half_carry_add(&self, other: &Self) -> bool;
    fn half_carry_sub(&self, other: &Self) -> bool;
    fn full_carry_add(&self, other: &Self) -> bool;
    fn full_carry_sub(&self, other: &Self) -> bool;
}

impl CarryFlags for u8 {
    fn half_carry_add(&self, other: &Self) -> bool {
        ((((*self as i8) & 0xF) + ((*other as i8) & 0xF)) & 0x10) == 0x10
    }

    fn half_carry_sub(&self, other: &Self) -> bool {
        ((((*self as i8) & 0xF) - ((*other as i8) & 0xF)) & 0x10) == 0x10
    }

    fn full_carry_add(&self, other: &Self) -> bool {
        self.checked_add(*other).map(|_| false).unwrap_or(true)
    }

    fn full_carry_sub(&self, other: &Self) -> bool {
        self.checked_sub(*other).map(|_| false).unwrap_or(true)
    }
}

impl CarryFlags for u16 {
    fn half_carry_add(&self, other: &Self) -> bool {
        (((self & 0xFFF) + (other & 0xFFF)) & 0x1000) == 0x1000
    }

    fn half_carry_sub(&self, other: &Self) -> bool {
        (((self & 0xFFF) - (other & 0xFFF)) & 0x1000) == 0x1000
    }

    fn full_carry_add(&self, other: &Self) -> bool {
        self.checked_add(*other).map(|_| false).unwrap_or(true)
    }

    fn full_carry_sub(&self, other: &Self) -> bool {
        self.checked_add(*other).map(|_| false).unwrap_or(true)
    }
}

pub trait ToPrimitive<T> {
    fn to_primitive_unchecked(&self) -> T;
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash, Debug)]
pub struct RegisterOp<T: Integer> {
    value: T,
}

pub struct RegisterOpResult<T: Integer> {
    pub value: T,
    pub flags: FlagRegister,
}

impl<T> RegisterOp<T>
where
    T: Integer + CarryFlags + WrappingAdd + WrappingSub,
{
    pub fn new(value: T) -> Self {
        RegisterOp { value }
    }

    pub fn add(&self, value: T) -> RegisterOpResult<T> {
        let result = self.value.wrapping_add(&value);
        let primitive_value = cast(result).unwrap_or(u16::MAX);

        let z = cast(result).unwrap_or(1) == 0;
        let n = false;
        let h = self.value.half_carry_add(&value);
        let c = self.value.full_carry_add(&value);
        let flags = FlagRegister::new(z, n, h, c);

        RegisterOpResult::new(result, flags)
    }

    pub fn sub(&self, value: T) -> RegisterOpResult<T> {
        let result = self.value.wrapping_sub(&value);

        let z = cast(result).unwrap_or(1) == 0;
        let n = true;
        let h = self.value.half_carry_sub(&value);
        let c = self.value.full_carry_sub(&value);
        let flags = FlagRegister::new(z, n, h, c);

        RegisterOpResult::new(result, FlagRegister::default())
    }
}

impl<T> RegisterOpResult<T>
where
    T: Integer,
{
    pub fn new(value: T, flags: FlagRegister) -> Self {
        RegisterOpResult { value, flags }
    }
}

impl PartialEq<RegisterOp<u8>> for u8 {
    fn eq(&self, other: &RegisterOp<u8>) -> bool {
        self == &other.value
    }
}

impl PartialEq<RegisterOp<u16>> for u16 {
    fn eq(&self, other: &RegisterOp<u16>) -> bool {
        self == &other.value
    }
}

impl PartialEq<u8> for RegisterOp<u8> {
    fn eq(&self, other: &u8) -> bool {
        &self.value == other
    }
}

impl PartialEq<u16> for RegisterOp<u16> {
    fn eq(&self, other: &u16) -> bool {
        &self.value == other
    }
}

impl<T> Deref for RegisterOp<T>
where
    T: Integer,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod register_ops_test {
    use crate::spec::register_ops::{CarryFlags, FlagRegister, RegisterOp};

    #[test]
    fn addition_half_carry_u8() {
        let x: u8 = 62;

        assert_eq!(true, x.half_carry_add(&35));
        assert_eq!(false, x.half_carry_add(&0))
    }

    #[test]
    fn subtraction_half_carry_u8() {
        let x: u8 = 0x3C;

        assert_eq!(true, x.half_carry_sub(&0x2F))
    }

    #[test]
    fn addition() {
        let x = RegisterOp::new(10 as u8);
        let result = x.add(10);
        assert_eq!(20, result.value);
        assert_eq!(FlagRegister::new(false, false, true, false), result.flags);
    }

    // #[test]
    // fn addition_assignment(){
    //     let mut x = RegisterOp(14 as u8);
    //
    //     x += 10;
    //
    //     assert_eq!(24, x)
    // }
}
