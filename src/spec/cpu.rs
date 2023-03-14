use crate::dasm::{DasmError, Disassembler, InstructionData};
use crate::spec::clock::Clock;
use crate::spec::mmu::{Error as MmuError, MMU};
use crate::spec::register::Register;
use std::convert::TryFrom;
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::*;

pub trait CPU {
    type E;
    fn tick(&mut self, mmu: &mut MMU) -> Result<(), Self::E>;
}

pub struct CPUImpl {
    registers: Register,
}

#[derive(Debug)]
pub enum Error {
    Default(String),
    InitializationError,
    UnsupportedOpcode(Instruction),
    MmuReadError(MmuError),
    DecodeError(DasmError),
}

impl CPU for CPUImpl {
    type E = Error;

    fn tick(&mut self, mmu: &mut MMU) -> Result<(), Error> {
        let buf: Vec<u8> = Vec::new();
        let opcode = self.fetch(mmu)?;
        let data = [
            mmu.read_byte(self.registers.pc)
                .map_err(|x| Error::MmuReadError(x))?,
            mmu.read_byte(self.registers.pc+1)
                .map_err(|x| Error::MmuReadError(x))?,
        ];

        let timing_info = self.execute(&opcode, &data)?;

        Ok(())
    }
}

impl CPUImpl {
    fn increment_pc(&mut self) -> u16 {
        let next = self.registers.pc;
        self.registers.pc += 1;
        // TODO intentional wrap;
        return next;
    }

    fn fetch(&mut self, mmu: &MMU) -> Result<InstructionData, Error> {
        let pc = self.increment_pc();
        let op = {
            let op = mmu.read_byte(pc).map_err(|x| Error::MmuReadError(x))?;

            match op {
                0xCB => {
                    self.increment_pc();
                    mmu.read_byte(pc).map_err(|x| Error::MmuReadError(x))?
                }
                _ => op,
            }
        };

        InstructionData::try_from(op).map_err(|x| Error::DecodeError(x))
    }

    fn execute(&mut self, instruction_data: &InstructionData, opcode_data: &[u8; 2]) -> Result<Clock, Error> {
        println!("{}", instruction_data);
        match instruction_data.mnemonic {
            Mnemonic::LD => self.evaluate_ld(instruction_data, opcode_data),
            _ => Err(Error::UnsupportedOpcode(instruction_data.instruction.clone()))
        }
    }

    pub fn new() -> Result<CPUImpl, Error> {
        Ok(CPUImpl {
            registers: Register::new(),
        })
    }
}
