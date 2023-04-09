use crate::dasm::{DasmError, Disassembler, InstructionData};
use crate::spec::clock::Clock;
use crate::spec::mmu::{Error as MmuError, MMU};
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::*;
use crate::spec::register::{RegisterError, Registers, TRegister};
use std::convert::TryFrom;
use std::num::Wrapping;

pub trait TCPU {
    type E;
    fn tick(&mut self, mmu: &mut MMU) -> Result<u8, Self::E>;
}

pub trait TStackable {
    fn push_stack_byte(&mut self, value: u8, mmu: &mut MMU) -> Result<(), Error>;
    fn push_stack_word(&mut self, value: u16, mmu: &mut MMU) -> Result<(), Error>;
    fn pop_stack_byte(&mut self, mmu: &mut MMU) -> Result<u8, Error>;
    fn pop_stack_word(&mut self, mmu: &mut MMU) -> Result<u16, Error>;
}

pub struct CPU {
    pub(crate) registers: Registers,
}

#[derive(Debug)]
pub enum Error {
    Default(String),
    InitializationError,
    UnexpectedOpcode(String),
    UnsupportedOpcode(Instruction),
    MmuError(MmuError),
    DecodeError(DasmError),
    RegisterError(RegisterError),
    UnexpectedOpcodeState(InstructionData, u16),
}

impl From<RegisterError> for Error {
    fn from(reg_error: RegisterError) -> Self {
        Error::RegisterError(reg_error)
    }
}

impl From<MmuError> for Error {
    fn from(mmu_error: MmuError) -> Self {
        Error::MmuError(mmu_error)
    }
}

impl TCPU for CPU {
    type E = Error;

    fn tick(&mut self, mmu: &mut MMU) -> Result<u8, Error> {
        let last_pc = *self.registers.pc.get_value();
        let opcode = self.fetch(mmu)?;
        let data = [
            mmu.read_byte(*self.registers.pc.get_value())
                .map_err(Error::MmuError)?,
            mmu.read_byte((Wrapping(*self.registers.pc.get_value()) + Wrapping(1)).0)
                .map_err(Error::MmuError)?,
        ];
        // println!(
        //     "[PC: {:#X}] Op: {}, Dat: [{:X?}]",
        //     last_pc,
        //     opcode,
        //     data
        // );
        self.registers.pc.set_value(*self.registers.pc.get_value()+opcode.size as u16);
        let cycles = self.execute(&opcode, &data, mmu)?;
        // println!("\t{}", self.registers);
        // if *self.registers.pc.get_value() == 0xCBB0 {
        //     panic!()
        // }


        Ok(cycles)
    }
}

impl TStackable for CPU {
    fn push_stack_byte(&mut self, value: u8, mmu: &mut MMU) -> Result<(), Error> {
        self.registers
            .sp
            .update_value_checked(|sp| {
                mmu.write_byte(*sp, value)?;
                Ok(sp.checked_sub(1))
            })
            .map_err(Error::RegisterError)
    }

    fn push_stack_word(&mut self, value: u16, mmu: &mut MMU) -> Result<(), Error> {
        self.registers
            .sp
            .update_value_checked(|sp| {
                mmu.write_word(*sp - 1, value)?;
                Ok(sp.checked_sub(2))
            })
            .map_err(Error::RegisterError)
    }

    fn pop_stack_byte(&mut self, mmu: &mut MMU) -> Result<u8, Error> {
        unimplemented!()
    }

    fn pop_stack_word(&mut self, mmu: &mut MMU) -> Result<u16, Error> {
        let stack_val = mmu.read_word(*self.registers.sp.get_value() + 1)?;
        self.registers
            .sp
            .update_value_checked(|sp| Ok(sp.checked_add(2)))?;

        Ok(stack_val)
    }
}

impl CPU {
    fn increment_pc(&mut self) -> Result<u16, Error> {
        let next = *self.registers.pc.get_value();
        self.registers
            .pc
            .update_value_checked(|pc| Ok(pc.checked_add(1)))
            .map_err(Error::RegisterError)?;

        Ok(next)
    }

    fn fetch(&mut self, mmu: &MMU) -> Result<InstructionData, Error> {
        let pc = self.increment_pc()?;
        let op = mmu.read_byte(pc).map_err(Error::MmuError)?;
        let cb_byte = match op {
            0xCB => Some(mmu.read_byte(pc+1).map_err(Error::MmuError)?),
            _ => None
        };

        InstructionData::try_from((op, cb_byte)).map_err(Error::DecodeError)
    }

    fn execute(
        &mut self,
        instruction_data: &InstructionData,
        opcode_data: &[u8; 2],
        mmu: &mut MMU,
    ) -> Result<u8, Error> {
        let result = match instruction_data.mnemonic {
            Mnemonic::LD | Mnemonic::LDHL => self.evaluate_ld(instruction_data, opcode_data, mmu),
            Mnemonic::PUSH | Mnemonic::POP => {
                self.evaluate_stack_op(instruction_data, opcode_data, mmu)
            }
            Mnemonic::ADD
            | Mnemonic::ADC
            | Mnemonic::SUB
            | Mnemonic::SBC
            | Mnemonic::AND
            | Mnemonic::XOR
            | Mnemonic::OR
            | Mnemonic::CP
            | Mnemonic::INC
            | Mnemonic::DEC
            | Mnemonic::DAA
            | Mnemonic::CPL => self.evaluate_alu(instruction_data, opcode_data, mmu),
            Mnemonic::RLCA
            | Mnemonic::RLA
            | Mnemonic::RRCA
            | Mnemonic::RRA
            | Mnemonic::RLC
            | Mnemonic::RL
            | Mnemonic::RRC
            | Mnemonic::RR
            | Mnemonic::SLA
            | Mnemonic::SWAP
            | Mnemonic::SRA
            | Mnemonic::SET
            | Mnemonic::SRL => self.evaluate_bitwise(instruction_data, opcode_data, mmu),
            Mnemonic::CCF
            | Mnemonic::SCF
            | Mnemonic::NOP
            | Mnemonic::HALT
            | Mnemonic::STOP
            | Mnemonic::DI
            | Mnemonic::EI => self.evaluate_control(instruction_data, opcode_data, mmu),
            Mnemonic::JP
            | Mnemonic::JR
            | Mnemonic::CALL
            | Mnemonic::RET
            | Mnemonic::RETI
            | Mnemonic::RST
            | Mnemonic::DB
            | Mnemonic::DW => self.evaluate_branch(instruction_data, opcode_data, mmu),
            Mnemonic::UNIMPLEMENTED => Ok(0),
            _ => Err(Error::UnsupportedOpcode(
                instruction_data.instruction.clone(),
            )),
        }?;
        Ok(result)
    }

    pub fn new() -> Result<CPU, Error> {
        Ok(CPU {
            registers: Registers::new(),
        })
    }
}
