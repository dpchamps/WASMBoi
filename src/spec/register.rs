use crate::util::byte_ops::*;

pub enum RegisterError {
    InvalidLookupInput
}
type RegisterValue = &'static str;
type RegisterLookupResult = Result<RegisterValue, RegisterError>;

pub const A : RegisterValue = "A";
pub const B : RegisterValue = "B";
pub const C : RegisterValue = "C";
pub const D : RegisterValue = "D";
pub const E : RegisterValue = "E";
pub const H : RegisterValue = "H";
pub const L : RegisterValue = "L";
pub const F : RegisterValue = "F";
pub const BC : RegisterValue = "(BC)";
pub const DE : RegisterValue = "(DE)";
pub const HL : RegisterValue = "(HL)";
pub const SP : RegisterValue = "SP";
pub const FF00C : RegisterValue = "(C)";
pub const AF : RegisterValue = "(AF)";
pub const HLI : RegisterValue = "(HLI)";
pub const HLD : RegisterValue = "(HLD)";

pub struct Register {
    a : u8,
    b : u8,
    c : u8,
    d : u8,
    e : u8,
    h : u8,
    l : u8,
    f : u8,
    pc : u16,
    sp : u16
}

impl Register {
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
}

pub fn lookup_register(input : u8) -> RegisterLookupResult {
    match input {
        0b111 => Ok(A),
        0b000 => Ok(B),
        0b001 => Ok(C),
        0b010 => Ok(D),
        0b011 => Ok(E),
        0b100 => Ok(H),
        0b101 => Ok(L),
        _ => Err(RegisterError::InvalidLookupInput)
    }
}

pub fn lookup_ld_register(input : u8) -> RegisterLookupResult {
    match input {
        0b00 => Ok(BC),
        0b01 => Ok(DE),
        0b10 => Ok(HL),
        0b11 => Ok(SP),
        _ => Err(RegisterError::InvalidLookupInput)
    }
}

pub fn lookup_stack_op_register(input : u8) -> RegisterLookupResult{
    match input {
        0b11 => Ok(AF),
        _ => lookup_ld_register(input)
    }
}