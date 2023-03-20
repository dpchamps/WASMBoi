use crate::spec::mmu::Error as MmuError;
use crate::util::byte_ops::*;
use std::num::Wrapping;

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

pub mod decoded_register {
    use crate::spec::register::RegisterValue;

    pub const A: RegisterValue = "A";
    pub const B: RegisterValue = "B";
    pub const C: RegisterValue = "C";
    pub const D: RegisterValue = "D";
    pub const E: RegisterValue = "E";
    pub const H: RegisterValue = "H";
    pub const L: RegisterValue = "L";
    pub const F: RegisterValue = "F";
    pub const BC: RegisterValue = "(BC)";
    pub const DE: RegisterValue = "(DE)";
    pub const HL: RegisterValue = "(HL)";
    pub const SP: RegisterValue = "SP";
    pub const FF00C: RegisterValue = "(C)";
    pub const AF: RegisterValue = "(AF)";
    pub const HLI: RegisterValue = "(HLI)";
    pub const HLD: RegisterValue = "(HLD)";
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

pub trait TRegister<'a> {
    type ValueType: 'a;
    fn update_value_checked<F>(&'a mut self, f: F) -> Result<&'a Self::ValueType, RegisterError>
    where
        F: for<'b> FnMut(&'b Self::ValueType) -> Result<Option<Self::ValueType>, RegisterError>;
    fn update_value_wrapped<F>() -> &'a Self::ValueType
    where
        F: for<'b> FnMut(Wrapping<&'b Self::ValueType>) -> Self::ValueType;
    fn get_value(&'a self) -> &'a Self::ValueType;
    fn set_value(&mut self, value: Self::ValueType) -> ();
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

impl<'a, T: 'a + Default> TRegister<'a> for Register<T> {
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

    pub fn set_flags(&mut self, z: u8, h: u8, n: u8, c: u8) {
        self.f.value = (z << 3) | (h << 2) | (n << 1) | c
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

    pub fn lookup_register(input: u8) -> Result<RegisterValue, RegisterError> {
        match input {
            0b111 => Ok(decoded_register::A),
            0b000 => Ok(decoded_register::B),
            0b001 => Ok(decoded_register::C),
            0b010 => Ok(decoded_register::D),
            0b011 => Ok(decoded_register::E),
            0b100 => Ok(decoded_register::H),
            0b101 => Ok(decoded_register::L),
            _ => Err(RegisterError::InvalidLookupInput),
        }
    }

    pub fn lookup_ld_register(input: u8) -> Result<RegisterValue, RegisterError> {
        match input {
            0b00 => Ok(decoded_register::BC),
            0b01 => Ok(decoded_register::DE),
            0b10 => Ok(decoded_register::HL),
            0b11 => Ok(decoded_register::SP),
            _ => Err(RegisterError::InvalidLookupInput),
        }
    }

    pub fn lookup_stack_op_register(input: u8) -> Result<RegisterValue, RegisterError> {
        match input {
            0b11 => Ok(decoded_register::AF),
            _ => Registers::lookup_ld_register(input),
        }
    }
}
