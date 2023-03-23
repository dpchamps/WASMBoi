use crate::spec::mmu::Error as MmuError;
use crate::spec::register_ops::{CarryFlags, RegisterOp, RegisterOpResult};
use crate::util::byte_ops::*;
use num::traits::{WrappingAdd, WrappingSub};
use num::PrimInt;
use num_integer::Integer;
use std::fmt::{Binary, Display, Formatter, UpperHex};
use std::num::Wrapping;
use std::ops::Index;

#[derive(Debug)]
pub enum RegisterError {
    InvalidLookupInput,
    CheckedFailure,
    MmuError(MmuError),
}

impl From<MmuError> for RegisterError {
    fn from(mmu_error: MmuError) -> Self {
        RegisterError::MmuError(mmu_error)
    }
}

type RegisterValue = &'static str;

pub trait TRegister<'a> {
    type ValueType: 'a + PrimInt;

    fn update_value_checked<F>(&'a mut self, f: F) -> Result<&'a Self::ValueType, RegisterError>
    where
        F: for<'b> FnMut(&'b Self::ValueType) -> Result<Option<Self::ValueType>, RegisterError>;

    fn update_value_wrapped<F>() -> &'a Self::ValueType
    where
        F: for<'b> FnMut(Wrapping<&'b Self::ValueType>) -> Self::ValueType;

    fn get_value(&'a self) -> &'a Self::ValueType;

    fn set_value(&mut self, value: Self::ValueType);

    fn result_wrapped<F>(&self, f: F) -> Wrapping<Self::ValueType>
    where
        F: for<'b> FnMut(&'b Self::ValueType) -> Wrapping<Self::ValueType>;
}

pub trait FlagRegister {}

#[derive(Debug)]
pub struct Register<T: Default> {
    value: T,
    tag: RegisterType,
}

impl<'a, T: 'a + Default + PrimInt> TRegister<'a> for Register<T> {
    type ValueType = T;

    fn update_value_checked<F>(&'a mut self, mut f: F) -> Result<&'a Self::ValueType, RegisterError>
    where
        F: for<'b> FnMut(&'b Self::ValueType) -> Result<Option<Self::ValueType>, RegisterError>,
    {
        let result = f(&self.value)?.ok_or(RegisterError::CheckedFailure)?;
        self.value = result;

        Ok(&self.value)
    }

    fn update_value_wrapped<F>() -> &'a Self::ValueType
    where
        F: for<'b> FnMut(Wrapping<&'b Self::ValueType>) -> Self::ValueType,
    {
        todo!()
    }

    fn get_value(&'a self) -> &'a Self::ValueType {
        &self.value
    }

    fn set_value(&mut self, value: T) {
        self.value = value;
    }

    fn result_wrapped<F>(&self, mut f: F) -> Wrapping<Self::ValueType>
    where
        F: for<'b> FnMut(&'b Self::ValueType) -> Wrapping<Self::ValueType>,
    {
        f(&self.value)
    }
}

impl<T: Default> Register<T> {
    pub fn new(value: T, tag: RegisterType) -> Self {
        Self { value, tag }
    }
}

#[derive(Debug)]
pub enum RegisterType {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    F,
    PC,
    SP,
}

#[derive(Debug)]
pub enum RegisterRef<'a> {
    Byte(&'a Register<u8>),
    Flag(&'a Register<u8>),
    PC(&'a Register<u16>),
    SP(&'a Register<u16>),
}

#[derive(Debug)]
pub struct Registers {
    pub a: Register<u8>,
    pub b: Register<u8>,
    pub c: Register<u8>,
    pub d: Register<u8>,
    pub e: Register<u8>,
    pub h: Register<u8>,
    pub l: Register<u8>,
    pub f: Register<u8>,
    pub pc: Register<u16>,
    pub sp: Register<u16>,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: Register::new(0x01, RegisterType::A),
            b: Register::new(0x00, RegisterType::B),
            c: Register::new(0x13, RegisterType::C),
            d: Register::new(0x00, RegisterType::D),
            e: Register::new(0xD8, RegisterType::E),
            h: Register::new(0x01, RegisterType::H),
            l: Register::new(0x4D, RegisterType::L),
            f: Register::new(0b1011, RegisterType::F),
            pc: Register::new(0x100, RegisterType::SP),
            sp: Register::new(0xFFFE, RegisterType::SP),
        }
    }

    pub fn bc(&self) -> u16 {
        hi_lo_combine(self.b.value, self.c.value)
    }

    pub fn de(&self) -> u16 {
        hi_lo_combine(self.d.value, self.c.value)
    }

    pub fn hl(&self) -> u16 {
        hi_lo_combine(self.h.value, self.l.value)
    }

    pub fn af(&self) -> u16 {
        hi_lo_combine(self.a.value, self.f.value)
    }

    pub fn op<F, T>(&mut self, mut f: F) -> T
    where
        T: PrimInt + CarryFlags + WrappingAdd + WrappingSub,
        F: for<'b> FnMut(&'b Self) -> RegisterOpResult<T>,
    {
        let result = f(self);
        self.f.set_value(result.flags.get_value());

        result.value
    }

    pub fn op_with_effect<F, T>(&mut self, mut f: F) -> T
    where
        T: PrimInt + CarryFlags + WrappingAdd + WrappingSub,
        F: for<'b> FnMut(&'b mut Self) -> RegisterOpResult<T>,
    {
        let result = f(self);
        self.f.set_value(result.flags.get_value());

        result.value
    }

    fn at(&self, index: RegisterType) -> RegisterRef {
        match index {
            RegisterType::A => RegisterRef::Byte(&self.a),
            RegisterType::B => RegisterRef::Byte(&self.b),
            RegisterType::C => RegisterRef::Byte(&self.c),
            RegisterType::D => RegisterRef::Byte(&self.d),
            RegisterType::E => RegisterRef::Byte(&self.e),
            RegisterType::H => RegisterRef::Byte(&self.f),
            RegisterType::L => RegisterRef::Byte(&self.l),
            RegisterType::F => RegisterRef::Flag(&self.f),
            RegisterType::PC => RegisterRef::PC(&self.pc),
            RegisterType::SP => RegisterRef::SP(&self.sp),
        }
    }
}

impl Display for Registers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Registers[{} {} {} {} {} {} {} {} {}, BC: {:X}, DE: {:X}, HL: {:X}, AF: {:X}]",
            self.a,
            self.b,
            self.c,
            self.d,
            self.h,
            self.l,
            self.f,
            self.pc,
            self.sp,
            self.bc(),
            self.de(),
            self.hl(),
            self.af()
        )
    }
}

impl<T: Default + UpperHex + Binary> Display for Register<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.tag {
            RegisterType::F => write!(f, "[{}]: {:04b}", self.tag, self.value),
            _ => write!(f, "[{}]: {:X}", self.tag, self.value),
        }
    }
}

impl Display for RegisterType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RegisterType::A => "A",
            RegisterType::B => "B",
            RegisterType::C => "C",
            RegisterType::D => "D",
            RegisterType::E => "E",
            RegisterType::H => "H",
            RegisterType::L => "L",
            RegisterType::F => "F",
            RegisterType::PC => "PC",
            RegisterType::SP => "SP",
        };
        write!(f, "{}", s)
    }
}
