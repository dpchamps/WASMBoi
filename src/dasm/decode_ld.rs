use crate::{format_args, format_byte, format_instruction};

use crate::dasm::{InstructionData};
use crate::dasm::decoder::{format_data, extract_ld_register, extract_register};

use crate::util::byte_ops::{extract_lhs, extract_rhs};

use crate::spec::opcode::Instruction;
use crate::spec::mnemonic::mnemonic::LD;
use crate::spec::register::decoded_register;

pub fn decode(instruction_data : &InstructionData) -> Result<String, &'static str> {
    match instruction_data.instruction {
        Instruction::LD_RR => {
            let lhs_register = extract_register(extract_lhs(instruction_data.byte))?;
            let rhs_register = extract_register(extract_rhs(instruction_data.byte))?;

            Ok(format_instruction!(LD, lhs_register ,rhs_register))
        },
        Instruction::LD_RN => {
            let lhs_register = extract_register(extract_lhs(instruction_data.byte))?;
            let rhs = format_byte!(instruction_data.data[0]);

            Ok(format_instruction!(LD, lhs_register, rhs))
        },
        Instruction::LD_RHL => {
            let lhs_register = extract_register(extract_lhs(instruction_data.byte))?;

            Ok(format_instruction!(LD, lhs_register, decoded_register::HL))
        },
        Instruction::LD_HLR => {
            let rhs_register = extract_register(extract_rhs(instruction_data.byte))?;

            Ok(format_instruction!(LD, decoded_register::HL, rhs_register))
        },
        Instruction::LD_HLN => {
            Ok(format_instruction!(LD, decoded_register::HL, format_byte!(instruction_data.data[0])))
        },
        Instruction::LD_ABC => {
            Ok(format_instruction!(LD, decoded_register::A, decoded_register::BC))
        },
        Instruction::LD_ADE => {
            Ok(format_instruction!(LD, decoded_register::A, decoded_register::DE))
        },
        Instruction::LD_AN => {
            let arg = format_byte!(instruction_data.data[0]);

            Ok(format_instruction!(LD, decoded_register::A, arg))
        },
        Instruction::LD_ANN => {
            let data = format_data(&instruction_data.data);

            Ok(format_instruction!(LD, decoded_register::A, data))
        },
        Instruction::LD_BCA => {
            Ok(format_instruction!(LD, decoded_register::BC, decoded_register::A))
        },
        Instruction::LD_DEA => {
            Ok(format_instruction!(LD, decoded_register::DE, decoded_register::A))
        },
        Instruction::LD_NA => {
            let arg = format_byte!(instruction_data.data[0]);

            Ok(format_instruction!(LD, arg, decoded_register::A))
        },
        Instruction::LD_NNA => {
            let data = format_data(&instruction_data.data);

            Ok(format_instruction!(LD, data, decoded_register::A))
        },
        Instruction::LD_AFF00C => {
            Ok(format_instruction!(LD, decoded_register::A, decoded_register::FF00C))
        },
        Instruction::LD_FF00CA => {
            Ok(format_instruction!(LD, decoded_register::FF00C, decoded_register::A))
        },
        Instruction::LD_HLIA => {
            Ok(format_instruction!(LD, decoded_register::HLI, decoded_register::A))
        },
        Instruction::LD_AHLI => {
            Ok(format_instruction!(LD, decoded_register::A, decoded_register::HLI))
        },
        Instruction::LD_HLDA => {
            Ok(format_instruction!(LD, decoded_register::HLD, decoded_register::A))
        },
        Instruction::LD_AHLD => {
            Ok(format_instruction!(LD, decoded_register::A, decoded_register::HLD))
        },
        Instruction::LD_RRNN => {
            let lhs_register = extract_ld_register(extract_lhs(instruction_data.byte))?;
            let data = format_data(&instruction_data.data);

            Ok(format_instruction!(LD, lhs_register, data))
        },
        Instruction::LD_SPHL => {
            Ok(format_instruction!(LD, decoded_register::SP, decoded_register::HL))
        },
        Instruction::LDHL => {
            let arg = format_byte!(instruction_data.data[0]);

            Ok(format_instruction!(LD, decoded_register::SP, arg))
        }
        _ => Err("Received an invalid LD instruction")
    }
}

#[cfg(test)]
mod decode_ld_test{
    use crate::dasm::InstructionData;
    use crate::spec::opcode::{Instruction, instruction_lookup};
    use crate::dasm::decode_ld::decode;

    fn create_instruction_data(byte : u8, data : Vec<u8>) -> InstructionData {
        let instruction = instruction_lookup(&byte).ok().unwrap();

        InstructionData{
            byte,
            data : data.clone(),
            size : data.len(),
            instruction
        }
    }

    fn create_and_decode(byte : u8, data : Vec<u8>) -> String {
        decode(
            &create_instruction_data(byte, data)
        ).ok().unwrap()
    }

    #[test]
    fn ld_rr_test() {
        assert_eq!(
            create_and_decode(0x7F, vec![]),
            "LD\tA, A"
        );

        assert_eq!(
            create_and_decode(0x36, vec![0xC]),
            "LD\t(HL), Ch"
        );
    }

    #[test]
    fn ld_an_test(){
        assert_eq!(
            create_and_decode(0x3E, vec![0xDA]),
            "LD\tA, DAh"
        );

        assert_eq!(
            create_and_decode(0xFA, vec![0xDA, 0xCC]),
            "LD\tA, (DACC)h"
        )
    }

    #[test]
    fn ld_na_test(){
        assert_eq!(
            create_and_decode(0x6F, vec![]),
            "LD\tL, A"
        );

        assert_eq!(
            create_and_decode(0xEA, vec![0xED, 0xAF]),
            "LD\t(EDAF)h, A"
        )
    }
}