use crate::dasm::InstructionData;
use crate::spec::clock::Clock;
use crate::spec::cpu::{CPUImpl, Error};
use crate::spec::mmu::MMU;
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;

impl CPUImpl {
    pub(crate) fn evaluate_branch(
        &mut self,
        instruction_data: &InstructionData,
        opcode_data: &[u8; 2],
        mmu: &mut MMU
    ) -> Result<u8, Error> {
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
                mmu.write_word(self.registers.sp, self.registers.pc).map_err(Error::MmuReadError)?;
                self.registers.sp -= 2;
                self.registers.pc = (instruction_data.byte_data.lhs as u16) * 8;;
                Ok(4)
            }
            _ => Err(unexpected_op(&instruction_data.mnemonic, &Mnemonic::PUSH)),
        }
    }
}
