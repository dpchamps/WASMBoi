use crate::dasm::InstructionData;
use crate::spec::clock::Clock;
use crate::spec::cpu::{Error, TStackable, CPU};
use crate::spec::mmu::MMU;
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;
use crate::spec::register::TRegister;
use crate::util::byte_ops::hi_lo_combine;
use std::num::Wrapping;

impl CPU {
    pub(crate) fn evaluate_branch(
        &mut self,
        instruction_data: &InstructionData,
        opcode_data: &[u8; 2],
        mmu: &mut MMU,
    ) -> Result<u8, Error> {
        match instruction_data.instruction {
            Instruction::JP_NN => {
                let address = hi_lo_combine(opcode_data[1], opcode_data[0]);

                self.registers.pc.set_value(address);
                Ok(4)
            }
            Instruction::JP_HL => {
                self.registers.pc.set_value(self.registers.hl());

                Ok(1)
            }
            Instruction::JP_FNN => {
                let cc = instruction_data.opcode_info.hi & 0b011;
                let address = hi_lo_combine(opcode_data[1], opcode_data[0]);

                if self.registers.jump_condition(cc)? {
                    self.registers.pc.set_value(address);

                    return Ok(4);
                }

                Ok(3)
            }
            Instruction::JR_PCDD => {
                let offset = ((opcode_data[0] as i8) as i16);
                // println!("\t\tPC:{:X}+{} ({})", *self.registers.pc.get_value() as i16, offset as i8, offset);
                self.registers.pc.update_value_checked(|last_val| {
                    let result = ((*last_val) as i16).checked_add(offset).map(|x| x as u16);
                    // println!("\t\t\t PC{:X?}", result);
                    Ok(result)
                })?;
                Ok(3)
            }
            Instruction::JR_FPCDD => {
                let cc = instruction_data.opcode_info.hi & 0b011;
                let data = opcode_data[0];

                if self.registers.jump_condition(cc)? {
                    let val = ((data as i8) as i16);

                    // println!("\t\tPC:{:X}+{} ({})", *self.registers.pc.get_value() as i16, data as i8, val);
                    self.registers.pc.update_value_checked(|last| {
                        let result = ((*last) as i16).checked_add(val).map(|x| x as u16);
                        // println!("\t\t\t PC{:X?}", result);
                        Ok(result)
                    })?;

                    return Ok(3);
                }

                Ok(2)
            }
            Instruction::CALL_NN => {
                self.push_stack_word(*self.registers.pc.get_value(), mmu)?;
                self.registers
                    .pc
                    .set_value(hi_lo_combine(opcode_data[1], opcode_data[0]));

                Ok(6)
            }
            Instruction::CALL_FNN => {
                let cc = instruction_data.opcode_info.hi & 0b11;

                if self.registers.jump_condition(cc)? {
                    self.push_stack_word(*self.registers.pc.get_value(), mmu)?;
                    self.registers
                        .pc
                        .set_value(hi_lo_combine(opcode_data[1], opcode_data[0]));

                    return Ok(6);
                }

                Ok(3)
            }
            Instruction::RET => {
                let stack_val = self.pop_stack_word(mmu)?;
                self.registers.pc.set_value(stack_val);

                Ok(4)
            }
            Instruction::RET_F => {
                let cc = instruction_data.opcode_info.hi & 0b011;

                if self.registers.jump_condition(cc)? {
                    let stack_val = self.pop_stack_word(mmu)?;
                    self.registers.pc.set_value(stack_val);

                    return Ok(4);
                }

                Ok(3)
            }
            Instruction::RETI => {
                let stack_val = self.pop_stack_word(mmu)?;

                mmu.write_interrupt_enable_reg(true);
                self.registers.pc.set_value(stack_val);

                Ok(4)
            }
            Instruction::RST => {
                self.push_stack_word(*self.registers.pc.get_value(), mmu)?;
                self.registers
                    .pc
                    .set_value((instruction_data.opcode_info.hi as u16) * 8);
                Ok(4)
            }
            _ => Err(unexpected_op(&instruction_data.mnemonic, &Mnemonic::PUSH)),
        }
    }
}
