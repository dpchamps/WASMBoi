use crate::dasm::InstructionData;
use crate::spec::clock::Clock;
use crate::spec::cpu::{CPUImpl, Error};
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;

impl CPUImpl {
    pub(crate) fn evaluate_branch(
        &mut self,
        instruction_data: &InstructionData,
        opcode_data: &[u8; 2],
    ) -> Result<Clock, Error> {
        match instruction_data.instruction {
            Instruction::JP_NN => {
                unimplemented!()
            }
            Instruction::JP_HL => {
                unimplemented!()
            }
            Instruction::JP_FNN => {
                unimplemented!()
            }
            Instruction::JR_PCDD => {
                unimplemented!()
            }
            Instruction::JR_FPCDD => {
                unimplemented!()
            }
            Instruction::CALL_NN => {
                unimplemented!()
            }
            Instruction::CALL_FNN => {
                unimplemented!()
            }
            Instruction::RET => {
                unimplemented!()
            }
            Instruction::RET_F => {
                unimplemented!()
            }
            Instruction::RETI => {
                unimplemented!()
            }
            Instruction::RST => {
                unimplemented!()
            }
            _ => Err(unexpected_op(&instruction_data.mnemonic, &Mnemonic::PUSH)),
        }
    }
}
