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
        mmu: &mut MMU,
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
                let carry_flag = self.registers.flag_register().c;
                let value = self
                    .registers
                    .op(|registers| RegisterOp::new(*registers.a.get_value()).rotate_left(1));

                self.registers.f.set_value(
                    FlagRegister::new(false, false, false, true).0 & *self.registers.f.get_value(),
                );

                self.registers.a.set_value((value & 0xFE) | carry_flag);

                Ok(1)
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
                self.registers.op_with_effect(|registers| {
                    let mut reg = registers.reg_from_byte(instruction_data.opcode_info.lo)?;
                    let mut result = RegisterOp::new(reg.get_eight_bit_val()?).rotate_left(1);

                    reg.set_eight_bit_val(result.value)?;

                    result.flags.update_zero(reg.get_eight_bit_val()?);

                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::RLC_HL => {
                self.registers.op_with_effect(|registers| {
                    let value = mmu.read_byte(registers.hl())?;
                    let mut result = RegisterOp::new(value).rotate_left(1);

                    mmu.write_byte(registers.hl(), result.value)?;
                    result.flags.update_zero(result.value);

                    Ok(result)
                })?;

                Ok(4)
            }
            Instruction::RL_R => {
                let carry_flag = self.registers.flag_register().c;

                self.registers.op_with_effect(|registers| {
                    let mut reg = registers.reg_from_byte(instruction_data.opcode_info.lo)?;
                    let mut result = RegisterOp::new(reg.get_eight_bit_val()?).rotate_left(1);

                    reg.set_eight_bit_val((result.value & 0xFE) | carry_flag)?;
                    result.flags.update_zero(reg.get_eight_bit_val()?);

                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::RL_HL => {
                let carry_flag = self.registers.flag_register().c;

                self.registers.op_with_effect(|registers| {
                    let value = mmu.read_byte(registers.hl())?;
                    let mut result = RegisterOp::new(value).rotate_left(1);
                    let carried_result = (result.value & 0xFE) | carry_flag;

                    mmu.write_byte(registers.hl(), carried_result)?;
                    result.flags.update_zero(carried_result);

                    Ok(result)
                })?;

                Ok(4)
            }
            Instruction::RRC_R => {
                self.registers.op_with_effect(|registers| {
                    let mut reg = registers.reg_from_byte(instruction_data.opcode_info.lo)?;
                    let mut result = RegisterOp::new(reg.get_eight_bit_val()?).rotate_right(1);

                    reg.set_eight_bit_val(result.value)?;

                    result.flags.update_zero(reg.get_eight_bit_val()?);

                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::RRC_HL => {
                self.registers.op_with_effect(|registers| {
                    let value = mmu.read_byte(registers.hl())?;
                    let mut result = RegisterOp::new(value).rotate_right(1);

                    mmu.write_byte(registers.hl(), result.value)?;

                    result.flags.update_zero(result.value);

                    Ok(result)
                })?;

                Ok(4)
            }
            Instruction::RR_R => {
                let carry_flag = (self.registers.flag_register().c << 7) | 0x7F;

                self.registers.op_with_effect(|registers| {
                    let mut reg = registers.reg_from_byte(instruction_data.opcode_info.lo)?;
                    let mut result = RegisterOp::new(reg.get_eight_bit_val()?).rotate_right(1);
                    reg.set_eight_bit_val(carry_flag & (result.value | 0x80))?;

                    result.flags.update_zero(reg.get_eight_bit_val()?);

                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::RR_HL => {
                let carry_flag = (self.registers.flag_register().c << 7) | 0x7F;

                self.registers.op_with_effect(|registers| {
                    let value = mmu.read_byte(registers.hl())?;
                    let mut result = RegisterOp::new(value).rotate_right(1);
                    let carried_result = carry_flag & (result.value | 0x80);

                    mmu.write_byte(registers.hl(), carried_result)?;

                    result.flags.update_zero(carried_result);

                    Ok(result)
                })?;

                Ok(4)
            }
            Instruction::SLA_R => {
                self.registers.op_with_effect(|registers| {
                    let mut reg = registers.reg_from_byte(instruction_data.opcode_info.lo)?;
                    let mut result = RegisterOp::new(reg.get_eight_bit_val()?).rotate_left(1);

                    reg.set_eight_bit_val(result.value & 0xFE)?;
                    result.flags.update_zero(reg.get_eight_bit_val()?);

                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::SLA_HL => {
                self.registers.op_with_effect(|registers| {
                    let value = mmu.read_byte(registers.hl())?;
                    let mut result = RegisterOp::new(value).rotate_left(1);
                    let carried_result = result.value & 0xFE;

                    mmu.write_byte(registers.hl(), carried_result)?;
                    result.flags.update_zero(carried_result);

                    Ok(result)
                })?;

                Ok(4)
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
                    let value = mmu.read_byte(registers.hl())?;
                    let result = RegisterOp::new(value).swap();

                    mmu.write_byte(registers.hl(), result.value)?;

                    Ok(result)
                })?;

                Ok(4)
            }
            Instruction::SRA_R => {
                self.registers.op_with_effect(|registers| {
                    let mut reg = registers.reg_from_byte(instruction_data.opcode_info.lo)?;
                    let bit_val = reg.get_eight_bit_val()? & 0x80;
                    let mut result = RegisterOp::new(reg.get_eight_bit_val()?).rotate_right(1);

                    reg.set_eight_bit_val((result.value & 0x7f) | bit_val)?;
                    result.flags.update_zero(reg.get_eight_bit_val()?);

                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::SRA_HL => {
                self.registers.op_with_effect(|registers| {
                    let value = mmu.read_byte(registers.hl())?;
                    let bit_val = value & 0x80;
                    let mut result = RegisterOp::new(value).rotate_right(1);
                    let carried_result = (result.value & 0x7f) | bit_val;

                    mmu.write_byte(registers.hl(), carried_result)?;
                    result.flags.update_zero(carried_result);

                    Ok(result)
                })?;

                Ok(4)
            }
            Instruction::SRL_R => {
                self.registers.op_with_effect(|registers| {
                    let mut reg = registers.reg_from_byte(instruction_data.opcode_info.lo)?;
                    let mut result = RegisterOp::new(reg.get_eight_bit_val()?).rotate_right(1);

                    reg.set_eight_bit_val(0b01111111 & result.value)?;
                    result.flags.update_zero(reg.get_eight_bit_val()?);

                    Ok(result)
                })?;

                Ok(2)
            }
            Instruction::SRL_HL => {
                self.registers.op_with_effect(|registers| {
                    let value = mmu.read_byte(registers.hl())?;
                    let mut result = RegisterOp::new(value).rotate_right(1);
                    let carried_result = 0b01111111 & result.value;

                    mmu.write_byte(registers.hl(), carried_result)?;
                    result.flags.update_zero(carried_result);

                    Ok(result)
                })?;

                Ok(4)
            }
            Instruction::BIT_NR => {
                let bit = 1 << instruction_data.opcode_info.hi;
                let reg = self
                    .registers
                    .reg_from_byte(instruction_data.opcode_info.lo)?;

                let selected_bit = reg.get_eight_bit_val()? & bit;
                let mut flags = self.registers.flag_register();
                flags.z = (selected_bit == 0) as u8;
                flags.h = 1;
                flags.n = 0;

                self.registers.f.set_value(flags.into());

                Ok(2)
            }
            Instruction::BIT_NHL => {
                let bit = 1 << instruction_data.opcode_info.hi;
                let value = mmu.read_byte(self.registers.hl())?;

                let selected_bit = value & bit;
                let mut flags = self.registers.flag_register();
                flags.z = (selected_bit == 0) as u8;
                flags.h = 1;
                flags.n = 0;

                self.registers.f.set_value(flags.into());

                Ok(3)
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
                let bit = 1 << instruction_data.opcode_info.hi;
                let value = mmu.read_byte(self.registers.hl())?;

                mmu.write_byte(self.registers.hl(), value | bit)?;

                Ok(4)
            }
            Instruction::RES_NR => {
                let bit = 1 << instruction_data.opcode_info.hi;
                let mut reg = self
                    .registers
                    .reg_from_byte(instruction_data.opcode_info.lo)?;

                // 100 -> 11111011
                // 0b10101110 & 11111011

                reg.set_eight_bit_val(reg.get_eight_bit_val()? & !bit)?;

                Ok(2)
            }
            Instruction::RES_NHL => {
                let bit = 1 << instruction_data.opcode_info.hi;
                let value = mmu.read_byte(self.registers.hl())?;

                mmu.write_byte(self.registers.hl(), value & !bit)?;

                Ok(4)
            }
            _ => Err(unexpected_op(&instruction_data.mnemonic, &Mnemonic::PUSH)),
        }
    }
}
