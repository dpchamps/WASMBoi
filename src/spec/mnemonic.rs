#![allow(non_camel_case_types)]

pub type MnemonicValue = &'static str;

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

pub enum Instruction {
//8 BIT LOAD
 LD_RR,
 LD_RN ,
 LD_RHL ,
 LD_HLR ,
 LD_HLN ,
 LD_ABC ,
 LD_ADE ,
 LD_AN ,
 LD_ANN ,
 LD_BCA ,
 LD_DEA ,
 LD_NA ,
 LD_NNA ,
 LD_AFF00C ,
 LD_FF00CA ,
 LD_HLIA ,
 LD_AHLI ,
 LD_HLDA ,
 LD_AHLD ,

//16 BIT LOAD
 LD_RRNN ,
 LD_SPHL ,
 PUSH_RR ,
 POP_RR ,

//8 BIT ARITHMETIC / LOGIC
 ADD_AR ,
 ADD_AN ,
 ADD_AHL ,
 ADC_AR ,
 ADC_AN ,
 ADC_AHL ,
 SUB_R ,
 SUB_N ,
 SUB_HL ,
 SBC_AR ,
 SBC_AN ,
 SBC_AHL ,
 AND_R ,
 AND_N ,
 AND_HL ,
 XOR_R ,
 XOR_N ,
 XOR_HL ,
 OR_R ,
 OR_N ,
 OR_HL ,
 CP_R ,
 CP_N ,
 CP_HL ,
 INC_R ,
 INC_HL ,
 DEC_R ,
 DEC_HL ,
 DAA ,
 CPL ,

//16 BIT ARITHMETIC / LOGICAL
 ADD_HLRR ,
 ADD_SPN ,
 INC_RR ,
 DEC_RR ,
 LD_SPDD ,
 LDHL ,

//ROTATE AND SHIFT
 RLCA ,
 RLA ,
 RRCA ,
 RRA ,
 RLC_R ,
 RLC_HL ,
 RL_R ,
 RL_HL ,
 RRC_R ,
 RRC_HL ,
 RR_R ,
 RR_HL ,
 SLA_R ,
 SLA_HL ,
 SWAP_R ,
 SWAP_HL ,
 SRA_R ,
 SRA_HL ,
 SRL_R ,
 SRL_HL ,

//BITWISE
 BIT_NR ,
 BIT_NHL ,
 SET_NR ,
 SET_NHL ,
 RES_NR ,
 RES_NHL ,

//CPU CONTROL
 CCF ,
 SCF ,
 NOP ,
 HALT ,
 STOP ,
 DI ,
 EI ,

//JUMP
 JP_NN ,
 JP_HL ,
 JP_FNN ,
 JR_PCDD ,
 JR_FPCDD ,
 CALL_NN ,
 CALL_FNN ,
 RET,
 RET_F ,
 RETI ,
 RST ,
}

pub fn mnemonic_lookup(instruction : Instruction) -> MnemonicValue {
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
  Instruction::LD_SPHL => LD,

  Instruction::PUSH_RR => PUSH,
  Instruction::POP_RR => POP,

  Instruction::ADD_AR |
  Instruction::ADD_AN |
  Instruction::ADD_AHL => ADD,

  Instruction::ADC_AR |
  Instruction::ADC_AN |
  Instruction::ADC_AHL => ADC,

  Instruction::SUB_R |
  Instruction::SUB_N |
  Instruction::SUB_HL => SUB,

  Instruction::SBC_AR |
  Instruction::SBC_AN |
  Instruction::SBC_AHL => SBC,

  Instruction::AND_R |
  Instruction::AND_N |
  Instruction::AND_HL => AND,

  Instruction::XOR_R |
  Instruction::XOR_N |
  Instruction::XOR_HL => XOR,

  Instruction::OR_R |
  Instruction::OR_N |
  Instruction::OR_HL => OR,

  Instruction::CP_R |
  Instruction::CP_N |
  Instruction::CP_HL => CP,

  Instruction::INC_R |
  Instruction::INC_HL => INC,

  Instruction::DEC_R |
  Instruction::DEC_HL => DEC,

  Instruction::DAA => DAA,

  Instruction::CPL => CPL,

  Instruction::ADD_HLRR |
  Instruction::ADD_SPN => ADD,

  Instruction::INC_RR => INC,

  Instruction::DEC_RR => DEC,

  Instruction::LD_SPDD => LD,

  Instruction::LDHL => LDHL,

  Instruction::RLCA => RLCA,

  Instruction::RLA => RLA,

  Instruction::RRCA => RRCA,

  Instruction::RRA => RRA,

  Instruction::RLC_R |
  Instruction::RLC_HL => RLC,

  Instruction::RL_R |
  Instruction::RL_HL => RL,

  Instruction::RRC_R |
  Instruction::RRC_HL => RRC,

  Instruction::RR_R |
  Instruction::RR_HL  => RR,

  Instruction::SLA_R |
  Instruction::SLA_HL => SLA,

  Instruction::SWAP_R |
  Instruction::SWAP_HL => SWAP,

  Instruction::SRA_R |
  Instruction::SRA_HL => SRA,

  Instruction::SRL_R |
  Instruction::SRL_HL => SRL,

  Instruction::BIT_NR |
  Instruction::BIT_NHL => BIT,

  Instruction::SET_NR |
  Instruction::SET_NHL => SET,

  Instruction::RES_NR |
  Instruction::RES_NHL => RES,

  Instruction::CCF => CCF,

  Instruction::SCF => SCF,

  Instruction::NOP  => NOP,

  Instruction::HALT => HALT,

  Instruction::STOP => STOP,

  Instruction::DI => DI,

  Instruction::EI => EI,

  Instruction::JP_NN |
  Instruction::JP_HL |
  Instruction::JP_FNN => JP,

  Instruction::JR_PCDD |
  Instruction::JR_FPCDD => JR,

  Instruction::CALL_NN |
  Instruction::CALL_FNN => CALL,

  Instruction::RET|
  Instruction::RET_F => RET,

  Instruction::RETI => RETI,

  Instruction::RST => RST
 }
}