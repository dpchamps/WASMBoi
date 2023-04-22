use std::ops::Deref;
use std::u16;

use crate::util::byte_ops::hi_lo_combine;
use num::traits::{WrappingAdd, WrappingSub};
use num::{cast, PrimInt as Integer};

#[derive(Debug, Clone)]
pub struct Flags {
    pub z: u8,
    pub n: u8,
    pub h: u8,
    pub c: u8,
}

impl From<&FlagRegister> for Flags {
    fn from(fr: &FlagRegister) -> Self {
        Flags::from(fr.0)
    }
}

impl From<u8> for Flags {
    fn from(byte: u8) -> Self {
        Flags {
            z: (byte & 0b10000000) >> 7,
            n: (byte & 0b01000000) >> 6,
            h: (byte & 0b00100000) >> 5,
            c: (byte & 0b00010000) >> 4,
        }
    }
}

impl From<Flags> for FlagRegister {
    fn from(flags: Flags) -> Self {
        FlagRegister::new(flags.z != 0, flags.n != 0, flags.h != 0, flags.c != 0)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash, Debug)]
pub struct FlagRegister(pub u8);

impl FlagRegister {
    pub fn get_value(&self) -> u8 {
        self.0
    }

    pub fn new(z: bool, n: bool, h: bool, c: bool) -> Self {
        FlagRegister(((z as u8) << 7) | ((n as u8) << 6) | ((h as u8) << 5) | (c as u8) << 4)
    }

    pub fn new_with_existing(
        z: bool,
        n: bool,
        h: bool,
        c: bool,
        other_flags: Option<FlagRegister>,
    ) -> Self {
        let mut flags =
            FlagRegister(((z as u8) << 7) | ((n as u8) << 6) | ((h as u8) << 5) | (c as u8) << 4);

        if let Some(other) = other_flags {
            flags.or_from_flags(other)
        };

        flags
    }

    pub fn update<F>(&mut self, mut f: F)
    where
        F: FnMut(Flags) -> Flags,
    {
        let flags = Flags::from(self.0);
        self.0 = FlagRegister::from(f(flags)).get_value();
    }

    pub fn or_from_flags(&mut self, other: FlagRegister) {
        let self_flags = Flags::from(&*self);
        let other_flags = Flags::from(&other);
        let next = FlagRegister::new(
            (self_flags.z) != 0,
            (self_flags.n) != 0,
            (self_flags.h | other_flags.h) != 0,
            (self_flags.c | other_flags.c) != 0,
        );

        self.0 = next.0;
    }
}

pub trait CarryFlags {
    fn half_carry_add(&self, other: &Self) -> bool;
    fn half_carry_sub(&self, other: &Self) -> bool;
    fn half_carry<F>(&self, value: &Self, f: F) -> bool
    where
        F: FnMut((Self, Self)) -> Self,
        Self: Sized;
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

    fn half_carry<F>(&self, value: &Self, mut f: F) -> bool
    where
        F: FnMut((Self, Self)) -> Self,
        Self: Sized,
    {
        f((*self & 0xf, *value & 0xF)) == 0x10
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
        (((*self as i16 & 0xFFF) - (*other as i16 & 0xFFF)) & 0x1000) == 0x1000
    }

    fn half_carry<F>(&self, value: &Self, mut f: F) -> bool
    where
        F: FnMut((Self, Self)) -> Self,
    {
        f((*self & 0xFFF, *value & 0xFFF)) == 0x1000
    }

    fn full_carry_add(&self, other: &Self) -> bool {
        self.checked_add(*other).map(|_| false).unwrap_or(true)
    }

    fn full_carry_sub(&self, other: &Self) -> bool {
        self.checked_sub(*other).map(|_| false).unwrap_or(true)
    }
}

impl CarryFlags for i16 {
    fn half_carry_add(&self, other: &Self) -> bool {
        ((((*self as i8) & 0xF) + ((*other as i8) & 0xF)) & 0x10) == 0x10
    }

    fn half_carry_sub(&self, other: &Self) -> bool {
        ((((*self as i8) & 0xF) - ((*other as i8) & 0xF)) & 0x10) == 0x10
    }

    fn half_carry<F>(&self, value: &Self, mut f: F) -> bool
    where
        F: FnMut((Self, Self)) -> Self,
        Self: Sized,
    {
        f((*self & 0xf, *value & 0xF)) == 0x10
    }

    fn full_carry_add(&self, other: &Self) -> bool {
        ((((*self) & 0xFF) + ((*other) & 0xFF)) & 0x100) == 0x100
    }

    fn full_carry_sub(&self, other: &Self) -> bool {
        ((((*self) & 0xFF) - ((*other) & 0xFF)) & 0x100) == 0x100
    }
}

pub trait ToPrimitive<T> {
    fn to_primitive_unchecked(&self) -> T;
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default, Hash, Debug)]
pub struct RegisterOp<T: Integer> {
    value: T,
    flags: Option<FlagRegister>,
}

pub struct RegisterOpResult<T: Integer> {
    pub value: T,
    pub flags: FlagRegister,
    pub mask: Option<FlagRegister>,
}

impl<T: Integer> RegisterOpResult<T> {
    pub fn set_mask(&mut self, mask: FlagRegister) {
        self.mask = Some(mask)
    }

    pub fn get_masked_value(&self, last: u8) -> u8 {
        let mask = self.mask.as_ref().map(|x| x.0).unwrap_or(0b11110000);

        (last & !mask) | (self.flags.0 & mask)
    }
}

impl<T: Integer> From<RegisterOpResult<T>> for RegisterOp<T> {
    fn from(value: RegisterOpResult<T>) -> Self {
        RegisterOp {
            value: value.value,
            flags: Some(value.flags),
        }
    }
}

impl<T> RegisterOp<T>
where
    T: Integer + CarryFlags + WrappingAdd + WrappingSub,
{
    pub fn new(value: T) -> Self {
        RegisterOp { value, flags: None }
    }

    pub fn add(&self, value: T) -> RegisterOpResult<T> {
        let result = self.value.wrapping_add(&value);

        let z = cast(result).unwrap_or(1) == 0;
        let n = false;
        let h = self.value.half_carry_add(&value);
        let c = self.value.full_carry_add(&value);
        let flags = FlagRegister::new_with_existing(z, n, h, c, self.flags);

        RegisterOpResult::new(result, flags)
    }

    pub fn sub(&self, value: T) -> RegisterOpResult<T> {
        let result = self.value.wrapping_sub(&value);

        let z = cast(result).unwrap_or(1) == 0;
        let n = true;
        let h = self.value.half_carry_sub(&value);
        let c = self.value.full_carry_sub(&value);
        let flags = FlagRegister::new_with_existing(z, n, h, c, self.flags);

        RegisterOpResult::new(result, flags)
    }

    pub fn rotate_left(&self, value: T) -> RegisterOpResult<T> {
        let c = self.value.bitand(T::from(0b10000000).unwrap()) == T::from(1).unwrap();

        let result = self.value.rotate_left(cast(value).unwrap());

        let z = false;
        let n = false;
        let h = false;

        RegisterOpResult::new(
            result,
            FlagRegister::new_with_existing(z, n, h, c, self.flags),
        )
    }

    pub fn rotate_right(&self, value: T) -> RegisterOpResult<T> {
        // value & 1 == 1
        // 0b0001 -> 0b1000
        let c = self.value.bitand(T::from(1).unwrap()) == T::from(1).unwrap();

        let result = self.value.rotate_right(cast(value).unwrap());

        let z = result == T::from(0).unwrap();
        let n = false;
        let h = false;

        RegisterOpResult::new(
            result,
            FlagRegister::new_with_existing(z, n, h, c, self.flags),
        )
    }

    pub fn or(&self, value: T) -> RegisterOpResult<T> {
        let result = self.value.bitor(value);

        let z = result == T::from(0).unwrap();
        let n = false;
        let h = false;
        let c = false;

        RegisterOpResult::new(
            result,
            FlagRegister::new_with_existing(z, n, h, c, self.flags),
        )
    }

    pub fn and(&self, value: T) -> RegisterOpResult<T> {
        let result = self.value.bitand(value);

        let z = result == T::from(0).unwrap();
        let n = false;
        let h = true;
        let c = false;

        RegisterOpResult::new(
            result,
            FlagRegister::new_with_existing(z, n, h, c, self.flags),
        )
    }

    pub fn xor(&self, value: T) -> RegisterOpResult<T> {
        let result = self.value.bitxor(value);

        let z = result == T::from(0).unwrap();
        let n = false;
        let h = false;
        let c = false;

        RegisterOpResult::new(
            result,
            FlagRegister::new_with_existing(z, n, h, c, self.flags),
        )
    }
}

impl RegisterOp<u8> {
    pub fn swap(&self) -> RegisterOpResult<u8> {
        let hi = self.value >> 4;
        let lo = self.value & 0b1111;
        let result = lo << 4 | hi;

        let z = self.value == 0;
        let n = false;
        let h = false;
        let c = false;

        RegisterOpResult::new(
            result,
            FlagRegister::new_with_existing(z, n, h, c, self.flags),
        )
    }
}

impl RegisterOp<u16> {
    pub fn swap(&self) -> RegisterOpResult<u16> {
        let hi = self.value >> 8;
        let lo = self.value & 0xF;
        let result = hi_lo_combine(lo as u8, hi as u8);

        let z = self.value == 0;
        let n = false;
        let h = false;
        let c = false;

        RegisterOpResult::new(
            result,
            FlagRegister::new_with_existing(z, n, h, c, self.flags),
        )
    }
}

impl<T> RegisterOpResult<T>
where
    T: Integer,
{
    pub fn new(value: T, flags: FlagRegister) -> Self {
        RegisterOpResult {
            value,
            flags,
            mask: None,
        }
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

        assert!(x.half_carry_add(&35));
        assert!(!x.half_carry_add(&0))
    }

    #[test]
    fn subtraction_half_carry_u8() {
        let x: u8 = 0x3C;

        assert!(x.half_carry_sub(&0x2F))
    }

    #[test]
    fn addition() {
        let x = RegisterOp::new(10_u8);
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
