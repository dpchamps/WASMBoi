use crate::dasm::InstructionData;
use crate::spec::mnemonic::{mnemonic, mnemonic_lookup};
use crate::spec::opcode::Instruction;
use crate::spec::opcode::CB_PREFIX;
use crate::spec::register::decoded_register::HLI;
use crate::spec::register::{decoded_register, Register};
use crate::util::byte_ops::{extract_lhs, extract_rhs};

use crate::dasm::decode_ld;

#[macro_export]
macro_rules! format_args {
    ($arg:expr) => (format!("{}", $arg));
    ($arg:expr, $($args:expr),+) => (format!("{}, {}", $arg, format_args!($($args),+)));
}

#[macro_export]
macro_rules! format_instruction{
    ($mnemonic:expr) => (format!("{}", $mnemonic));
    ($mnemonic:expr, $($data:expr),+) => (format!("{}\t{}", $mnemonic, format_args!($($data),+)));
}

#[macro_export]
macro_rules! format_byte {
    ($byte:expr) => {
        format!("{:X}h", $byte)
    };
}

pub fn format_data(data: &[u8]) -> String {
    let byte_strings: Vec<String> = data
        .clone()
        .into_iter()
        .map(|x| format!("{:X}", x))
        .collect();
    let byte_string = byte_strings.concat();

    if data.len() > 1 {
        return format!("({})h", byte_string);
    } else {
        return format!("(FF{})", byte_string);
    }
}

pub fn extract_register(register_code: u8) -> Result<&'static str, &'static str> {
    match Register::lookup_register(register_code) {
        Ok(register) => Ok(register),
        _ => Err("Unable to lookup register"),
    }
}

pub fn extract_ld_register(register_code: u8) -> Result<&'static str, &'static str> {
    match Register::lookup_ld_register(register_code) {
        Ok(register) => Ok(register),
        _ => Err("Unable to lookup register"),
    }
}

pub fn decode(instruction_data: &InstructionData) -> Result<String, &'static str> {
    let mnemonic_value = mnemonic_lookup(&instruction_data.instruction);

    match mnemonic_value {
        mnemonic::LD => decode_ld::decode(instruction_data),
        _ => return Err(mnemonic_value),
    }
}

#[cfg(test)]
mod decoder_test {
    use crate::dasm::decoder::format_data;
    use crate::spec::mnemonic::{mnemonic, mnemonic_lookup};

    #[test]
    fn format_instruction() {
        assert_eq!(format_instruction!(mnemonic::LD), "LD")
    }

    #[test]
    fn format_instructions() {
        assert_eq!(format_instruction!(mnemonic::LD, 10), "LD\t10")
    }

    #[test]
    fn format_instructions_multiple() {
        assert_eq!(
            format_instruction!(mnemonic::LD, 10, 11, 12),
            "LD\t10, 11, 12"
        )
    }

    #[test]
    fn format_byte() {
        assert_eq!(format_byte!(10), "Ah")
    }

    #[test]
    fn format_data_single() {
        let data = [0xC];
        assert_eq!(format_data(&data), "(FFC)")
    }

    #[test]
    fn format_data_multiple() {
        let data = [0xC, 0xA];
        assert_eq!(format_data(&data), "(CA)h")
    }
}
