#![allow(arithmetic_overflow)]

use crate::dasm::InstructionData;
use crate::spec::clock::Clock;
use crate::spec::cpu::{Error, CPU};
use crate::spec::mmu::MMU;
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;
use crate::spec::register::{RegisterError, RegisterRefMut, RegisterType, TRegister};
use crate::spec::register_ops::{Flags, RegisterOp, RegisterOpResult};
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
                    let reg_r_val = registers.reg_from_byte(instruction_data.byte_data.rhs)?.get_eight_bit_val()?;
                    let op = RegisterOp::new(*registers.a.get_value())
                        .add(reg_r_val);

                    registers.a.set_value(op.value);

                    Ok(op)
                })?;

                Ok(1)
            }
            Instruction::ADD_AN => {
                self.registers.op_with_effect(|registers| {
                    let result = RegisterOp::new(*registers.a.get_value()).add(opcode_data[0]);
                    registers.a.set_value(result.value);
                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::ADD_AHL => {
                unimplemented!()
            }
            Instruction::ADC_AR => {
                unimplemented!()
            }
            Instruction::ADC_AN => {
                self.registers.op_with_effect(|registers| {
                    let mut result = RegisterOp::new(*registers.a.get_value()).add(opcode_data[0] + registers.flag_register().z);
                    registers.a.set_value(result.value);

                    result.flags.update(|flags| {
                        let mut next_flags = flags.clone();
                        next_flags.n = 0;

                        next_flags
                    });

                    Ok(result)
                })?;

                Ok(2)
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
                self.registers.op_with_effect(|registers| {
                    let result = RegisterOp::new(*registers.a.get_value()).and(opcode_data[0]);

                    registers.a.set_value(result.value);

                    Ok(result)
                })?;
                Ok(2)
            }
            Instruction::AND_HL => {
                unimplemented!()
            }
            Instruction::XOR_R => {
                let reg_r_value = self.registers.reg_from_byte(instruction_data.byte_data.rhs)?.get_eight_bit_val()?;
                self.registers.op_with_effect(|registers| {
                    let result = RegisterOp::new(*registers.a.get_value()).xor(reg_r_value);
                    registers.a.set_value(result.value);
                    Ok(result)
                })?;

                Ok(1)
            }
            Instruction::XOR_N => {
                self.registers.op_with_effect(|registers| {
                    let result = RegisterOp::new(*registers.a.get_value()).xor(opcode_data[0]);
                    registers.a.set_value(result.value);
                    Ok(result)
                })?;

                Ok(1)
            }
            Instruction::XOR_HL => {
                let value = mmu.read_byte(self.registers.hl())?;
                self.registers.op_with_effect(|registers| {
                    let result = RegisterOp::new(*registers.a.get_value()).xor(value);

                    registers.a.set_value(result.value);
                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::OR_R => {
                self.registers.op_with_effect(|registers|{
                    let mut reg_r_val = registers.reg_from_byte(instruction_data.byte_data.rhs)?.get_eight_bit_val()?;
                    let result = RegisterOp::new(*registers.a.get_value()).or(reg_r_val);

                    registers.a.set_value(result.value);
                    Ok(result)
                })?;

                Ok(1)
            }
            Instruction::OR_N => {
                unimplemented!()
            }
            Instruction::OR_HL => {
                let value = mmu.read_byte(self.registers.hl())?;
                self.registers.op_with_effect(|registers| {
                    let result = RegisterOp::new(*registers.a.get_value()).or(value);
                    registers.a.set_value(result.value);

                    Ok(result)
                })?;
                Ok(2)
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
                        }
                        _ => Err(RegisterError::InvalidLookupInput),
                    }
                })?;

                Ok(1)
            }
            Instruction::INC_HL => {
                unimplemented!()
            }
            Instruction::DEC_R => {
                self.registers.op_with_effect(|registers| {
                    let byte_reg = registers.reg_from_byte(instruction_data.byte_data.lhs)?;

                    match byte_reg {
                        RegisterRefMut::Byte(reg) => {
                            let reg_op = RegisterOp::new(*reg.get_value()).sub(1);
                            reg.set_value(reg_op.value);
                            Ok(reg_op)
                        }
                        _ => Err(RegisterError::InvalidLookupInput),
                    }
                })?;


                Ok(1)
            }
            Instruction::DEC_HL => {
                self.registers.op_with_effect(|registers| {
                    let result = RegisterOp::new(registers.hl()).sub(1);

                    registers.hl_mut().set_value_16(result.value);

                    Ok(result)
                })?;

                Ok(3)
            }
            Instruction::DAA => {
                unimplemented!()
            }
            Instruction::CPL => {
                unimplemented!()
            }
            Instruction::ADD_HLRR => {
                let dd = instruction_data.byte_data.lhs >> 1;
                let reg_value = self.registers.reg_pair_from_dd(dd)?.get_value();
                self.registers.op_with_effect(|register| {
                    let mut hl = register.hl_mut();
                    let result = RegisterOp::new(hl.get_value()).add(reg_value);

                    // println!("\t\t {:X?} <- {:X} ({:X} + {:X})", hl, result.value, hl.get_value(), reg_value);

                    hl.set_value_16(result.value);

                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::ADD_SPN => {
                unimplemented!()
            }
            Instruction::INC_RR => {
                let dd = instruction_data.byte_data.lhs >> 1;
                let mut reg_pair = self.registers.reg_pair_from_dd(dd)?;
                let result = Wrapping(reg_pair.get_value()) + Wrapping(1);

                reg_pair.set_value_16(result.0);
                Ok(2)
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
