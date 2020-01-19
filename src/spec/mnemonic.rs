#![allow(non_camel_case_types)]

use crate::spec::opcode::Instruction;
pub type MnemonicValue = &'static str;

pub mod mnemonic {
 use crate::spec::mnemonic::MnemonicValue;

 pub const LD : MnemonicValue = "LD";
 pub const LDHL : MnemonicValue = "LDHL";
 pub const PUSH : MnemonicValue = "PUSH";
 pub const POP : MnemonicValue = "POP";
 pub const ADD : MnemonicValue = "ADD";
 pub const ADC : MnemonicValue = "ADC";
 pub const SUB : MnemonicValue = "SUB";
 pub const SBC : MnemonicValue = "SBC";
 pub const AND : MnemonicValue = "AND";
 pub const XOR : MnemonicValue = "XOR";
 pub const OR : MnemonicValue = "OR";
 pub const CP : MnemonicValue = "CP";
 pub const INC : MnemonicValue = "INC";
 pub const DEC : MnemonicValue = "DEC";
 pub const DAA : MnemonicValue = "DAA";
 pub const CPL : MnemonicValue = "CPL";
 pub const RLCA : MnemonicValue = "RLCA";
 pub const RLA : MnemonicValue = "RLA";
 pub const RRCA : MnemonicValue = "RRCA";
 pub const RRA : MnemonicValue = "RRA";
 pub const RLC : MnemonicValue = "RLC";
 pub const RL : MnemonicValue = "RL";
 pub const RRC : MnemonicValue = "RRC";
 pub const RR : MnemonicValue = "RR";
 pub const SLA : MnemonicValue = "SLA";
 pub const SWAP : MnemonicValue = "SWAP";
 pub const SRA : MnemonicValue = "SRA";
 pub const SRL : MnemonicValue = "SRL";
 pub const BIT : MnemonicValue = "BIT";
 pub const SET : MnemonicValue = "SET";
 pub const RES : MnemonicValue = "RES";
 pub const CCF : MnemonicValue = "CCF";
 pub const SCF : MnemonicValue = "SCF";
 pub const NOP : MnemonicValue = "NOP";
 pub const HALT : MnemonicValue = "HALT";
 pub const STOP : MnemonicValue = "STOP";
 pub const DI : MnemonicValue = "DI";
 pub const EI : MnemonicValue = "EI";
 pub const JP : MnemonicValue = "JP";
 pub const JR : MnemonicValue = "JR";
 pub const CALL : MnemonicValue = "CALL";
 pub const RET : MnemonicValue = "RET";
 pub const RETI : MnemonicValue = "RETI";
 pub const RST : MnemonicValue = "RST";
 //Data mnemonics; write byte and write word (borrowed from 8080 spec) these mnemonics are used
// for unreachable pieces of code
 pub const DB : MnemonicValue = "DB";
 pub const DW : MnemonicValue = "DW";
}

pub fn mnemonic_lookup(instruction : &Instruction) -> MnemonicValue {
 match instruction {
  Instruction::LD_RR|
  Instruction::LD_RN |
  Instruction::LD_RHL |
  Instruction::LD_HLR |
  Instruction::LD_HLN |
  Instruction::LD_ABC |
  Instruction::LD_ADE |
  Instruction::LD_AN |
  Instruction::LD_ANN |
  Instruction::LD_BCA |
  Instruction::LD_DEA |
  Instruction::LD_NA |
  Instruction::LD_NNA |
  Instruction::LD_AFF00C |
  Instruction::LD_FF00CA |
  Instruction::LD_HLIA |
  Instruction::LD_AHLI |
  Instruction::LD_HLDA |
  Instruction::LD_AHLD |
  Instruction::LD_RRNN |
  Instruction::LD_SPHL => mnemonic::LD,

  Instruction::PUSH_RR => mnemonic::PUSH,
  Instruction::POP_RR => mnemonic::POP,

  Instruction::ADD_AR |
  Instruction::ADD_AN |
  Instruction::ADD_AHL => mnemonic::ADD,

  Instruction::ADC_AR |
  Instruction::ADC_AN |
  Instruction::ADC_AHL => mnemonic::ADC,

  Instruction::SUB_R |
  Instruction::SUB_N |
  Instruction::SUB_HL => mnemonic::SUB,

  Instruction::SBC_AR |
  Instruction::SBC_AN |
  Instruction::SBC_AHL => mnemonic::SBC,

  Instruction::AND_R |
  Instruction::AND_N |
  Instruction::AND_HL => mnemonic::AND,

  Instruction::XOR_R |
  Instruction::XOR_N |
  Instruction::XOR_HL => mnemonic::XOR,

  Instruction::OR_R |
  Instruction::OR_N |
  Instruction::OR_HL => mnemonic::OR,

  Instruction::CP_R |
  Instruction::CP_N |
  Instruction::CP_HL => mnemonic::CP,

  Instruction::INC_R |
  Instruction::INC_HL => mnemonic::INC,

  Instruction::DEC_R |
  Instruction::DEC_HL => mnemonic::DEC,

  Instruction::DAA => mnemonic::DAA,

  Instruction::CPL => mnemonic::CPL,

  Instruction::ADD_HLRR |
  Instruction::ADD_SPN => mnemonic::ADD,

  Instruction::INC_RR => mnemonic::INC,

  Instruction::DEC_RR => mnemonic::DEC,

  Instruction::LD_SPDD => mnemonic::LD,

  Instruction::LDHL => mnemonic::LDHL,

  Instruction::RLCA => mnemonic::RLCA,

  Instruction::RLA => mnemonic::RLA,

  Instruction::RRCA => mnemonic::RRCA,

  Instruction::RRA => mnemonic::RRA,

  Instruction::RLC_R |
  Instruction::RLC_HL => mnemonic::RLC,

  Instruction::RL_R |
  Instruction::RL_HL => mnemonic::RL,

  Instruction::RRC_R |
  Instruction::RRC_HL => mnemonic::RRC,

  Instruction::RR_R |
  Instruction::RR_HL  => mnemonic::RR,

  Instruction::SLA_R |
  Instruction::SLA_HL => mnemonic::SLA,

  Instruction::SWAP_R |
  Instruction::SWAP_HL => mnemonic::SWAP,

  Instruction::SRA_R |
  Instruction::SRA_HL => mnemonic::SRA,

  Instruction::SRL_R |
  Instruction::SRL_HL => mnemonic::SRL,

  Instruction::BIT_NR |
  Instruction::BIT_NHL => mnemonic::BIT,

  Instruction::SET_NR |
  Instruction::SET_NHL => mnemonic::SET,

  Instruction::RES_NR |
  Instruction::RES_NHL => mnemonic::RES,

  Instruction::CCF => mnemonic::CCF,

  Instruction::SCF => mnemonic::SCF,

  Instruction::NOP  => mnemonic::NOP,

  Instruction::HALT => mnemonic::HALT,

  Instruction::STOP => mnemonic::STOP,

  Instruction::DI => mnemonic::DI,

  Instruction::EI => mnemonic::EI,

  Instruction::JP_NN |
  Instruction::JP_HL |
  Instruction::JP_FNN => mnemonic::JP,

  Instruction::JR_PCDD |
  Instruction::JR_FPCDD => mnemonic::JR,

  Instruction::CALL_NN |
  Instruction::CALL_FNN => mnemonic::CALL,

  Instruction::RET|
  Instruction::RET_F => mnemonic::RET,

  Instruction::RETI => mnemonic::RETI,

  Instruction::RST => mnemonic::RST
 }
}