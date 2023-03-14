use crate::dasm::InstructionData;
use crate::spec::clock::Clock;
use crate::spec::cpu::*;
use crate::spec::opcode::Instruction;


impl CPUImpl {

    pub fn evaluate_ld(&mut self, instruction_data: &InstructionData, opcode_data: &[u8; 2]) -> Result<Clock, Error> {
        match instruction_data.instruction {
            Instruction::LD_RR => {
                unimplemented!()
            }
            Instruction::LD_RN => {
                unimplemented!()
            }
            Instruction::LD_RHL => {
                unimplemented!()
            }
            Instruction::LD_HLR => {
                unimplemented!()
            }
            Instruction::LD_HLN => {
                unimplemented!()
            }
            Instruction::LD_ABC => {
                unimplemented!()
            }
            Instruction::LD_ADE => {
                unimplemented!()
            }
            Instruction::LD_AN => {
                unimplemented!()
            }
            Instruction::LD_ANN => {
                unimplemented!()
            }
            Instruction::LD_BCA => {
                unimplemented!()
            }
            Instruction::LD_DEA => {
                unimplemented!()
            }
            Instruction::LD_NA => {
                unimplemented!()
            }
            Instruction::LD_NNA => {
                unimplemented!()
            }
            Instruction::LD_AFF00C => {
                unimplemented!()
            }
            Instruction::LD_FF00CA => {
                unimplemented!()
            }
            Instruction::LD_HLIA => {
                unimplemented!()
            }
            Instruction::LD_AHLI => {
                unimplemented!()
            }
            Instruction::LD_HLDA => {
                unimplemented!()
            }
            Instruction::LD_AHLD => {
                unimplemented!()
            }
            Instruction::LD_RRNN => {
                unimplemented!()
            }
            Instruction::LD_SPHL => {
                unimplemented!()
            }
            _ => panic!("Called evaluate_ld with a non-ld like opcode. This is an unrecoverable program error")
        }
    }
}