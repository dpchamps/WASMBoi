use crate::util::byte_ops::*;

pub enum RegisterError {
    InvalidLookupInput,
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

pub struct Register {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    f: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Register {
    pub fn new() -> Register {
        Register {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            f: 0,
            pc: 0,
            sp: 0,
        }
    }

    pub fn bc(&self) -> u16 {
        hi_lo_combine(self.b, self.c)
    }

    pub fn de(&self) -> u16 {
        hi_lo_combine(self.d, self.c)
    }

    pub fn hl(&self) -> u16 {
        hi_lo_combine(self.h, self.l)
    }

    pub fn af(&self) -> u16 {
        hi_lo_combine(self.a, self.f)
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
            _ => Register::lookup_ld_register(input),
        }
    }
}
