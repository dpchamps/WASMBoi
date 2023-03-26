#![allow(arithmetic_overflow)]

use crate::dasm::InstructionData;
use crate::spec::clock::Clock;
use crate::spec::cpu::{Error, CPU};
use crate::spec::mmu::MMU;
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;
use crate::spec::register::{RegisterError, RegisterRefMut, RegisterType, TRegister};
use crate::spec::register_ops::{RegisterOp, RegisterOpResult};
use crate::util::byte_ops::hi_lo_combine;
use std::num::Wrapping;
use std::ops::Add;

impl CPU {
    pub(crate) fn evaluate_alu(
        &mut self,
        instruction_data: &InstructionData,
        opcode_data: &[u8; 2],
        mmu: &mut MMU,
    ) -> Result<u8, Error> {
        match instruction_data.instruction {
            Instruction::ADD_AR => {
                self.registers.op_with_effect(|registers| {
                    let op = RegisterOp::new(*registers.a.get_value()).add(instruction_data.byte_data.rhs);

                    registers.a.set_value(op.value);

                    Ok(op)
                })?;

                Ok(1)
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
                self.registers.op_with_effect(|registers| {
                    let op_result = RegisterOp::new(*registers.a.get_value()).sub(opcode_data[0]);

                    registers.a.set_value(op_result.value);

                    Ok(op_result)
                })?;

                Ok(2)
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
                self.registers
                    .op(|registers| RegisterOp::new(*registers.a.get_value()).sub(opcode_data[0]));

                Ok(2)
            }
            Instruction::CP_HL => {
                unimplemented!()
            }
            Instruction::INC_R => {
                self.registers.op_with_effect(|registers| {
                    let byte_reg = registers.reg_from_byte(instruction_data.byte_data.lhs)?;

                    match byte_reg {
                        RegisterRefMut::Byte(reg) => {
                            let reg_op = RegisterOp::new(*reg.get_value()).add(1);
                            reg.set_value(reg_op.value);
                            Ok(reg_op)
                        },
                        _ => Err(RegisterError::InvalidLookupInput)
                    }
                })?;

                Ok(1)
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
