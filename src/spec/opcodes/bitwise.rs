use crate::dasm::InstructionData;

use crate::spec::cpu::{Error, CPU};
use crate::spec::mmu::MMU;
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;
use crate::spec::register::TRegister;
use crate::spec::register_ops::{FlagRegister, RegisterOp};

impl CPU {
    pub(crate) fn evaluate_bitwise(
        &mut self,
        instruction_data: &InstructionData,
        _opcode_data: &[u8; 2],
        _mmu: &mut MMU,
    ) -> Result<u8, Error> {
        match instruction_data.instruction {
            Instruction::RLCA => {
                let value = self
                    .registers
                    .op(|registers| RegisterOp::new(*registers.a.get_value()).rotate_left(1));

                self.registers.a.set_value(value);
                Ok(1)
            }
            Instruction::RLA => {
                unimplemented!()
            }
            Instruction::RRCA => {
                let value = self
                    .registers
                    .op(|registers| RegisterOp::new(*registers.a.get_value()).rotate_right(1));

                self.registers.a.set_value(value);

                Ok(1)
            }
            Instruction::RRA => {
                // 0b10000001, cy = 0, carry_flag = 0
                //  0b11000000, cy = 1 (0b01111111 | carry_flag) & value
                // 0b01000000, cy = 1
                let carry_flag = (self.registers.flag_register().c << 7) | 0x7F;
                let value = self
                    .registers
                    .op(|registers| RegisterOp::new(*registers.a.get_value()).rotate_right(1));

                self.registers.f.set_value(
                    FlagRegister::new(false, false, false, true).0 & *self.registers.f.get_value(),
                );

                self.registers.a.set_value(carry_flag & (value | 0x80));

                Ok(1)
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
                let carry_flag = (self.registers.flag_register().c << 7) | 0x7F;

                self.registers.op_with_effect(|registers| {
                    let mut reg = registers.reg_from_byte(instruction_data.opcode_info.lo)?;
                    let mut result = RegisterOp::new(reg.get_eight_bit_val()?).rotate_right(1);
                    reg.set_eight_bit_val(carry_flag & (result.value | 0x80))?;

                    result.flags.update(|mut f| {
                        f.z = (reg.get_eight_bit_val().unwrap() == 0) as u8;

                        f
                    });

                    Ok(result)
                })?;

                Ok(2)
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
                self.registers.op_with_effect(|registers| {
                    let mut reg = registers.reg_from_byte(instruction_data.opcode_info.lo)?;
                    let result = RegisterOp::new(reg.get_eight_bit_val()?).swap();

                    reg.set_eight_bit_val(result.value)?;

                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::SWAP_HL => {
                self.registers.op_with_effect(|registers| {
                    let mut reg = registers.reg_from_byte(instruction_data.opcode_info.lo)?;
                    let result = RegisterOp::new(reg.get_sixtn_bit_val()?).swap();

                    reg.set_sixtn_bit_val(result.value)?;

                    Ok(result)
                })?;

                Ok(4)
            }
            Instruction::SRA_R => {
                unimplemented!()
            }
            Instruction::SRA_HL => {
                unimplemented!()
            }
            Instruction::SRL_R => {
                self.registers.op_with_effect(|registers| {
                    let mut reg = registers.reg_from_byte(instruction_data.opcode_info.lo)?;
                    let result = RegisterOp::new(reg.get_eight_bit_val()?).rotate_right(1);

                    reg.set_eight_bit_val(0b01111111 & result.value)?;

                    Ok(result)
                })?;

                Ok(2)
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
                let bit = 1 << instruction_data.opcode_info.hi;
                let mut reg = self
                    .registers
                    .reg_from_byte(instruction_data.opcode_info.lo)?;

                reg.set_eight_bit_val(reg.get_eight_bit_val()? | bit)?;

                Ok(2)
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
