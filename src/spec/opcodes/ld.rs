use crate::dasm::InstructionData;

use crate::spec::cpu::*;
use crate::spec::mmu::MMU;
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;
use crate::spec::register::{RegisterRefMut, TRegister};
use crate::spec::register_ops::RegisterOp;
use crate::util::byte_ops::hi_lo_combine;
use std::num::Wrapping;
use std::ops::Add;

impl CPU {
    pub(crate) fn evaluate_ld(
        &mut self,
        instruction_data: &InstructionData,
        opcode_data: &[u8; 2],
        mmu: &mut MMU,
    ) -> Result<u8, Error> {
        match instruction_data.instruction {
            Instruction::LD_RR => {
                let r_prime_value = self
                    .registers
                    .reg_from_byte(instruction_data.opcode_info.lo)?
                    .get_eight_bit_val()?;
                let mut r = self
                    .registers
                    .reg_from_byte(instruction_data.opcode_info.hi)?;
                // println!("\t\t {:?} <- {:X}", r, r_prime_value);

                r.set_eight_bit_val(r_prime_value)?;

                Ok(1)
            }
            Instruction::LD_RN => {
                match self
                    .registers
                    .reg_from_byte(instruction_data.opcode_info.hi)?
                {
                    RegisterRefMut::Byte(byte_ref) => {
                        // println!("\t\t {:?} <- {:X}", byte_ref, opcode_data[0]);
                        byte_ref.set_value(opcode_data[0]);
                        Ok(2)
                    }
                    _ => Err(Error::UnexpectedOpcodeState(
                        instruction_data.clone(),
                        hi_lo_combine(opcode_data[1], opcode_data[0]),
                    )),
                }
            }
            Instruction::LD_RHL => {
                let value = mmu.read_byte(self.registers.hl())?;
                let mut reg = self
                    .registers
                    .reg_from_byte(instruction_data.opcode_info.hi)?;

                reg.set_eight_bit_val(value)?;

                Ok(2)
            }
            Instruction::LD_HLR => {
                let reg_r_value = self
                    .registers
                    .reg_from_byte(instruction_data.opcode_info.lo)?
                    .get_eight_bit_val()?;
                mmu.write_byte(self.registers.hl(), reg_r_value)?;

                Ok(2)
            }
            Instruction::LD_HLN => {
                unimplemented!()
            }
            Instruction::LD_ABC => {
                unimplemented!()
            }
            Instruction::LD_ADE => {
                let value = mmu.read_byte(self.registers.de())?;
                self.registers.a.set_value(value);

                Ok(2)
            }
            Instruction::LD_AN => {
                let value = mmu.read_byte(0xFF00 + (opcode_data[0] as u16))?;

                self.registers.a.set_value(value);
                Ok(3)
            }
            Instruction::LD_ANN => {
                let value = mmu.read_byte(hi_lo_combine(opcode_data[1], opcode_data[0]))?;

                self.registers.a.set_value(value);

                Ok(4)
            }
            Instruction::LD_BCA => {
                unimplemented!()
            }
            Instruction::LD_DEA => {
                mmu.write_byte(self.registers.de(), *self.registers.a.get_value())?;
                Ok(2)
            }
            Instruction::LD_NA => {
                let address = 0xFF00 + (opcode_data[0] as u16);
                mmu.write_byte(address, *self.registers.a.get_value())?;
                Ok(3)
            }
            Instruction::LD_NNA => {
                let address = hi_lo_combine(opcode_data[1], opcode_data[0]);
                mmu.write_byte(address, *self.registers.a.get_value())?;
                Ok(4)
            }
            Instruction::LD_AFF00C => {
                unimplemented!()
            }
            Instruction::LD_FF00CA => {
                unimplemented!()
            }
            Instruction::LD_HLIA => {
                let hl = self.registers.hl();
                mmu.write_byte(hl, *self.registers.a.get_value())?;
                let next_hl = Wrapping(hl) + Wrapping(1);
                self.registers.hl_mut().set_value_16(next_hl.0);

                Ok(2)
            }
            Instruction::LD_AHLI => {
                let hl = self.registers.hl();
                let value = mmu.read_byte(hl)?;

                let next_hl = Wrapping(hl) + Wrapping(1);

                self.registers.a.set_value(value);
                self.registers.hl_mut().set_value_16(next_hl.0);

                Ok(2)
            }
            Instruction::LD_HLDA => {
                let hl = self.registers.hl();
                mmu.write_byte(hl, *self.registers.a.get_value())?;
                let next_hl = Wrapping(hl) - Wrapping(1);
                self.registers.hl_mut().set_value_16(next_hl.0);

                Ok(2)
            }
            Instruction::LD_AHLD => {
                unimplemented!()
            }
            Instruction::LD_RRNN => {
                let dd = instruction_data.opcode_info.hi >> 1;
                let mut reg_pair = self.registers.reg_pair_from_dd(dd)?;
                // println!("\t\t{:?} <- {:X?}", reg_pair, opcode_data);

                reg_pair.set_value(opcode_data[1], opcode_data[0]);

                Ok(3)
            }
            Instruction::LD_SPHL => {
                self.registers.sp.set_value(self.registers.hl());

                Ok(2)
            }
            Instruction::LD_SPDD => {
                let address = hi_lo_combine(opcode_data[1], opcode_data[0]);
                mmu.write_word(address, *self.registers.sp.get_value())?;

                Ok(5)
            }
            Instruction::LDHL => {
                self.registers.op_with_effect(|registers| {
                    let mut result = RegisterOp::new(*registers.sp.get_value() as i16)
                        .add((opcode_data[0] as i8) as i16);

                    result.flags.update(|flags| {
                        let mut next = flags;
                        next.z = 0;

                        next
                    });

                    registers.hl_mut().set_value_16(result.value as u16);

                    Ok(result)
                })?;

                Ok(3)
            }
            _ => Err(unexpected_op(&instruction_data.mnemonic, &Mnemonic::LD)),
        }
    }
}
