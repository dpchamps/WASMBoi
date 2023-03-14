use crate::dasm::InstructionData;
use crate::spec::clock::Clock;
use crate::spec::cpu::{CPUImpl, Error};
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;

impl CPUImpl {
    pub(crate) fn evaluate_bitwise(
        &mut self,
        instruction_data: &InstructionData,
        opcode_data: &[u8; 2],
    ) -> Result<Clock, Error> {
        match instruction_data.instruction {
            Instruction::RLCA => {
                unimplemented!()
            }
            Instruction::RLA => {
                unimplemented!()
            }
            Instruction::RRCA => {
                unimplemented!()
            }
            Instruction::RRA => {
                unimplemented!()
            }
            Instruction::RLC_R => {
                unimplemented!()
            }
            Instruction::RLC_HL => {
                unimplemented!()
            }
            Instruction::RL_R => {
                unimplemented!()
            }
            Instruction::RL_HL => {
                unimplemented!()
            }
            Instruction::RRC_R => {
                unimplemented!()
            }
            Instruction::RRC_HL => {
                unimplemented!()
            }
            Instruction::RR_R => {
                unimplemented!()
            }
            Instruction::RR_HL => {
                unimplemented!()
            }
            Instruction::SLA_R => {
                unimplemented!()
            }
            Instruction::SLA_HL => {
                unimplemented!()
            }
            Instruction::SWAP_R => {
                unimplemented!()
            }
            Instruction::SWAP_HL => {
                unimplemented!()
            }
            Instruction::SRA_R => {
                unimplemented!()
            }
            Instruction::SRA_HL => {
                unimplemented!()
            }
            Instruction::SRL_R => {
                unimplemented!()
            }
            Instruction::SRL_HL => {
                unimplemented!()
            }
            Instruction::BIT_NR => {
                unimplemented!()
            }
            Instruction::BIT_NHL => {
                unimplemented!()
            }
            Instruction::SET_NR => {
                unimplemented!()
            }
            Instruction::SET_NHL => {
                unimplemented!()
            }
            Instruction::RES_NR => {
                unimplemented!()
            }
            Instruction::RES_NHL => {
                unimplemented!()
            }
            _ => Err(unexpected_op(&instruction_data.mnemonic, &Mnemonic::PUSH)),
        }
    }
}
