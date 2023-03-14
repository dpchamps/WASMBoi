use crate::dasm::InstructionData;
use crate::spec::clock::Clock;
use crate::spec::cpu::{CPUImpl, Error};
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;

impl CPUImpl {
    pub(crate) fn evaluate_alu(
        &mut self,
        instruction_data: &InstructionData,
        opcode_data: &[u8; 2],
    ) -> Result<Clock, Error> {
        match instruction_data.instruction {
            Instruction::ADD_AR => {
                unimplemented!()
            }
            Instruction::ADD_AN => {
                unimplemented!()
            }
            Instruction::ADD_AHL => {
                unimplemented!()
            }
            Instruction::ADC_AR => {
                unimplemented!()
            }
            Instruction::ADC_AN => {
                unimplemented!()
            }
            Instruction::ADC_AHL => {
                unimplemented!()
            }
            Instruction::SUB_R => {
                unimplemented!()
            }
            Instruction::SUB_N => {
                unimplemented!()
            }
            Instruction::SUB_HL => {
                unimplemented!()
            }
            Instruction::SBC_AR => {
                unimplemented!()
            }
            Instruction::SBC_AN => {
                unimplemented!()
            }
            Instruction::SBC_AHL => {
                unimplemented!()
            }
            Instruction::AND_R => {
                unimplemented!()
            }
            Instruction::AND_N => {
                unimplemented!()
            }
            Instruction::AND_HL => {
                unimplemented!()
            }
            Instruction::XOR_R => {
                unimplemented!()
            }
            Instruction::XOR_N => {
                unimplemented!()
            }
            Instruction::XOR_HL => {
                unimplemented!()
            }
            Instruction::OR_R => {
                unimplemented!()
            }
            Instruction::OR_N => {
                unimplemented!()
            }
            Instruction::OR_HL => {
                unimplemented!()
            }
            Instruction::CP_R => {
                unimplemented!()
            }
            Instruction::CP_N => {
                unimplemented!()
            }
            Instruction::CP_HL => {
                unimplemented!()
            }
            Instruction::INC_R => {
                unimplemented!()
            }
            Instruction::INC_HL => {
                unimplemented!()
            }
            Instruction::DEC_R => {
                unimplemented!()
            }
            Instruction::DEC_HL => {
                unimplemented!()
            }
            Instruction::DAA => {
                unimplemented!()
            }
            Instruction::CPL => {
                unimplemented!()
            }
            Instruction::ADD_HLRR => {
                unimplemented!()
            }
            Instruction::ADD_SPN => {
                unimplemented!()
            }
            Instruction::INC_RR => {
                unimplemented!()
            }
            Instruction::DEC_RR => {
                unimplemented!()
            }
            Instruction::LD_SPDD => {
                unimplemented!()
            }
            Instruction::LDHL => {
                unimplemented!()
            }
            _ => Err(unexpected_op(&instruction_data.mnemonic, &Mnemonic::ADD)),
        }
    }
}
