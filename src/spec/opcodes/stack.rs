use crate::dasm::InstructionData;
use crate::spec::clock::Clock;
use crate::spec::cpu::{CPUImpl, Error};
use crate::spec::mmu::MMU;
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;

impl CPUImpl {
    pub(crate) fn evaluate_stack_op(
        &mut self,
        instruction_data: &InstructionData,
        opcode_data: &[u8; 2],
        mmu: &mut MMU
    ) -> Result<u8, Error> {
        match instruction_data.instruction {
            Instruction::PUSH_RR => {
                unimplemented!()
            }
            Instruction::POP_RR => {
                unimplemented!()
            }
            _ => Err(unexpected_op(&instruction_data.mnemonic, &Mnemonic::PUSH)),
        }
    }
}
