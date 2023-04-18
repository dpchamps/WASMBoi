use crate::dasm::InstructionData;

use crate::spec::cpu::{Error, CPU};
use crate::spec::mmu::MMU;
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;

impl CPU {
    pub(crate) fn evaluate_control(
        &mut self,
        instruction_data: &InstructionData,
        _opcode_data: &[u8; 2],
        mmu: &mut MMU,
    ) -> Result<u8, Error> {
        match instruction_data.instruction {
            Instruction::CCF => {
                unimplemented!()
            }
            Instruction::SCF => {
                unimplemented!()
            }
            Instruction::NOP => Ok(1),
            Instruction::HALT => {
                self.halt = true;
                Ok(1)
            }
            Instruction::STOP => {
                unimplemented!()
            }
            Instruction::DI => {
                mmu.write_interrupt_enable_reg(false);
                Ok(1)
            }
            Instruction::EI => {
                mmu.write_interrupt_enable_reg(true);
                Ok(1)
            }
            _ => Err(unexpected_op(&instruction_data.mnemonic, &Mnemonic::PUSH)),
        }
    }
}
