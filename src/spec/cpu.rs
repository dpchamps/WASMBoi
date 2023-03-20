use crate::dasm::{DasmError, Disassembler, InstructionData};
use crate::spec::clock::Clock;
use crate::spec::mmu::{Error as MmuError, MMU};
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::*;
use crate::spec::register::{RegisterError, Registers, TRegister};
use std::convert::TryFrom;

pub trait TCPU {
    type E;
    fn tick(&mut self, mmu: &mut MMU) -> Result<u8, Self::E>;
}

pub trait TStackable {
    fn push_stack_byte(&mut self, value: u8, mmu: &mut MMU) -> Result<(), Error>;
    fn push_stack_word(&mut self, value: u16, mmu: &mut MMU) -> Result<(), Error>;
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
    MmuReadError(MmuError),
    DecodeError(DasmError),
    RegisterError(RegisterError),
}

impl From<RegisterError> for Error {
    fn from(reg_error: RegisterError) -> Self {
        Error::RegisterError(reg_error)
    }
}

impl TCPU for CPU {
    type E = Error;

    fn tick(&mut self, mmu: &mut MMU) -> Result<u8, Error> {
        let buf: Vec<u8> = Vec::new();
        let opcode = self.fetch(mmu)?;
        let data = [
            mmu.read_byte(*self.registers.pc.get_value())
                .map_err(|x| Error::MmuReadError(x))?,
            mmu.read_byte(*self.registers.pc.get_value() + 1)
                .map_err(|x| Error::MmuReadError(x))?,
        ];

        let cycles = self.execute(&opcode, &data, mmu)?;

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
            .map(|_| ())
            .map_err(Error::RegisterError)
    }

    fn push_stack_word(&mut self, value: u16, mmu: &mut MMU) -> Result<(), Error> {
        self.registers
            .sp
            .update_value_checked(|sp| {
                mmu.write_word(*sp, value)?;
                Ok(sp.checked_sub(2))
            })
            .map(|_| ())
            .map_err(Error::RegisterError)
    }
}

impl CPU {
    fn increment_pc(&mut self) -> Result<u16, Error> {
        let next = self.registers.pc.get_value().clone();
        self.registers
            .pc
            .update_value_checked(|pc| Ok(pc.checked_add(1)))
            .map_err(Error::RegisterError)?;

        Ok(next)
    }

    fn fetch(&mut self, mmu: &MMU) -> Result<InstructionData, Error> {
        let pc = self.increment_pc()?;
        let op = {
            let op = mmu.read_byte(pc).map_err(|x| Error::MmuReadError(x))?;

            match op {
                0xCB => {
                    self.increment_pc()?;
                    mmu.read_byte(pc).map_err(|x| Error::MmuReadError(x))?
                }
                _ => op,
            }
        };

        InstructionData::try_from(op).map_err(|x| Error::DecodeError(x))
    }

    fn execute(
        &mut self,
        instruction_data: &InstructionData,
        opcode_data: &[u8; 2],
        mmu: &mut MMU,
    ) -> Result<u8, Error> {
        println!(
            "PC: {:#X}, Opcode: {}, Data: [{:?}]",
            self.registers.pc.get_value(),
            instruction_data,
            opcode_data
        );
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
        println!("\t{:?}", self.registers);
        Ok(result)
    }

    pub fn new() -> Result<CPU, Error> {
        Ok(CPU {
            registers: Registers::new(),
        })
    }
}
