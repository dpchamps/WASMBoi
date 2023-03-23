#![allow(non_camel_case_types)]

use crate::spec::opcode::Instruction;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Mnemonic {
    LD,
    LDHL,

    PUSH,
    POP,

    ADD,
    ADC,
    SUB,
    SBC,
    AND,
    XOR,
    OR,
    CP,
    INC,
    DEC,
    DAA,
    CPL,

    RLCA,
    RLA,
    RRCA,
    RRA,
    RLC,
    RL,
    RRC,
    RR,
    SLA,
    SWAP,
    SRA,
    SRL,

    BIT,
    SET,
    RES,

    CCF,
    SCF,
    NOP,
    HALT,
    STOP,
    DI,
    EI,

    JP,
    JR,
    CALL,
    RET,
    RETI,
    RST,
    DB,
    DW,

    UNIMPLEMENTED,
}

impl From<&Mnemonic> for String {
    fn from(mnemonic: &Mnemonic) -> Self {
        let str = match mnemonic {
            Mnemonic::LD => "LD",
            Mnemonic::LDHL => "LDHL",
            Mnemonic::PUSH => "PUSH",
            Mnemonic::POP => "POP",
            Mnemonic::ADD => "ADD",
            Mnemonic::ADC => "ADC",
            Mnemonic::SUB => "SUB",
            Mnemonic::SBC => "SBC",
            Mnemonic::AND => "AND",
            Mnemonic::XOR => "XOR",
            Mnemonic::OR => "OR",
            Mnemonic::CP => "CP",
            Mnemonic::INC => "INC",
            Mnemonic::DEC => "DEC",
            Mnemonic::DAA => "DAA",
            Mnemonic::CPL => "CPL",
            Mnemonic::RLCA => "RLCA",
            Mnemonic::RLA => "RLA",
            Mnemonic::RRCA => "RRCA",
            Mnemonic::RRA => "RRA",
            Mnemonic::RLC => "RLC",
            Mnemonic::RL => "RL",
            Mnemonic::RRC => "RRC",
            Mnemonic::RR => "RR",
            Mnemonic::SLA => "SLA",
            Mnemonic::SWAP => "SWAP",
            Mnemonic::SRA => "SRA",
            Mnemonic::SRL => "SRL",
            Mnemonic::BIT => "BIT",
            Mnemonic::SET => "SET",
            Mnemonic::RES => "RES",
            Mnemonic::CCF => "CCF",
            Mnemonic::SCF => "SCF",
            Mnemonic::NOP => "NOP",
            Mnemonic::HALT => "HALT",
            Mnemonic::STOP => "STOP",
            Mnemonic::DI => "DI",
            Mnemonic::EI => "EI",
            Mnemonic::JP => "JP",
            Mnemonic::JR => "JR",
            Mnemonic::CALL => "CALL",
            Mnemonic::RET => "RET",
            Mnemonic::RETI => "RETI",
            Mnemonic::RST => "RST",
            Mnemonic::DB => "DB",
            Mnemonic::DW => "DW",
            Mnemonic::UNIMPLEMENTED => "UNIMPLEMENTED",
        };

        String::from(str)
    }
}

impl Display for Mnemonic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl From<&Instruction> for Mnemonic {
    fn from(instruction: &Instruction) -> Self {
        match instruction {
            Instruction::LD_RR
            | Instruction::LD_RN
            | Instruction::LD_RHL
            | Instruction::LD_HLR
            | Instruction::LD_HLN
            | Instruction::LD_ABC
            | Instruction::LD_ADE
            | Instruction::LD_AN
            | Instruction::LD_ANN
            | Instruction::LD_BCA
            | Instruction::LD_DEA
            | Instruction::LD_NA
            | Instruction::LD_NNA
            | Instruction::LD_AFF00C
            | Instruction::LD_FF00CA
            | Instruction::LD_HLIA
            | Instruction::LD_AHLI
            | Instruction::LD_HLDA
            | Instruction::LD_AHLD
            | Instruction::LD_RRNN
            | Instruction::LD_SPHL => Mnemonic::LD,

            Instruction::PUSH_RR => Mnemonic::PUSH,
            Instruction::POP_RR => Mnemonic::POP,

            Instruction::ADD_AR | Instruction::ADD_AN | Instruction::ADD_AHL => Mnemonic::ADD,

            Instruction::ADC_AR | Instruction::ADC_AN | Instruction::ADC_AHL => Mnemonic::ADC,

            Instruction::SUB_R | Instruction::SUB_N | Instruction::SUB_HL => Mnemonic::SUB,

            Instruction::SBC_AR | Instruction::SBC_AN | Instruction::SBC_AHL => Mnemonic::SBC,

            Instruction::AND_R | Instruction::AND_N | Instruction::AND_HL => Mnemonic::AND,

            Instruction::XOR_R | Instruction::XOR_N | Instruction::XOR_HL => Mnemonic::XOR,

            Instruction::OR_R | Instruction::OR_N | Instruction::OR_HL => Mnemonic::OR,

            Instruction::CP_R | Instruction::CP_N | Instruction::CP_HL => Mnemonic::CP,

            Instruction::INC_R | Instruction::INC_HL => Mnemonic::INC,

            Instruction::DEC_R | Instruction::DEC_HL => Mnemonic::DEC,

            Instruction::DAA => Mnemonic::DAA,

            Instruction::CPL => Mnemonic::CPL,

            Instruction::ADD_HLRR | Instruction::ADD_SPN => Mnemonic::ADD,

            Instruction::INC_RR => Mnemonic::INC,

            Instruction::DEC_RR => Mnemonic::DEC,

            Instruction::LD_SPDD => Mnemonic::LD,

            Instruction::LDHL => Mnemonic::LDHL,

            Instruction::RLCA => Mnemonic::RLCA,

            Instruction::RLA => Mnemonic::RLA,

            Instruction::RRCA => Mnemonic::RRCA,

            Instruction::RRA => Mnemonic::RRA,

            Instruction::RLC_R | Instruction::RLC_HL => Mnemonic::RLC,

            Instruction::RL_R | Instruction::RL_HL => Mnemonic::RL,

            Instruction::RRC_R | Instruction::RRC_HL => Mnemonic::RRC,

            Instruction::RR_R | Instruction::RR_HL => Mnemonic::RR,

            Instruction::SLA_R | Instruction::SLA_HL => Mnemonic::SLA,

            Instruction::SWAP_R | Instruction::SWAP_HL => Mnemonic::SWAP,

            Instruction::SRA_R | Instruction::SRA_HL => Mnemonic::SRA,

            Instruction::SRL_R | Instruction::SRL_HL => Mnemonic::SRL,

            Instruction::BIT_NR | Instruction::BIT_NHL => Mnemonic::BIT,

            Instruction::SET_NR | Instruction::SET_NHL => Mnemonic::SET,

            Instruction::RES_NR | Instruction::RES_NHL => Mnemonic::RES,

            Instruction::CCF => Mnemonic::CCF,

            Instruction::SCF => Mnemonic::SCF,

            Instruction::NOP => Mnemonic::NOP,

            Instruction::HALT => Mnemonic::HALT,

            Instruction::STOP => Mnemonic::STOP,

            Instruction::DI => Mnemonic::DI,

            Instruction::EI => Mnemonic::EI,

            Instruction::JP_NN | Instruction::JP_HL | Instruction::JP_FNN => Mnemonic::JP,

            Instruction::JR_PCDD | Instruction::JR_FPCDD => Mnemonic::JR,

            Instruction::CALL_NN | Instruction::CALL_FNN => Mnemonic::CALL,

            Instruction::RET | Instruction::RET_F => Mnemonic::RET,

            Instruction::RETI => Mnemonic::RETI,

            Instruction::RST => Mnemonic::RST,

            Instruction::UNIMPLEMENTED => Mnemonic::UNIMPLEMENTED,
        }
    }
}
