use crate::spec::cpu::Error;
use crate::spec::mmu::Error as MmuError;
use crate::spec::register_ops::{CarryFlags, FlagRegister, Flags, RegisterOp, RegisterOpResult};
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

pub trait TRegister<'a> {
    type ValueType: 'a + PrimInt;

    fn update_value_checked<F>(&'a mut self, f: F) -> Result<(), RegisterError>
    where
        F: for<'b> FnMut(&'b Self::ValueType) -> Result<Option<Self::ValueType>, RegisterError>;

    fn update_value_wrapped<F>(&'a mut self, f: F)
    where
        F: FnMut(Wrapping<Self::ValueType>) -> Wrapping<Self::ValueType>;

    fn get_value(&'a self) -> &'a Self::ValueType;

    fn set_value(&mut self, value: Self::ValueType);
}


#[derive(Debug)]
pub struct Register<T: Default> {
    value: T,
    tag: RegisterType,
}

impl<'a, T: 'a + Default + PrimInt> TRegister<'a> for Register<T> {
    type ValueType = T;

    fn update_value_checked<F>(&'a mut self, mut f: F) -> Result<(), RegisterError>
    where
        F: for<'b> FnMut(&'b Self::ValueType) -> Result<Option<Self::ValueType>, RegisterError>,
    {
        let result = f(&self.value)?.ok_or(RegisterError::CheckedFailure)?;
        self.value = result;

        Ok(())
    }

    fn update_value_wrapped<F>(&'a mut self, mut f: F)
    where
        F: FnMut(Wrapping<Self::ValueType>) -> Wrapping<Self::ValueType>,
    {
        let result = f(Wrapping(self.value)).0;
        self.value = result;
    }

    fn get_value(&'a self) -> &'a Self::ValueType {
        &self.value
    }

    fn set_value(&mut self, value: T) {
        self.value = value;
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
pub enum RegisterRefMut<'a> {
    Byte(&'a mut Register<u8>),
    Flag(&'a mut Register<u8>),
    PC(&'a mut Register<u16>),
    SP(&'a mut Register<u16>),
}

impl<'a> RegisterRefMut<'a> {
    pub fn set_eight_bit_val(&mut self, value: u8) -> Result<(), RegisterError> {
        match self {
            RegisterRefMut::Byte(byte_reg) => {
                byte_reg.set_value(value);
                Ok(())
            },
            RegisterRefMut::Flag(flag_reg) => {
                flag_reg.set_value(value);
                Ok(())
            },
            _ => Err(RegisterError::CheckedFailure)
        }
    }

    pub fn set_sixtn_bit_val(&mut self, value: u16) -> Result<(), RegisterError> {
        match self {
            RegisterRefMut::SP(stack_pointer) => {
                stack_pointer.set_value(value);
                Ok(())
            },
            RegisterRefMut::PC(pc) => {
                pc.set_value(value);
                Ok(())
            }
            _ => Err(RegisterError::CheckedFailure)
        }
    }

    pub fn get_eight_bit_val(&self) -> Result<u8, RegisterError> {
        match self {
            RegisterRefMut::Byte(byte_reg) => {
                Ok(*byte_reg.get_value())
            },
            RegisterRefMut::Flag(flag_reg) => {
                Ok(*flag_reg.get_value())
            },
            _ => Err(RegisterError::CheckedFailure)
        }
    }

    pub fn get_sixtn_bit_val(&self) -> Result<u16, RegisterError> {
        match self {
            RegisterRefMut::SP(stack_pointer) => {
                Ok(*stack_pointer.get_value())
            },
            RegisterRefMut::PC(pc) => {
                Ok(*pc.get_value())
            }
            _ => Err(RegisterError::CheckedFailure)
        }
    }
}

#[derive(Debug)]
pub enum RegisterPair<'a>{
    EightBit(&'a mut Register<u8>, &'a mut Register<u8>),
    SixteenBit(&'a mut Register<u16>)
}

impl<'a> RegisterPair<'a> {
    pub fn set_value(&mut self, lhs: u8, rhs: u8){
        match self {
            RegisterPair::EightBit(lhs_reg, rhs_reg) => {
                lhs_reg.set_value(lhs);
                rhs_reg.set_value(rhs)
            },
            RegisterPair::SixteenBit(reg) => {
                reg.set_value(hi_lo_combine(lhs, rhs))
            }
        }
    }

    pub fn set_value_16(&mut self, value: u16){
        let lhs = ((value & 0xFF00) >> 8) as u8;
        let rhs = (value & 0x00FF) as u8;

        self.set_value(lhs, rhs)
    }

    pub fn get_value(&self) -> u16 {
        match self {
            RegisterPair::EightBit(lhs, rhs) => hi_lo_combine(lhs.value, rhs.value),
            RegisterPair::SixteenBit(reg) => reg.value
        }
    }
}

/// TODO: Implement Display Trait for Registers
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
            pc: Register::new(0x100, RegisterType::PC),
            sp: Register::new(0xFFFE, RegisterType::SP),
        }
    }

    pub fn flag_register(&self) -> Flags {
        Flags::from(&FlagRegister(self.f.value))
    }

    pub fn bc(&self) -> u16 {
        hi_lo_combine(self.b.value, self.c.value)
    }

    pub fn de(&self) -> u16 {
        hi_lo_combine(self.d.value, self.e.value)
    }

    pub fn hl(&self) -> u16 {
        hi_lo_combine(self.h.value, self.l.value)
    }

    pub fn af(&self) -> u16 {
        hi_lo_combine(self.a.value, self.f.value)
    }

    pub fn bc_mut(&mut self) -> RegisterPair {
        RegisterPair::EightBit(&mut self.b, &mut self.c)
    }

    pub fn de_mut(&mut self) -> RegisterPair {
        RegisterPair::EightBit(&mut self.d, &mut self.e)
    }

    pub fn hl_mut(&mut self) -> RegisterPair {
        RegisterPair::EightBit(&mut self.h, &mut self.l)
    }

    pub fn af_mut(&mut self) -> RegisterPair {
        RegisterPair::EightBit(&mut self.a, &mut self.f)
    }

    pub fn update<F>(&mut self, mut f: F)
    where
        F: for<'b> FnMut(&'b mut Self),
    {
        f(self)
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

    pub fn op_with_effect<F, T>(&mut self, mut f: F) -> Result<T, RegisterError>
    where
        T: PrimInt + CarryFlags + WrappingAdd + WrappingSub,
        F: for<'b> FnMut(&'b mut Self) -> Result<RegisterOpResult<T>, RegisterError>,
    {
        let result = f(self)?;
        self.f.set_value(result.flags.get_value());

        Ok(result.value)
    }

    pub fn reg_from_byte(&mut self, value: u8) -> Result<RegisterRefMut, RegisterError> {
        match value {
            0b111 => Ok(RegisterRefMut::Byte(&mut self.a)),
            0b000 => Ok(RegisterRefMut::Byte(&mut self.b)),
            0b001 => Ok(RegisterRefMut::Byte(&mut self.c)),
            0b010 => Ok(RegisterRefMut::Byte(&mut self.d)),
            0b011 => Ok(RegisterRefMut::Byte(&mut self.e)),
            0b100 => Ok(RegisterRefMut::Byte(&mut self.h)),
            0b101 => Ok(RegisterRefMut::Byte(&mut self.l)),
            _ => Err(RegisterError::InvalidLookupInput),
        }
    }

    pub fn reg_pair_from_dd(&mut self, value: u8) -> Result<RegisterPair, RegisterError> {
        match value {
            0b00 => {
                // BC
                Ok(RegisterPair::EightBit(&mut self.b, &mut self.c))
            }
            0b01 => {
                // DE
                Ok(RegisterPair::EightBit(&mut self.d, &mut self.e))
            }
            0b10 => {
                // HL
                Ok(RegisterPair::EightBit(&mut self.h, &mut self.l))

            }
            0b11 => {
                Ok(RegisterPair::SixteenBit(&mut self.sp))
            }
            _ => Err(RegisterError::InvalidLookupInput)
        }
    }

    pub fn reg_pair_from_qq(&mut self, value: u8) -> Result<RegisterPair, RegisterError> {
        match value {
            // AF
            0b11 => Ok(RegisterPair::EightBit(&mut self.a, &mut self.f)),
            _ => self.reg_pair_from_dd(value)
        }
    }

    pub fn jump_condition(&self, cc: u8) -> Result<bool, RegisterError> {
        let flags = self.flag_register();
        match cc {
            // NZ
            0b00 => Ok(flags.z == 0),
            // Z
            0b01 => Ok(flags.z == 1),
            // NC
            0b10 => Ok(flags.c == 0),
            // C
            0b11 => Ok(flags.c == 1),
            _ => Err(RegisterError::InvalidLookupInput)
        }
    }

    // fn at(&self, index: RegisterType) -> RegisterRef {
    //     match index {
    //         RegisterType::A => RegisterRef::Byte(&self.a),
    //         RegisterType::B => RegisterRef::Byte(&self.b),
    //         RegisterType::C => RegisterRef::Byte(&self.c),
    //         RegisterType::D => RegisterRef::Byte(&self.d),
    //         RegisterType::E => RegisterRef::Byte(&self.e),
    //         RegisterType::H => RegisterRef::Byte(&self.f),
    //         RegisterType::L => RegisterRef::Byte(&self.l),
    //         RegisterType::F => RegisterRef::Flag(&self.f),
    //         RegisterType::PC => RegisterRef::PC(&self.pc),
    //         RegisterType::SP => RegisterRef::SP(&self.sp),
    //     }
    // }
}

impl Display for Registers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Registers[{} {} {} {} {} {} {} {} {} {}, BC: {:X}, DE: {:X}, HL: {:X}, AF: {:X}]",
            self.a,
            self.b,
            self.c,
            self.d,
            self.e,
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
