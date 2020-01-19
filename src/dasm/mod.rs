use crate::spec::mnemonic::{mnemonic_lookup, MnemonicValue, mnemonic};
use crate::spec::opcode::{Instruction, instruction_lookup};
use crate::spec::register::{
    Register,
    lookup_ld_register,
    lookup_stack_op_register
};
use crate::spec::cartridge_header::{Cartridge, CartridgeError};
use std::collections::HashSet;
use crate::util::byte_ops::hi_lo_combine;

pub mod decoder;
pub mod decode_ld;

pub enum DasmError {
    InvalidRom,
    DecoderError(&'static str)
}

pub struct Disassembler {
    branches : Vec<u16>,
    visited : HashSet<u16>,
    labels : HashSet<u16>,
    buffer : Vec<u8>,
    cartridge : Cartridge
}

pub struct InstructionData {
    byte : u8,
    size: usize,
    instruction : Instruction,
    data : Vec<u8>
}

impl Disassembler {
    pub fn new(rom : &Vec<u8>) -> Result<Self, DasmError> {
        let cartridge = match Cartridge::new(rom) {
            Ok(cartridge) => cartridge,
            _ => return Err(DasmError::InvalidRom)
        };

        Ok(Disassembler {
            branches : vec![],
            visited : HashSet::new(),
            labels : HashSet::new(),
            buffer : rom.clone(),
            cartridge
        })
    }

    pub fn extract_instruction_data(&self, byte : &u8, idx : usize) -> Result<InstructionData, DasmError> {
        let instruction = match instruction_lookup(byte){
            Ok(instruction) => instruction,
            _ => return Err(DasmError::InvalidRom)
        };

        let size = Instruction::get_size(&instruction);
        let data : Vec<u8> = self.buffer[idx..idx+size].to_vec();

        Ok(InstructionData{
            byte : byte.clone(),
            instruction,
            data,
            size
        })
    }
}

fn extract_instruction_data(byte : &u8, buffer : &Vec<u8>, idx : usize) -> Result<InstructionData, DasmError> {
    let instruction = match instruction_lookup(byte){
        Ok(instruction) => instruction,
        _ => return Err(DasmError::InvalidRom)
    };

    let size = Instruction::get_size(&instruction);
    let data : Vec<u8> = buffer[idx..idx+size].to_vec();

    Ok(InstructionData{
        byte : byte.clone(),
        instruction,
        data,
        size
    })
}

fn trace(dasm : &mut Disassembler) -> Result<(), DasmError>{
    while dasm.branches.len() > 0 {
        let head =  match dasm.branches.pop(){
            Some(result) => result,
            _ => return Err(DasmError::InvalidRom)
        };

        let mut idx : usize = head as usize;

        loop{
            if idx > dasm.buffer.len() || dasm.labels.contains(&(idx as u16)){
                break;
            }

            dasm.visited.insert(idx as u16);

            let instruction_data = dasm.extract_instruction_data(&dasm.buffer[idx], idx)?;

            idx += 1 + instruction_data.size;

            if Instruction::is_branch(&instruction_data.instruction) {
                let next_address = hi_lo_combine(instruction_data.byte, dasm.buffer[idx+1]);
                dasm.labels.insert(next_address);

                if Instruction::is_call(&instruction_data.instruction) {
                    dasm.branches.push(idx as u16);
                }

                idx = next_address as usize;
            }

            if Instruction::is_return(&instruction_data.instruction) {
                break;
            };
        }
    }

    Ok(())
}

pub fn disassemble(rom : &Vec<u8>) -> Result<String, DasmError>{
    let mut disassembly : String = String::from("");
    let mut dasm = Disassembler::new(rom)?;

    dasm.branches.push(dasm.cartridge.start_address);

    trace(&mut dasm);

    for (idx, byte) in dasm.buffer.clone().into_iter().enumerate() {
        let instruction_data = dasm.extract_instruction_data(&byte,  idx)?;

        let decoded = match decoder::decode(&instruction_data) {
            Ok(decoded) => decoded,
            Err(message) => return Err(DasmError::DecoderError(message))
        };

        disassembly = format!("{}\n{}", disassembly, decoded);
    }

    Ok(disassembly)
}