use crate::dasm::{DasmError, Disassembler, InstructionData};
use crate::spec::clock::Clock;
use crate::spec::mmu::{Error as MmuError, MMU};
use crate::spec::mnemonic::Mnemonic;
use crate::spec::opcode::Instruction;
use crate::spec::opcodes::*;
use crate::spec::register::{RegisterError, Registers, TRegister};
use std::convert::TryFrom;
use std::env;
use std::num::Wrapping;

pub trait TCPU {
    type E;
    fn tick(&mut self, mmu: &mut MMU) -> Result<u8, Self::E>;
}

pub trait TStackable {
    fn push_stack_byte(&mut self, value: u8, mmu: &mut MMU) -> Result<(), Error>;
    fn push_stack_word(&mut self, value: u16, mmu: &mut MMU) -> Result<(), Error>;
    fn pop_stack_byte(&mut self, mmu: &mut MMU) -> Result<u8, Error>;
    fn pop_stack_word(&mut self, mmu: &mut MMU) -> Result<u16, Error>;
}

pub struct CpuDebug {
    current_instruction: bool,
    current_register_values: bool,
    alu: bool,
    bitwise: bool,
    control: bool,
    ld: bool,
    stack: bool,
    interrupts: bool,
    gb_doc: bool
}

impl Default for CpuDebug {
    fn default() -> Self {
        let cpu_env = env::var("CPU_DEBUG").unwrap_or("".into());
        let debug_list: Vec<&str> = cpu_env.split(",").collect();

        CpuDebug {
            current_instruction: debug_list.contains(&"PC"),
            current_register_values: debug_list.contains(&"REG"),
            alu: debug_list.contains(&"ALU"),
            bitwise: debug_list.contains(&"BIT"),
            control: debug_list.contains(&"CONTROL"),
            ld: debug_list.contains(&"LD"),
            stack: debug_list.contains(&"STACK"),
            interrupts: debug_list.contains(&"INTS"),
            gb_doc: debug_list.contains(&"GB_DOC")
        }
    }
}



impl CpuDebug {
    #[cfg(debug_assertions)]
    pub fn log<F>(&self, t: &str, f: F)
    where
        F: Fn() -> ()
    {
        let should_log = match t {
            "PC" => self.current_instruction,
            "REG" => self.current_register_values,
            "ALU" => self.alu,
            "BIT" => self.bitwise,
            "CONTROL" => self.control,
            "LD" => self.ld,
            "STACK" => self.stack,
            "INTS" => self.interrupts,
            "GB_DOC" => self.gb_doc,
            _ => false
        };

        if should_log {
            f()
        }
    }

    #[cfg(not(debug_assertions))]
    pub fn log<F>(&self, t: &str, f: F)
        where
            F: Fn() -> ()
    {

    }
}

pub struct CPU {
    pub(crate) registers: Registers,
    pub(crate) debug: CpuDebug,
    pub(crate) halt: bool
}

#[derive(Debug)]
pub enum Error {
    Default(String),
    InitializationError,
    UnexpectedOpcode(String),
    UnsupportedOpcode(Instruction),
    MmuError(MmuError),
    DecodeError(DasmError),
    RegisterError(RegisterError),
    UnexpectedOpcodeState(InstructionData, u16),
}

impl From<RegisterError> for Error {
    fn from(reg_error: RegisterError) -> Self {
        Error::RegisterError(reg_error)
    }
}

impl From<MmuError> for Error {
    fn from(mmu_error: MmuError) -> Self {
        Error::MmuError(mmu_error)
    }
}

impl TCPU for CPU {
    type E = Error;

    fn tick(&mut self, mmu: &mut MMU) -> Result<u8, Error> {

        self.gameboy_doc_debug(mmu);

        let last_pc = *self.registers.pc.get_value();
        let opcode = self.fetch(mmu)?;
        let data = [
            mmu.read_byte(*self.registers.pc.get_value())
                .map_err(Error::MmuError)?,
            mmu.read_byte((Wrapping(*self.registers.pc.get_value()) + Wrapping(1)).0)
                .map_err(Error::MmuError)?,
        ];
        self.debug.log("PC", || {
            println!("[PC: {:#X}] Op: {}, Dat: [{:X?}]",
                     last_pc,
                     opcode,
                     data)
        });
        self.registers
            .pc
            .set_value(*self.registers.pc.get_value() + opcode.size as u16);
        let cycles = self.execute(&opcode, &data, mmu)?;
        self.debug.log("REG", || {
            println!("\t{}", self.registers)
        });
        Ok(cycles)
    }
}

impl TStackable for CPU {
    fn push_stack_byte(&mut self, value: u8, mmu: &mut MMU) -> Result<(), Error> {
        self.registers
            .sp
            .update_value_checked(|sp| {
                mmu.write_byte(*sp, value)?;
                Ok(sp.checked_sub(1))
            })
            .map_err(Error::RegisterError)
    }

    fn push_stack_word(&mut self, value: u16, mmu: &mut MMU) -> Result<(), Error> {
        self.registers.sp.update_value_checked(|sp| Ok(sp.checked_sub(2)))?;
        mmu.write_word(*self.registers.sp.get_value(), value)?;

        Ok(())
    }

    fn pop_stack_byte(&mut self, mmu: &mut MMU) -> Result<u8, Error> {
        unimplemented!()
    }

    fn pop_stack_word(&mut self, mmu: &mut MMU) -> Result<u16, Error> {
        let stack_val = mmu.read_word(*self.registers.sp.get_value())?;
        self.registers
            .sp
            .update_value_checked(|sp| Ok(sp.checked_add(2)))?;

        Ok(stack_val)
    }
}

impl CPU {
    fn increment_pc(&mut self) -> Result<u16, Error> {
        let next = *self.registers.pc.get_value();
        self.registers
            .pc
            .update_value_checked(|pc| Ok(pc.checked_add(1)))
            .map_err(Error::RegisterError)?;

        Ok(next)
    }

    fn fetch(&mut self, mmu: &MMU) -> Result<InstructionData, Error> {
        let pc = self.increment_pc()?;
        let op = mmu.read_byte(pc).map_err(Error::MmuError)?;
        let cb_byte = match op {
            0xCB => Some(mmu.read_byte(pc + 1).map_err(Error::MmuError)?),
            _ => None,
        };

        InstructionData::try_from((op, cb_byte)).map_err(Error::DecodeError)
    }


    fn execute(
        &mut self,
        instruction_data: &InstructionData,
        opcode_data: &[u8; 2],
        mmu: &mut MMU,
    ) -> Result<u8, Error> {
        let result = match instruction_data.mnemonic {
            Mnemonic::LD | Mnemonic::LDHL => self.evaluate_ld(instruction_data, opcode_data, mmu),
            Mnemonic::PUSH | Mnemonic::POP => {
                self.evaluate_stack_op(instruction_data, opcode_data, mmu)
            }
            Mnemonic::ADD
            | Mnemonic::ADC
            | Mnemonic::SUB
            | Mnemonic::SBC
            | Mnemonic::AND
            | Mnemonic::XOR
            | Mnemonic::OR
            | Mnemonic::CP
            | Mnemonic::INC
            | Mnemonic::DEC
            | Mnemonic::DAA
            | Mnemonic::CPL => self.evaluate_alu(instruction_data, opcode_data, mmu),
            Mnemonic::RLCA
            | Mnemonic::RLA
            | Mnemonic::RRCA
            | Mnemonic::RRA
            | Mnemonic::RLC
            | Mnemonic::RL
            | Mnemonic::RRC
            | Mnemonic::RR
            | Mnemonic::SLA
            | Mnemonic::SWAP
            | Mnemonic::SRA
            | Mnemonic::SET
            | Mnemonic::SRL => self.evaluate_bitwise(instruction_data, opcode_data, mmu),
            Mnemonic::CCF
            | Mnemonic::SCF
            | Mnemonic::NOP
            | Mnemonic::HALT
            | Mnemonic::STOP
            | Mnemonic::DI
            | Mnemonic::EI => self.evaluate_control(instruction_data, opcode_data, mmu),
            Mnemonic::JP
            | Mnemonic::JR
            | Mnemonic::CALL
            | Mnemonic::RET
            | Mnemonic::RETI
            | Mnemonic::RST
            | Mnemonic::DB
            | Mnemonic::DW => self.evaluate_branch(instruction_data, opcode_data, mmu),
            Mnemonic::UNIMPLEMENTED => Ok(0),
            _ => Err(Error::UnsupportedOpcode(
                instruction_data.instruction.clone(),
            )),
        }?;
        Ok(result)
    }

    pub fn handle_interrupts(&mut self, mmu: &mut MMU) -> Result<u8, Error> {
        if let Some(interrupt) = mmu.interrupts_enabled()? {
            self.debug.log("INTS", || {
                println!("Handling Interrupt: {:?}", interrupt)
            });

            let isr = interrupt.get_isr_location();

            self.debug.log("INTS", || println!("Jumping to {:X}", isr));

            self.push_stack_word(*self.registers.pc.get_value(), mmu)?;
            self.registers.pc.set_value(isr);
            mmu.write_interrupt_enable_reg(false);
            mmu.set_interrupt_bit(interrupt, false)?;

            return Ok(5)
        }

        Ok(0)
    }

    #[cfg(debug_assertions)]
    pub fn gameboy_doc_debug(&self, mmu: &MMU) {
        self.debug.log("GB_DOC", || {
            let pc_mem = [
                mmu.read_byte(*self.registers.pc.get_value())
                    .map_err(Error::MmuError).unwrap(),
                mmu.read_byte((Wrapping(*self.registers.pc.get_value()) + Wrapping(1)).0)
                    .map_err(Error::MmuError).unwrap(),
                mmu.read_byte((Wrapping(*self.registers.pc.get_value()) + Wrapping(2)).0)
                    .map_err(Error::MmuError).unwrap(),
                mmu.read_byte((Wrapping(*self.registers.pc.get_value()) + Wrapping(3)).0)
                    .map_err(Error::MmuError).unwrap(),

            ];
            println!(
                "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
                self.registers.a.get_value(),
                self.registers.f.get_value(),
                self.registers.b.get_value(),
                self.registers.c.get_value(),
                self.registers.d.get_value(),
                self.registers.e.get_value(),
                self.registers.h.get_value(),
                self.registers.l.get_value(),
                self.registers.sp.get_value(),
                self.registers.pc.get_value(),
                pc_mem[0],
                pc_mem[1],
                pc_mem[2],
                pc_mem[3]
            );
        });
    }

    pub fn new() -> Result<CPU, Error> {
        Ok(CPU {
            registers: Registers::new(),
            debug: CpuDebug::default(),
            halt: false
        })
    }
}
