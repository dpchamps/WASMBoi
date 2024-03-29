use crate::dasm::InstructionData;

use crate::spec::cpu::{Error, CPU};
use crate::spec::mmu::MMU;
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;
use crate::spec::register::{RegisterError, RegisterRefMut, TRegister};
use crate::spec::register_ops::{FlagRegister, RegisterOp};

use std::num::Wrapping;

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
                    let reg_r_val = registers
                        .reg_from_byte(instruction_data.opcode_info.lo)?
                        .get_eight_bit_val()?;
                    let op = RegisterOp::new(*registers.a.get_value()).add(reg_r_val);

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
                let value = mmu.read_byte(self.registers.hl())?;

                self.registers.op_with_effect(|registers| {
                    let result = RegisterOp::new(*registers.a.get_value()).add(value);

                    registers.a.set_value(result.value);

                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::ADC_AR => {
                let value = self
                    .registers
                    .reg_from_byte(instruction_data.opcode_info.lo)?
                    .get_eight_bit_val()?;
                self.registers.op_with_effect(|registers| {
                    let result =
                        RegisterOp::from(RegisterOp::new(*registers.a.get_value()).add(value))
                            .add(registers.flag_register().c);

                    registers.a.set_value(result.value);

                    Ok(result)
                })?;
                Ok(1)
            }
            Instruction::ADC_AN => {
                self.registers.op_with_effect(|registers| {
                    let result = RegisterOp::from(
                        RegisterOp::new(*registers.a.get_value()).add(opcode_data[0]),
                    )
                    .add(registers.flag_register().c);

                    registers.a.set_value(result.value);

                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::ADC_AHL => {
                let value = mmu.read_byte(self.registers.hl())?;
                self.registers.op_with_effect(|registers| {
                    let result =
                        RegisterOp::from(RegisterOp::new(*registers.a.get_value()).add(value))
                            .add(registers.flag_register().c);

                    registers.a.set_value(result.value);

                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::SUB_R => {
                let value = self
                    .registers
                    .reg_from_byte(instruction_data.opcode_info.lo)?
                    .get_eight_bit_val()?;
                self.registers.op_with_effect(|registers| {
                    let result = RegisterOp::new(*registers.a.get_value()).sub(value);

                    registers.a.set_value(result.value);

                    Ok(result)
                })?;

                Ok(1)
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
                let value = mmu.read_byte(self.registers.hl())?;

                self.registers.op_with_effect(|registers| {
                    let op_result = RegisterOp::new(*registers.a.get_value()).sub(value);

                    registers.a.set_value(op_result.value);

                    Ok(op_result)
                })?;

                Ok(2)
            }
            Instruction::SBC_AR => {
                let value = self
                    .registers
                    .reg_from_byte(instruction_data.opcode_info.lo)?
                    .get_eight_bit_val()?;

                self.registers.op_with_effect(|registers| {
                    let result =
                        RegisterOp::from(RegisterOp::new(*registers.a.get_value()).sub(value))
                            .sub(registers.flag_register().c);

                    registers.a.set_value(result.value);

                    Ok(result)
                })?;
                Ok(1)
            }
            Instruction::SBC_AN => {
                self.registers.op_with_effect(|registers| {
                    let result = RegisterOp::from(
                        RegisterOp::new(*registers.a.get_value()).sub(opcode_data[0]),
                    )
                    .sub(registers.flag_register().c);

                    registers.a.set_value(result.value);

                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::SBC_AHL => {
                let value = mmu.read_byte(self.registers.hl())?;
                self.registers.op_with_effect(|registers| {
                    let result =
                        RegisterOp::from(RegisterOp::new(*registers.a.get_value()).sub(value))
                            .sub(registers.flag_register().c);

                    registers.a.set_value(result.value);

                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::AND_R => {
                let value = self
                    .registers
                    .reg_from_byte(instruction_data.opcode_info.lo)?
                    .get_eight_bit_val()?;
                self.registers.op_with_effect(|register| {
                    let result = RegisterOp::new(*register.a.get_value()).and(value);

                    register.a.set_value(result.value);

                    Ok(result)
                })?;

                Ok(1)
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
                let value = mmu.read_byte(self.registers.hl())?;
                self.registers.op_with_effect(|registers| {
                    let result = RegisterOp::new(*registers.a.get_value()).and(value);

                    registers.a.set_value(result.value);

                    Ok(result)
                })?;
                Ok(2)
            }
            Instruction::XOR_R => {
                let reg_r_value = self
                    .registers
                    .reg_from_byte(instruction_data.opcode_info.lo)?
                    .get_eight_bit_val()?;
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

                Ok(2)
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
                self.registers.op_with_effect(|registers| {
                    let reg_r_val = registers
                        .reg_from_byte(instruction_data.opcode_info.lo)?
                        .get_eight_bit_val()?;
                    let result = RegisterOp::new(*registers.a.get_value()).or(reg_r_val);

                    registers.a.set_value(result.value);
                    Ok(result)
                })?;

                Ok(1)
            }
            Instruction::OR_N => {
                self.registers.op_with_effect(|registers| {
                    let result = RegisterOp::new(*registers.a.get_value()).or(opcode_data[0]);
                    registers.a.set_value(result.value);

                    Ok(result)
                })?;

                Ok(2)
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
                let value = self
                    .registers
                    .reg_from_byte(instruction_data.opcode_info.lo)?
                    .get_eight_bit_val()?;
                self.registers
                    .op(|registers| RegisterOp::new(*registers.a.get_value()).sub(value));

                Ok(1)
            }
            Instruction::CP_N => {
                self.registers
                    .op(|registers| RegisterOp::new(*registers.a.get_value()).sub(opcode_data[0]));

                Ok(2)
            }
            Instruction::CP_HL => {
                let value = mmu.read_byte(self.registers.hl())?;

                self.registers
                    .op(|registers| RegisterOp::new(*registers.a.get_value()).sub(value));

                Ok(2)
            }
            Instruction::INC_R => {
                self.registers.op_with_effect(|registers| {
                    let byte_reg = registers.reg_from_byte(instruction_data.opcode_info.hi)?;

                    match byte_reg {
                        RegisterRefMut::Byte(reg) => {
                            let mut reg_op = RegisterOp::new(*reg.get_value()).add(1);
                            reg_op.set_mask(FlagRegister::new(true, true, true, false));

                            reg.set_value(reg_op.value);
                            Ok(reg_op)
                        }
                        _ => Err(RegisterError::InvalidLookupInput),
                    }
                })?;
                Ok(1)
            }
            Instruction::INC_HL => {
                self.registers.op_with_effect(|registers| {
                    let value = mmu.read_byte(registers.hl())?;
                    let mut result = RegisterOp::new(value).add(1);
                    result.set_mask(FlagRegister::new(true, true, true, false));

                    mmu.write_byte(registers.hl(), result.value)?;
                    Ok(result)
                })?;

                Ok(3)
            }
            Instruction::DEC_R => {
                self.registers.op_with_effect(|registers| {
                    let byte_reg = registers.reg_from_byte(instruction_data.opcode_info.hi)?;

                    match byte_reg {
                        RegisterRefMut::Byte(reg) => {
                            let mut reg_op = RegisterOp::new(*reg.get_value()).sub(1);
                            reg_op.set_mask(FlagRegister::new(true, true, true, false));

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
                    let value = mmu.read_byte(registers.hl())?;
                    let mut result = RegisterOp::new(value).sub(1);
                    result.set_mask(FlagRegister::new(true, true, true, false));

                    mmu.write_byte(registers.hl(), result.value)?;

                    Ok(result)
                })?;

                Ok(3)
            }
            Instruction::DAA => {
                let mut flags = self.registers.flag_register();
                let mut a_value = *self.registers.a.get_value();
                // print!("{:?} {} ", flags, self.registers.a);

                match flags.n {
                    0 => {
                        if flags.c != 0 || a_value > 0x99 {
                            a_value = (Wrapping(a_value) + Wrapping(0x60)).0;
                            flags.c = 1;
                        }
                        if flags.h != 0 || (a_value & 0x0f) > 0x09 {
                            a_value = (Wrapping(a_value) + Wrapping(0x6)).0;
                        }
                    }
                    _ => {
                        if flags.c != 0 {
                            a_value = (Wrapping(a_value) - Wrapping(0x60)).0;
                        }

                        if flags.h != 0 {
                            a_value = (Wrapping(a_value) - Wrapping(0x6)).0;
                        }
                    }
                }

                flags.z = (a_value == 0) as u8;
                flags.h = 0;

                self.registers.f.set_value(FlagRegister::from(flags).0);
                self.registers.a.set_value(a_value);

                // println!("\t -> {:?} {}", self.registers.flag_register(), self.registers.a);

                Ok(1)
            }
            Instruction::CPL => {
                let flags = self.registers.flag_register();
                let a_value = *self.registers.a.get_value();

                self.registers.a.set_value(!a_value);
                self.registers
                    .f
                    .set_value(FlagRegister::new(flags.z != 0, true, true, flags.c != 0).0);

                Ok(1)
            }
            Instruction::ADD_HLRR => {
                let dd = instruction_data.opcode_info.hi >> 1;
                let reg_value = self.registers.reg_pair_from_dd(dd)?.get_value();
                self.registers.op_with_effect(|register| {
                    let mut hl = register.hl_mut();
                    let mut result = RegisterOp::new(hl.get_value()).add(reg_value);

                    result.set_mask(FlagRegister::new(false, true, true, true));

                    hl.set_value_16(result.value);

                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::ADD_SPN => {
                self.registers.op_with_effect(|registers| {
                    let mut result = RegisterOp::new(*registers.sp.get_value() as i16)
                        .add((opcode_data[0] as i8) as i16);

                    result.flags.update(|flags| {
                        let mut next = flags;
                        next.z = 0;

                        next
                    });

                    registers.sp.set_value(result.value as u16);

                    Ok(result)
                })?;
                Ok(4)
            }
            Instruction::INC_RR => {
                let dd = instruction_data.opcode_info.hi >> 1;
                let mut reg_pair = self.registers.reg_pair_from_dd(dd)?;
                let result = Wrapping(reg_pair.get_value()) + Wrapping(1);

                reg_pair.set_value_16(result.0);
                Ok(2)
            }
            Instruction::DEC_RR => {
                let dd = instruction_data.opcode_info.hi >> 1;
                self.registers.op_with_effect(|registers| {
                    let mut reg_pair = registers.reg_pair_from_dd(dd)?;
                    let mut result = RegisterOp::new(reg_pair.get_value()).sub(1);

                    result.set_mask(FlagRegister::new(false, false, false, false));

                    reg_pair.set_value_16(result.value);

                    Ok(result)
                })?;

                Ok(2)
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
