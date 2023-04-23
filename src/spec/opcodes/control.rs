use crate::dasm::InstructionData;

use crate::spec::cpu::{Error, CPU};
use crate::spec::mmu::MMU;
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::unexpected_op;
use crate::spec::register::TRegister;

impl CPU {
    pub(crate) fn evaluate_control(
        &mut self,
        instruction_data: &InstructionData,
        _opcode_data: &[u8; 2],
        mmu: &mut MMU,
    ) -> Result<u8, Error> {
        match instruction_data.instruction {
            Instruction::CCF => {
                let mut flags = self.registers.flag_register();
                flags.c = !(flags.c != 0) as u8;
                flags.h = 0;
                flags.n = 0;

                self.registers.f.set_value(flags.into());

                Ok(1)
            }
            Instruction::SCF => {
                let mut flags = self.registers.flag_register();
                flags.c = 1;
                flags.h = 0;
                flags.n = 0;

                self.registers.f.set_value(flags.into());

                Ok(1)
            }
            Instruction::NOP => Ok(1),
            Instruction::HALT => {
                self.halt = true;
                Ok(1)
            }
            Instruction::STOP => {
                unimplemented!()
            }
            Instruction::DI => {
                mmu.write_interrupt_enable_reg(false);
                Ok(1)
            }
            Instruction::EI => {
                mmu.write_interrupt_enable_reg(true);
                Ok(1)
            }
            _ => Err(unexpected_op(&instruction_data.mnemonic, &Mnemonic::PUSH)),
        }
    }
}
