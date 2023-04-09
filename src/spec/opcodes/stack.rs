use crate::dasm::InstructionData;
use crate::spec::clock::Clock;
use crate::spec::cpu::{Error, CPU, TStackable};
use crate::spec::mmu::MMU;
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;

impl CPU {
    pub(crate) fn evaluate_stack_op(
        &mut self,
        instruction_data: &InstructionData,
        opcode_data: &[u8; 2],
        mmu: &mut MMU,
    ) -> Result<u8, Error> {
        match instruction_data.instruction {
            Instruction::PUSH_RR => {
                let qq = instruction_data.byte_data.lhs >> 1;
                let value = self.registers.reg_pair_from_qq(qq)?.get_value();

                self.push_stack_word(value, mmu)?;

                Ok(4)
            }
            Instruction::POP_RR => {
                let qq = instruction_data.byte_data.lhs >> 1;
                let value = self.pop_stack_word(mmu)?;
                let mut reg_pair = self.registers.reg_pair_from_qq(qq)?;

                reg_pair.set_value_16(value);

                Ok(3)
            }
            _ => Err(unexpected_op(&instruction_data.mnemonic, &Mnemonic::PUSH)),
        }
    }
}
