#![allow(arithmetic_overflow)]

use crate::dasm::InstructionData;
use crate::spec::clock::Clock;
use crate::spec::cpu::{CPUImpl, Error};
use crate::spec::mmu::MMU;
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;
use crate::util::byte_ops::hi_lo_combine;

impl CPUImpl {
    pub(crate) fn evaluate_alu(
        &mut self,
        instruction_data: &InstructionData,
        opcode_data: &[u8; 2],
        mmu: &mut MMU
    ) -> Result<u8, Error> {
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
                let data = hi_lo_combine(opcode_data[0], opcode_data[1]) as i16;
                let result = (self.registers.a as i16) - data;
                let z = (result == 0) as u8;
                let h = (((self.registers.a&0xf) as i16 - (data&0xf)) < 0) as u8;
                let n = 1 as u8;
                let c = (data > self.registers.a as i16) as u8;

                self.registers.set_flags(z, h, n, c);
                Ok(2)
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
