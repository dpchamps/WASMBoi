use crate::format_byte;
use crate::spec::cartridge_header::{Cartridge, CartridgeError};
use crate::spec::mnemonic::{mnemonic, mnemonic_lookup, MnemonicValue};
use crate::spec::opcode::{instruction_lookup, Instruction};
use crate::spec::register::Register;
use crate::util::byte_ops::hi_lo_combine;

use crate::dasm::decoder::decode;
use std::collections::HashSet;
use std::fmt;

pub mod decode_ld;
pub mod decoder;

#[derive(Debug)]
pub enum DasmError {
    InvalidRom(String),
    DecoderError(&'static str),
    PartialDASM(String),
}

pub struct Disassembler {
    branches: Vec<u16>,
    visited: HashSet<u16>,
    labels: HashSet<u16>,
    buffer: Vec<u8>,
    cartridge: Cartridge,
}

#[derive(Debug)]
pub struct InstructionData {
    pub byte: u8,
    pub size: usize,
    pub instruction: Instruction,
    pub data: Vec<u8>,
}

impl fmt::Display for InstructionData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match decoder::decode(self) {
            Ok(s) => write!(f, "{}", s),
            Err(e) => Err(fmt::Error),
        }
    }
}

impl Disassembler {
    pub fn new(rom: &Vec<u8>) -> Result<Self, DasmError> {
        let cartridge = match Cartridge::new(rom) {
            Ok(cartridge) => cartridge,
            _ => return Err(DasmError::InvalidRom("Could not read cartridge".to_owned())),
        };

        Ok(Disassembler {
            branches: vec![],
            visited: HashSet::new(),
            labels: HashSet::new(),
            buffer: rom.clone(),
            cartridge,
        })
    }

    pub fn get_instruction_data(
        byte: &u8,
        idx: &u16,
        buffer: &[u8],
    ) -> Result<InstructionData, DasmError> {
        let instruction = match instruction_lookup(&byte) {
            Ok(instruction) => instruction,
            _ => Instruction::UNIMPLEMENTED,
        };

        let size = Instruction::get_size(&instruction);
        let data: Vec<u8> = buffer[(*idx as usize)..(*idx as usize) + size].to_vec();

        Ok(InstructionData {
            byte: byte.clone(),
            instruction,
            data,
            size,
        })
    }

    pub fn extract_instruction_data(
        &self,
        byte: &u8,
        idx: usize,
    ) -> Result<InstructionData, DasmError> {
        let instruction = match instruction_lookup(byte) {
            Ok(instruction) => instruction,
            _ => {
                // println!("{}", format!("Failed to disassemble instruction {}. Defaulting to NOP", format_byte!(byte)));
                Instruction::NOP
            }
        };

        let size = Instruction::get_size(&instruction);
        let data: Vec<u8> = self.buffer[idx..idx + size].to_vec();

        Ok(InstructionData {
            byte: byte.clone(),
            instruction,
            data,
            size,
        })
    }
}

fn trace(dasm: &mut Disassembler) -> Result<(Vec<Vec<String>>), DasmError> {
    let mut paths: Vec<Vec<String>> = Vec::new();

    while dasm.branches.len() > 0 {
        let mut path: Vec<String> = Vec::new();

        let head = match dasm.branches.pop() {
            Some(result) => result,
            _ => {
                return Err(DasmError::InvalidRom(
                    "Failed to pop a branch. Something terrible happened".to_owned(),
                ))
            }
        };

        let mut idx: usize = head as usize;
        println!("Head {}", idx);

        'inner: loop {
            // println!("Top Idx: {}. Buffer Length: {}", idx, dasm.buffer.len());
            // println!(
            //     "Label Exists: {}, Has Been Visited: {}",
            //     dasm.labels.contains(&(idx as u16)),
            //     dasm.visited.contains(&(idx as u16))
            // );
            if idx > dasm.buffer.len()
                || (dasm.labels.contains(&(idx as u16)) && dasm.visited.contains(&(idx as u16)))
            {
                break 'inner;
            }

            dasm.visited.insert(idx as u16);

            let instruction_data = dasm.extract_instruction_data(&dasm.buffer[idx], idx)?;
            let decoded = match decoder::decode(&instruction_data) {
                Ok(decoded) => decoded,
                // Err(message) => return Err(DasmError::DecoderError(message)),
                Err(message) => message.to_string(),
            };
            path.push(decoded);

            println!("idx: {}, {:?}", idx, instruction_data);

            idx += 1 + instruction_data.size;

            if Instruction::is_branch(&instruction_data.instruction) {
                let next_address =
                    hi_lo_combine(instruction_data.data[1], instruction_data.data[0]);
                println!(
                    "Next Address: {}, from: {}, {}",
                    next_address,
                    instruction_data.byte,
                    dasm.buffer[idx + 1]
                );
                dasm.labels.insert(next_address);

                if Instruction::is_call(&instruction_data.instruction) {
                    dasm.branches.push(idx as u16);
                }

                idx = next_address as usize;
            }

            if Instruction::is_return(&instruction_data.instruction) {
                break 'inner;
            };
        }
        println!("Finished inner loop {}", dasm.branches.len());
        paths.push(path);
    }
    println!("Finished outer loop");

    Ok(paths)
}

pub fn disassemble(rom: &Vec<u8>) -> Result<String, DasmError> {
    let mut disassembly: String = String::from("");
    let mut dasm = Disassembler::new(rom)?;

    dasm.branches.push(dasm.cartridge.start_address);

    let paths = trace(&mut dasm)?;
    println!("Done tracing");

    for path in paths {
        for instruction in path {
            disassembly.push_str("\n");
            disassembly.push_str(&instruction);
        }
        disassembly.push_str("\n;--\n");
    }

    println!("Done everything");

    Ok(disassembly)
}
