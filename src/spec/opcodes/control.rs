use crate::dasm::InstructionData;
use crate::spec::clock::Clock;
use crate::spec::cpu::{Error, CPU};
use crate::spec::mmu::MMU;
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;

impl CPU {
    pub(crate) fn evaluate_control(
        &mut self,
        instruction_data: &InstructionData,
        opcode_data: &[u8; 2],
        mmu: &mut MMU,
    ) -> Result<u8, Error> {
        match instruction_data.instruction {
            Instruction::CCF => {
                unimplemented!()
            }
            Instruction::SCF => {
                unimplemented!()
            }
            Instruction::NOP => {
                unimplemented!()
            }
            Instruction::HALT => {
                unimplemented!()
            }
            Instruction::STOP => {
                unimplemented!()
            }
            Instruction::DI => {
                unimplemented!()
            }
            Instruction::EI => {
                unimplemented!()
            }
            _ => Err(unexpected_op(&instruction_data.mnemonic, &Mnemonic::PUSH)),
        }
    }
}
