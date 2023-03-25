use crate::dasm::InstructionData;
use crate::spec::clock::Clock;
use crate::spec::cpu::*;
use crate::spec::mmu::MMU;
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;
use crate::spec::register::TRegister;
use crate::util::byte_ops::hi_lo_combine;

impl CPU {
    pub(crate) fn evaluate_ld(
        &mut self,
        instruction_data: &InstructionData,
        opcode_data: &[u8; 2],
        mmu: &mut MMU,
    ) -> Result<u8, Error> {
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
                let dd = instruction_data.byte_data.lhs >> 1;

                match dd {
                    0b00 => {
                        // BC
                        self.registers.b.set_value(opcode_data[0]);
                        self.registers.c.set_value(opcode_data[1]);
                    },
                    0b01 => {
                        // DE
                        self.registers.d.set_value(opcode_data[0]);
                        self.registers.e.set_value(opcode_data[1]);
                    },
                    0b10 => {
                        // HL
                        self.registers.h.set_value(opcode_data[0]);
                        self.registers.l.set_value(opcode_data[1]);
                    },
                    0b11 => {
                        self.registers.sp.set_value(hi_lo_combine(opcode_data[1], opcode_data[0]));
                    }
                    _ => {
                        return Err(Error::UnexpectedOpcodeState(instruction_data.clone(), dd as u16))
                    }
                }

                Ok(3)
            }
            Instruction::LD_SPHL => {
                unimplemented!()
            }
            _ => Err(unexpected_op(&instruction_data.mnemonic, &Mnemonic::LD)),
        }
    }
}
