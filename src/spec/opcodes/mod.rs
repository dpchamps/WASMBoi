use crate::spec::cpu::Error;
use crate::spec::mnemonic::Mnemonic;

mod alu;
mod bitwise;
mod branch;
mod control;
mod ld;
mod stack;

/// TODO:
/// This needs to be a variadic macro, to take any number of expected opcodes
pub fn unexpected_op(received: &Mnemonic, expected: &Mnemonic) -> Error {
    Error::UnexpectedOpcode(format!(
        "Called evaluate_ld with {} but expected type {}. This is an unrecoverable program error",
        received, expected
    ))
}
