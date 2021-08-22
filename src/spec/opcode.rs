#![allow(non_camel_case_types)]

pub const CB_PREFIX: u8 = 0xCB;

pub enum OpcodeError {
    InvalidOpcodeInput,
}

type OpcodeLookupResult = Result<Instruction, OpcodeError>;

#[derive(Debug)]
pub enum Instruction {
    //8 BIT LOAD
    LD_RR,
    LD_RN,
    LD_RHL,
    LD_HLR,
    LD_HLN,
    LD_ABC,
    LD_ADE,
    LD_AN,
    LD_ANN,
    LD_BCA,
    LD_DEA,
    LD_NA,
    LD_NNA,
    LD_AFF00C,
    LD_FF00CA,
    LD_HLIA,
    LD_AHLI,
    LD_HLDA,
    LD_AHLD,

    //16 BIT LOAD
    LD_RRNN,
    LD_SPHL,
    PUSH_RR,
    POP_RR,

    //8 BIT ARITHMETIC / LOGIC
    ADD_AR,
    ADD_AN,
    ADD_AHL,
    ADC_AR,
    ADC_AN,
    ADC_AHL,
    SUB_R,
    SUB_N,
    SUB_HL,
    SBC_AR,
    SBC_AN,
    SBC_AHL,
    AND_R,
    AND_N,
    AND_HL,
    XOR_R,
    XOR_N,
    XOR_HL,
    OR_R,
    OR_N,
    OR_HL,
    CP_R,
    CP_N,
    CP_HL,
    INC_R,
    INC_HL,
    DEC_R,
    DEC_HL,
    DAA,
    CPL,

    //16 BIT ARITHMETIC / LOGICAL
    ADD_HLRR,
    ADD_SPN,
    INC_RR,
    DEC_RR,
    LD_SPDD,
    LDHL,

    //ROTATE AND SHIFT
    RLCA,
    RLA,
    RRCA,
    RRA,
    RLC_R,
    RLC_HL,
    RL_R,
    RL_HL,
    RRC_R,
    RRC_HL,
    RR_R,
    RR_HL,
    SLA_R,
    SLA_HL,
    SWAP_R,
    SWAP_HL,
    SRA_R,
    SRA_HL,
    SRL_R,
    SRL_HL,

    //BITWISE
    BIT_NR,
    BIT_NHL,
    SET_NR,
    SET_NHL,
    RES_NR,
    RES_NHL,

    //CPU CONTROL
    CCF,
    SCF,
    NOP,
    HALT,
    STOP,
    DI,
    EI,

    //JUMP
    JP_NN,
    JP_HL,
    JP_FNN,
    JR_PCDD,
    JR_FPCDD,
    CALL_NN,
    CALL_FNN,
    RET,
    RET_F,
    RETI,
    RST,

    //Debug
    UNIMPLEMENTED
}

impl Instruction {
    pub fn is_branch(instruction: &Instruction) -> bool {
        match instruction {
            Instruction::JP_NN
            | Instruction::JP_HL
            | Instruction::JP_FNN
            | Instruction::CALL_FNN
            | Instruction::CALL_NN => true,
            _ => false,
        }
    }

    pub fn is_return(instruction: &Instruction) -> bool {
        match instruction {
            Instruction::RET => true,
            _ => false,
        }
    }

    pub fn is_call(instruction: &Instruction) -> bool {
        match instruction {
            Instruction::CALL_FNN | Instruction::CALL_NN => true,
            _ => false,
        }
    }

    pub fn get_size(instruction: &Instruction) -> usize {
        match instruction {
            Instruction::ADD_AR
            | Instruction::ADD_AHL
            | Instruction::ADD_HLRR
            | Instruction::AND_HL
            | Instruction::AND_R
            | Instruction::CP_HL
            | Instruction::CP_R
            | Instruction::OR_HL
            | Instruction::OR_R
            | Instruction::SUB_HL
            | Instruction::SUB_R
            | Instruction::XOR_HL
            | Instruction::XOR_R
            | Instruction::DEC_HL
            | Instruction::DEC_R
            | Instruction::DEC_RR
            | Instruction::INC_HL
            | Instruction::INC_R
            | Instruction::INC_RR
            | Instruction::SBC_AR
            | Instruction::SBC_AHL
            | Instruction::ADC_AHL
            | Instruction::ADC_AR
            | Instruction::RET
            | Instruction::RET_F
            | Instruction::RETI
            | Instruction::RST
            | Instruction::PUSH_RR
            | Instruction::POP_RR
            | Instruction::DAA
            | Instruction::CPL
            | Instruction::NOP
            | Instruction::CCF
            | Instruction::SCF
            | Instruction::DI
            | Instruction::EI
            | Instruction::HALT
            | Instruction::STOP
            | Instruction::JP_HL
            | Instruction::LD_RR
            | Instruction::LD_RHL
            | Instruction::LD_HLR
            | Instruction::LD_ABC
            | Instruction::LD_ADE
            | Instruction::LD_AFF00C
            | Instruction::LD_FF00CA
            | Instruction::LD_AHLI
            | Instruction::LD_AHLD
            | Instruction::LD_BCA
            | Instruction::LD_DEA
            | Instruction::LD_HLIA
            | Instruction::LD_HLDA
            | Instruction::LD_SPHL
            | Instruction::RLCA
            | Instruction::RLA
            | Instruction::RRCA
            | Instruction::UNIMPLEMENTED
            | Instruction::RRA => 0,

            Instruction::ADD_AN
            | Instruction::ADD_SPN
            | Instruction::AND_N
            | Instruction::CP_N
            | Instruction::OR_N
            | Instruction::SUB_N
            | Instruction::XOR_N
            | Instruction::SBC_AN
            | Instruction::ADC_AN
            | Instruction::BIT_NHL
            | Instruction::BIT_NR
            | Instruction::SET_NHL
            | Instruction::SET_NR
            | Instruction::RES_NHL
            | Instruction::RES_NR
            | Instruction::JR_PCDD
            | Instruction::JR_FPCDD
            | Instruction::LD_RN
            | Instruction::LD_HLN
            | Instruction::LD_AN
            | Instruction::LD_NA
            | Instruction::LDHL
            | Instruction::RLC_R
            | Instruction::RLC_HL
            | Instruction::RL_R
            | Instruction::RL_HL
            | Instruction::RRC_R
            | Instruction::RRC_HL
            | Instruction::RR_R
            | Instruction::RR_HL
            | Instruction::SLA_R
            | Instruction::SLA_HL
            | Instruction::SRA_R
            | Instruction::SRA_HL
            | Instruction::SRL_R
            | Instruction::SRL_HL
            | Instruction::SWAP_R
            | Instruction::SWAP_HL => 1,

            Instruction::CALL_NN
            | Instruction::CALL_FNN
            | Instruction::JP_NN
            | Instruction::JP_FNN
            | Instruction::LD_ANN
            | Instruction::LD_NNA
            | Instruction::LD_RRNN
            | Instruction::LD_SPDD => 2,
        }
    }
}

pub fn cb_prefix_instruction_lookup(byte: &u8) -> OpcodeLookupResult {
    match byte {
        0x0 => Ok(Instruction::RLC_R),
        0x1 => Ok(Instruction::RLC_R),
        0x2 => Ok(Instruction::RLC_R),
        0x3 => Ok(Instruction::RLC_R),
        0x4 => Ok(Instruction::RLC_R),
        0x5 => Ok(Instruction::RLC_R),
        0x6 => Ok(Instruction::RLC_HL),
        0x7 => Ok(Instruction::RLC_R),
        0x8 => Ok(Instruction::RRC_R),
        0x9 => Ok(Instruction::RRC_R),
        0xA => Ok(Instruction::RRC_R),
        0xB => Ok(Instruction::RRC_R),
        0xC => Ok(Instruction::RRC_R),
        0xD => Ok(Instruction::RRC_R),
        0xE => Ok(Instruction::RRC_HL),
        0xF => Ok(Instruction::RRC_R),
        0x10 => Ok(Instruction::RL_R),
        0x11 => Ok(Instruction::RL_R),
        0x12 => Ok(Instruction::RL_R),
        0x13 => Ok(Instruction::RL_R),
        0x14 => Ok(Instruction::RL_R),
        0x15 => Ok(Instruction::RL_R),
        0x16 => Ok(Instruction::RL_HL),
        0x17 => Ok(Instruction::RL_R),
        0x18 => Ok(Instruction::RR_R),
        0x19 => Ok(Instruction::RR_R),
        0x1A => Ok(Instruction::RR_R),
        0x1B => Ok(Instruction::RR_R),
        0x1C => Ok(Instruction::RR_R),
        0x1D => Ok(Instruction::RR_R),
        0x1E => Ok(Instruction::RR_HL),
        0x1F => Ok(Instruction::RR_R),
        0x20 => Ok(Instruction::SLA_R),
        0x21 => Ok(Instruction::SLA_R),
        0x22 => Ok(Instruction::SLA_R),
        0x23 => Ok(Instruction::SLA_R),
        0x24 => Ok(Instruction::SLA_R),
        0x25 => Ok(Instruction::SLA_R),
        0x26 => Ok(Instruction::SLA_HL),
        0x27 => Ok(Instruction::SLA_R),
        0x28 => Ok(Instruction::SRA_R),
        0x29 => Ok(Instruction::SRA_R),
        0x2A => Ok(Instruction::SRA_R),
        0x2B => Ok(Instruction::SRA_R),
        0x2C => Ok(Instruction::SRA_R),
        0x2D => Ok(Instruction::SRA_R),
        0x2E => Ok(Instruction::SRA_HL),
        0x2F => Ok(Instruction::SRA_R),
        0x30 => Ok(Instruction::SWAP_R),
        0x31 => Ok(Instruction::SWAP_R),
        0x32 => Ok(Instruction::SWAP_R),
        0x33 => Ok(Instruction::SWAP_R),
        0x34 => Ok(Instruction::SWAP_R),
        0x35 => Ok(Instruction::SWAP_R),
        0x36 => Ok(Instruction::SWAP_HL),
        0x37 => Ok(Instruction::SWAP_R),
        0x38 => Ok(Instruction::SRL_R),
        0x39 => Ok(Instruction::SRL_R),
        0x3A => Ok(Instruction::SRL_R),
        0x3B => Ok(Instruction::SRL_R),
        0x3C => Ok(Instruction::SRL_R),
        0x3D => Ok(Instruction::SRL_R),
        0x3E => Ok(Instruction::SRL_HL),
        0x3F => Ok(Instruction::SRL_R),
        0x40 => Ok(Instruction::BIT_NR),
        0x41 => Ok(Instruction::BIT_NR),
        0x42 => Ok(Instruction::BIT_NR),
        0x43 => Ok(Instruction::BIT_NR),
        0x44 => Ok(Instruction::BIT_NR),
        0x45 => Ok(Instruction::BIT_NR),
        0x46 => Ok(Instruction::BIT_NHL),
        0x47 => Ok(Instruction::BIT_NR),
        0x48 => Ok(Instruction::BIT_NR),
        0x49 => Ok(Instruction::BIT_NR),
        0x4A => Ok(Instruction::BIT_NR),
        0x4B => Ok(Instruction::BIT_NR),
        0x4C => Ok(Instruction::BIT_NR),
        0x4D => Ok(Instruction::BIT_NR),
        0x4E => Ok(Instruction::BIT_NHL),
        0x4F => Ok(Instruction::BIT_NR),
        0x50 => Ok(Instruction::BIT_NR),
        0x51 => Ok(Instruction::BIT_NR),
        0x52 => Ok(Instruction::BIT_NR),
        0x53 => Ok(Instruction::BIT_NR),
        0x54 => Ok(Instruction::BIT_NR),
        0x55 => Ok(Instruction::BIT_NR),
        0x56 => Ok(Instruction::BIT_NHL),
        0x57 => Ok(Instruction::BIT_NR),
        0x58 => Ok(Instruction::BIT_NR),
        0x59 => Ok(Instruction::BIT_NR),
        0x5A => Ok(Instruction::BIT_NR),
        0x5B => Ok(Instruction::BIT_NR),
        0x5C => Ok(Instruction::BIT_NR),
        0x5D => Ok(Instruction::BIT_NR),
        0x5E => Ok(Instruction::BIT_NHL),
        0x5F => Ok(Instruction::BIT_NR),
        0x60 => Ok(Instruction::BIT_NR),
        0x61 => Ok(Instruction::BIT_NR),
        0x62 => Ok(Instruction::BIT_NR),
        0x63 => Ok(Instruction::BIT_NR),
        0x64 => Ok(Instruction::BIT_NR),
        0x65 => Ok(Instruction::BIT_NR),
        0x66 => Ok(Instruction::BIT_NHL),
        0x67 => Ok(Instruction::BIT_NR),
        0x68 => Ok(Instruction::BIT_NR),
        0x69 => Ok(Instruction::BIT_NR),
        0x6A => Ok(Instruction::BIT_NR),
        0x6B => Ok(Instruction::BIT_NR),
        0x6C => Ok(Instruction::BIT_NR),
        0x6D => Ok(Instruction::BIT_NR),
        0x6E => Ok(Instruction::BIT_NHL),
        0x6F => Ok(Instruction::BIT_NR),
        0x70 => Ok(Instruction::BIT_NR),
        0x71 => Ok(Instruction::BIT_NR),
        0x72 => Ok(Instruction::BIT_NR),
        0x73 => Ok(Instruction::BIT_NR),
        0x74 => Ok(Instruction::BIT_NR),
        0x75 => Ok(Instruction::BIT_NR),
        0x76 => Ok(Instruction::BIT_NHL),
        0x77 => Ok(Instruction::BIT_NR),
        0x78 => Ok(Instruction::BIT_NR),
        0x79 => Ok(Instruction::BIT_NR),
        0x7A => Ok(Instruction::BIT_NR),
        0x7B => Ok(Instruction::BIT_NR),
        0x7C => Ok(Instruction::BIT_NR),
        0x7D => Ok(Instruction::BIT_NR),
        0x7E => Ok(Instruction::BIT_NHL),
        0x7F => Ok(Instruction::BIT_NR),
        0x80 => Ok(Instruction::RES_NR),
        0x81 => Ok(Instruction::RES_NR),
        0x82 => Ok(Instruction::RES_NR),
        0x83 => Ok(Instruction::RES_NR),
        0x84 => Ok(Instruction::RES_NR),
        0x85 => Ok(Instruction::RES_NR),
        0x86 => Ok(Instruction::RES_NHL),
        0x87 => Ok(Instruction::RES_NR),
        0x88 => Ok(Instruction::RES_NR),
        0x89 => Ok(Instruction::RES_NR),
        0x8A => Ok(Instruction::RES_NR),
        0x8B => Ok(Instruction::RES_NR),
        0x8C => Ok(Instruction::RES_NR),
        0x8D => Ok(Instruction::RES_NR),
        0x8E => Ok(Instruction::RES_NHL),
        0x8F => Ok(Instruction::RES_NR),
        0x90 => Ok(Instruction::RES_NR),
        0x91 => Ok(Instruction::RES_NR),
        0x92 => Ok(Instruction::RES_NR),
        0x93 => Ok(Instruction::RES_NR),
        0x94 => Ok(Instruction::RES_NR),
        0x95 => Ok(Instruction::RES_NR),
        0x96 => Ok(Instruction::RES_NHL),
        0x97 => Ok(Instruction::RES_NR),
        0x98 => Ok(Instruction::RES_NR),
        0x99 => Ok(Instruction::RES_NR),
        0x9A => Ok(Instruction::RES_NR),
        0x9B => Ok(Instruction::RES_NR),
        0x9C => Ok(Instruction::RES_NR),
        0x9D => Ok(Instruction::RES_NR),
        0x9E => Ok(Instruction::RES_NHL),
        0x9F => Ok(Instruction::RES_NR),
        0xA0 => Ok(Instruction::RES_NR),
        0xA1 => Ok(Instruction::RES_NR),
        0xA2 => Ok(Instruction::RES_NR),
        0xA3 => Ok(Instruction::RES_NR),
        0xA4 => Ok(Instruction::RES_NR),
        0xA5 => Ok(Instruction::RES_NR),
        0xA6 => Ok(Instruction::RES_NHL),
        0xA7 => Ok(Instruction::RES_NR),
        0xA8 => Ok(Instruction::RES_NR),
        0xA9 => Ok(Instruction::RES_NR),
        0xAA => Ok(Instruction::RES_NR),
        0xAB => Ok(Instruction::RES_NR),
        0xAC => Ok(Instruction::RES_NR),
        0xAD => Ok(Instruction::RES_NR),
        0xAE => Ok(Instruction::RES_NHL),
        0xAF => Ok(Instruction::RES_NR),
        0xB0 => Ok(Instruction::RES_NR),
        0xB1 => Ok(Instruction::RES_NR),
        0xB2 => Ok(Instruction::RES_NR),
        0xB3 => Ok(Instruction::RES_NR),
        0xB4 => Ok(Instruction::RES_NR),
        0xB5 => Ok(Instruction::RES_NR),
        0xB6 => Ok(Instruction::RES_NHL),
        0xB7 => Ok(Instruction::RES_NR),
        0xB8 => Ok(Instruction::RES_NR),
        0xB9 => Ok(Instruction::RES_NR),
        0xBA => Ok(Instruction::RES_NR),
        0xBB => Ok(Instruction::RES_NR),
        0xBC => Ok(Instruction::RES_NR),
        0xBD => Ok(Instruction::RES_NR),
        0xBE => Ok(Instruction::RES_NHL),
        0xBF => Ok(Instruction::RES_NR),
        0xC0 => Ok(Instruction::SET_NR),
        0xC1 => Ok(Instruction::SET_NR),
        0xC2 => Ok(Instruction::SET_NR),
        0xC3 => Ok(Instruction::SET_NR),
        0xC4 => Ok(Instruction::SET_NR),
        0xC5 => Ok(Instruction::SET_NR),
        0xC6 => Ok(Instruction::SET_NHL),
        0xC7 => Ok(Instruction::SET_NR),
        0xC8 => Ok(Instruction::SET_NR),
        0xC9 => Ok(Instruction::SET_NR),
        0xCA => Ok(Instruction::SET_NR),
        0xCB => Ok(Instruction::SET_NR),
        0xCC => Ok(Instruction::SET_NR),
        0xCD => Ok(Instruction::SET_NR),
        0xCE => Ok(Instruction::SET_NHL),
        0xCF => Ok(Instruction::SET_NR),
        0xD0 => Ok(Instruction::SET_NR),
        0xD1 => Ok(Instruction::SET_NR),
        0xD2 => Ok(Instruction::SET_NR),
        0xD3 => Ok(Instruction::SET_NR),
        0xD4 => Ok(Instruction::SET_NR),
        0xD5 => Ok(Instruction::SET_NR),
        0xD6 => Ok(Instruction::SET_NHL),
        0xD7 => Ok(Instruction::SET_NR),
        0xD8 => Ok(Instruction::SET_NR),
        0xD9 => Ok(Instruction::SET_NR),
        0xDA => Ok(Instruction::SET_NR),
        0xDB => Ok(Instruction::SET_NR),
        0xDC => Ok(Instruction::SET_NR),
        0xDD => Ok(Instruction::SET_NR),
        0xDE => Ok(Instruction::SET_NHL),
        0xDF => Ok(Instruction::SET_NR),
        0xE0 => Ok(Instruction::SET_NR),
        0xE1 => Ok(Instruction::SET_NR),
        0xE2 => Ok(Instruction::SET_NR),
        0xE3 => Ok(Instruction::SET_NR),
        0xE4 => Ok(Instruction::SET_NR),
        0xE5 => Ok(Instruction::SET_NR),
        0xE6 => Ok(Instruction::SET_NHL),
        0xE7 => Ok(Instruction::SET_NR),
        0xE8 => Ok(Instruction::SET_NR),
        0xE9 => Ok(Instruction::SET_NR),
        0xEA => Ok(Instruction::SET_NR),
        0xEB => Ok(Instruction::SET_NR),
        0xEC => Ok(Instruction::SET_NR),
        0xED => Ok(Instruction::SET_NR),
        0xEE => Ok(Instruction::SET_NHL),
        0xEF => Ok(Instruction::SET_NR),
        0xF0 => Ok(Instruction::SET_NR),
        0xF1 => Ok(Instruction::SET_NR),
        0xF2 => Ok(Instruction::SET_NR),
        0xF3 => Ok(Instruction::SET_NR),
        0xF4 => Ok(Instruction::SET_NR),
        0xF5 => Ok(Instruction::SET_NR),
        0xF6 => Ok(Instruction::SET_NHL),
        0xF7 => Ok(Instruction::SET_NR),
        0xF8 => Ok(Instruction::SET_NR),
        0xF9 => Ok(Instruction::SET_NR),
        0xFA => Ok(Instruction::SET_NR),
        0xFB => Ok(Instruction::SET_NR),
        0xFC => Ok(Instruction::SET_NR),
        0xFD => Ok(Instruction::SET_NR),
        0xFE => Ok(Instruction::SET_NHL),
        0xFF => Ok(Instruction::SET_NR),
        _ => Err(OpcodeError::InvalidOpcodeInput),
    }
}

pub fn instruction_lookup(byte: &u8) -> OpcodeLookupResult {
    match byte {
        0x0 => Ok(Instruction::NOP),
        0x1 => Ok(Instruction::LD_RRNN),
        0x2 => Ok(Instruction::LD_BCA),
        0x3 => Ok(Instruction::INC_RR),
        0x4 => Ok(Instruction::INC_R),
        0x5 => Ok(Instruction::DEC_R),
        0x6 => Ok(Instruction::LD_RN),
        0x7 => Ok(Instruction::RLCA),
        0x8 => Ok(Instruction::LD_SPDD),
        0x9 => Ok(Instruction::ADD_HLRR),
        0xA => Ok(Instruction::LD_ABC),
        0xB => Ok(Instruction::DEC_RR),
        0xC => Ok(Instruction::INC_R),
        0xD => Ok(Instruction::DEC_R),
        0xE => Ok(Instruction::LD_RN),
        0xF => Ok(Instruction::RRCA),
        0x10 => Ok(Instruction::STOP),
        0x11 => Ok(Instruction::LD_RRNN),
        0x12 => Ok(Instruction::LD_DEA),
        0x13 => Ok(Instruction::INC_RR),
        0x14 => Ok(Instruction::INC_R),
        0x15 => Ok(Instruction::DEC_R),
        0x16 => Ok(Instruction::LD_RN),
        0x17 => Ok(Instruction::RLA),
        0x18 => Ok(Instruction::JR_PCDD),
        0x19 => Ok(Instruction::ADD_HLRR),
        0x1A => Ok(Instruction::LD_ADE),
        0x1B => Ok(Instruction::DEC_RR),
        0x1C => Ok(Instruction::INC_R),
        0x1D => Ok(Instruction::DEC_R),
        0x1E => Ok(Instruction::LD_RN),
        0x1F => Ok(Instruction::RRA),
        0x20 => Ok(Instruction::JR_FPCDD),
        0x21 => Ok(Instruction::LD_RRNN),
        0x22 => Ok(Instruction::LD_HLDA),
        0x23 => Ok(Instruction::INC_RR),
        0x24 => Ok(Instruction::INC_R),
        0x25 => Ok(Instruction::DEC_R),
        0x26 => Ok(Instruction::LD_RN),
        0x27 => Ok(Instruction::DAA),
        0x28 => Ok(Instruction::JR_FPCDD),
        0x29 => Ok(Instruction::ADD_HLRR),
        0x2A => Ok(Instruction::LD_AHLI),
        0x2B => Ok(Instruction::DEC_RR),
        0x2C => Ok(Instruction::INC_R),
        0x2D => Ok(Instruction::DEC_R),
        0x2E => Ok(Instruction::LD_RN),
        0x2F => Ok(Instruction::CPL),
        0x30 => Ok(Instruction::JR_FPCDD),
        0x31 => Ok(Instruction::LD_RRNN),
        0x32 => Ok(Instruction::LD_HLDA),
        0x33 => Ok(Instruction::INC_RR),
        0x34 => Ok(Instruction::INC_HL),
        0x35 => Ok(Instruction::DEC_HL),
        0x36 => Ok(Instruction::LD_HLN),
        0x37 => Ok(Instruction::SCF),
        0x38 => Ok(Instruction::JR_FPCDD),
        0x39 => Ok(Instruction::ADD_HLRR),
        0x3A => Ok(Instruction::LD_AHLD),
        0x3B => Ok(Instruction::DEC_RR),
        0x3C => Ok(Instruction::INC_R),
        0x3D => Ok(Instruction::DEC_R),
        0x3E => Ok(Instruction::LD_RN),
        0x3F => Ok(Instruction::CCF),
        0x40 => Ok(Instruction::LD_RR),
        0x41 => Ok(Instruction::LD_RR),
        0x42 => Ok(Instruction::LD_RR),
        0x43 => Ok(Instruction::LD_RR),
        0x44 => Ok(Instruction::LD_RR),
        0x45 => Ok(Instruction::LD_RR),
        0x46 => Ok(Instruction::LD_RHL),
        0x47 => Ok(Instruction::LD_RR),
        0x48 => Ok(Instruction::LD_RR),
        0x49 => Ok(Instruction::LD_RR),
        0x4A => Ok(Instruction::LD_RR),
        0x4B => Ok(Instruction::LD_RR),
        0x4C => Ok(Instruction::LD_RR),
        0x4D => Ok(Instruction::LD_RR),
        0x4E => Ok(Instruction::LD_RHL),
        0x4F => Ok(Instruction::LD_RR),
        0x50 => Ok(Instruction::LD_RR),
        0x51 => Ok(Instruction::LD_RR),
        0x52 => Ok(Instruction::LD_RR),
        0x53 => Ok(Instruction::LD_RR),
        0x54 => Ok(Instruction::LD_RR),
        0x55 => Ok(Instruction::LD_RR),
        0x56 => Ok(Instruction::LD_RHL),
        0x57 => Ok(Instruction::LD_RR),
        0x58 => Ok(Instruction::LD_RR),
        0x59 => Ok(Instruction::LD_RR),
        0x5A => Ok(Instruction::LD_RR),
        0x5B => Ok(Instruction::LD_RR),
        0x5C => Ok(Instruction::LD_RR),
        0x5D => Ok(Instruction::LD_RR),
        0x5E => Ok(Instruction::LD_RHL),
        0x5F => Ok(Instruction::LD_RR),
        0x60 => Ok(Instruction::LD_RR),
        0x61 => Ok(Instruction::LD_RR),
        0x62 => Ok(Instruction::LD_RR),
        0x63 => Ok(Instruction::LD_RR),
        0x64 => Ok(Instruction::LD_RR),
        0x65 => Ok(Instruction::LD_RR),
        0x66 => Ok(Instruction::LD_RHL),
        0x67 => Ok(Instruction::LD_RR),
        0x68 => Ok(Instruction::LD_RR),
        0x69 => Ok(Instruction::LD_RR),
        0x6A => Ok(Instruction::LD_RR),
        0x6B => Ok(Instruction::LD_RR),
        0x6C => Ok(Instruction::LD_RR),
        0x6D => Ok(Instruction::LD_RR),
        0x6E => Ok(Instruction::LD_RHL),
        0x6F => Ok(Instruction::LD_RR),
        0x70 => Ok(Instruction::LD_HLR),
        0x71 => Ok(Instruction::LD_HLR),
        0x72 => Ok(Instruction::LD_HLR),
        0x73 => Ok(Instruction::LD_HLR),
        0x74 => Ok(Instruction::LD_HLR),
        0x75 => Ok(Instruction::LD_HLR),
        0x76 => Ok(Instruction::HALT),
        0x77 => Ok(Instruction::LD_HLR),
        0x78 => Ok(Instruction::LD_RR),
        0x79 => Ok(Instruction::LD_RR),
        0x7A => Ok(Instruction::LD_RR),
        0x7B => Ok(Instruction::LD_RR),
        0x7C => Ok(Instruction::LD_RR),
        0x7D => Ok(Instruction::LD_RR),
        0x7E => Ok(Instruction::LD_RHL),
        0x7F => Ok(Instruction::LD_RR),
        0x80 => Ok(Instruction::ADD_AR),
        0x81 => Ok(Instruction::ADD_AR),
        0x82 => Ok(Instruction::ADD_AR),
        0x83 => Ok(Instruction::ADD_AR),
        0x84 => Ok(Instruction::ADD_AR),
        0x85 => Ok(Instruction::ADD_AR),
        0x86 => Ok(Instruction::ADD_AHL),
        0x87 => Ok(Instruction::ADD_AR),
        0x88 => Ok(Instruction::ADC_AR),
        0x89 => Ok(Instruction::ADC_AR),
        0x8A => Ok(Instruction::ADC_AR),
        0x8B => Ok(Instruction::ADC_AR),
        0x8C => Ok(Instruction::ADC_AR),
        0x8D => Ok(Instruction::ADC_AR),
        0x8E => Ok(Instruction::ADC_AHL),
        0x8F => Ok(Instruction::ADC_AR),
        0x90 => Ok(Instruction::SUB_R),
        0x91 => Ok(Instruction::SUB_R),
        0x92 => Ok(Instruction::SUB_R),
        0x93 => Ok(Instruction::SUB_R),
        0x94 => Ok(Instruction::SUB_R),
        0x95 => Ok(Instruction::SUB_R),
        0x96 => Ok(Instruction::SUB_HL),
        0x97 => Ok(Instruction::SUB_R),
        0x98 => Ok(Instruction::SBC_AR),
        0x99 => Ok(Instruction::SBC_AR),
        0x9A => Ok(Instruction::SBC_AR),
        0x9B => Ok(Instruction::SBC_AR),
        0x9C => Ok(Instruction::SBC_AR),
        0x9D => Ok(Instruction::SBC_AR),
        0x9E => Ok(Instruction::SBC_AHL),
        0x9F => Ok(Instruction::SBC_AR),
        0xA0 => Ok(Instruction::AND_R),
        0xA1 => Ok(Instruction::AND_R),
        0xA2 => Ok(Instruction::AND_R),
        0xA3 => Ok(Instruction::AND_R),
        0xA4 => Ok(Instruction::AND_R),
        0xA5 => Ok(Instruction::AND_R),
        0xA6 => Ok(Instruction::AND_HL),
        0xA7 => Ok(Instruction::AND_R),
        0xA8 => Ok(Instruction::XOR_R),
        0xA9 => Ok(Instruction::XOR_R),
        0xAA => Ok(Instruction::XOR_R),
        0xAB => Ok(Instruction::XOR_R),
        0xAC => Ok(Instruction::XOR_R),
        0xAD => Ok(Instruction::XOR_R),
        0xAE => Ok(Instruction::XOR_HL),
        0xAF => Ok(Instruction::XOR_R),
        0xB0 => Ok(Instruction::OR_R),
        0xB1 => Ok(Instruction::OR_R),
        0xB2 => Ok(Instruction::OR_R),
        0xB3 => Ok(Instruction::OR_R),
        0xB4 => Ok(Instruction::OR_R),
        0xB5 => Ok(Instruction::OR_R),
        0xB6 => Ok(Instruction::OR_HL),
        0xB7 => Ok(Instruction::OR_R),
        0xB8 => Ok(Instruction::CP_R),
        0xB9 => Ok(Instruction::CP_R),
        0xBA => Ok(Instruction::CP_R),
        0xBB => Ok(Instruction::CP_R),
        0xBC => Ok(Instruction::CP_R),
        0xBD => Ok(Instruction::CP_R),
        0xBE => Ok(Instruction::CP_HL),
        0xBF => Ok(Instruction::CP_R),
        0xC0 => Ok(Instruction::RET_F),
        0xC1 => Ok(Instruction::POP_RR),
        0xC2 => Ok(Instruction::JP_FNN),
        0xC3 => Ok(Instruction::JP_NN),
        0xC4 => Ok(Instruction::CALL_FNN),
        0xC5 => Ok(Instruction::PUSH_RR),
        0xC6 => Ok(Instruction::ADD_AN),
        0xC7 => Ok(Instruction::RST),
        0xC8 => Ok(Instruction::RET_F),
        0xC9 => Ok(Instruction::RET),
        0xCA => Ok(Instruction::JP_FNN),
        0xCB => cb_prefix_instruction_lookup(byte),
        0xCC => Ok(Instruction::CALL_FNN),
        0xCD => Ok(Instruction::CALL_NN),
        0xCE => Ok(Instruction::ADC_AN),
        0xCF => Ok(Instruction::RST),
        0xD0 => Ok(Instruction::RET_F),
        0xD1 => Ok(Instruction::POP_RR),
        0xD2 => Ok(Instruction::JP_FNN),
        0xD4 => Ok(Instruction::CALL_FNN),
        0xD5 => Ok(Instruction::PUSH_RR),
        0xD6 => Ok(Instruction::SUB_N),
        0xD7 => Ok(Instruction::RST),
        0xD8 => Ok(Instruction::RET_F),
        0xD9 => Ok(Instruction::RETI),
        0xDA => Ok(Instruction::JP_FNN),
        0xDC => Ok(Instruction::CALL_FNN),
        0xDE => Ok(Instruction::SBC_AN),
        0xDF => Ok(Instruction::RST),
        0xE0 => Ok(Instruction::LD_NA),
        0xE1 => Ok(Instruction::POP_RR),
        0xE2 => Ok(Instruction::LD_FF00CA),
        0xE5 => Ok(Instruction::PUSH_RR),
        0xE6 => Ok(Instruction::AND_N),
        0xE7 => Ok(Instruction::RST),
        0xE8 => Ok(Instruction::ADD_SPN),
        0xE9 => Ok(Instruction::JP_HL),
        0xEA => Ok(Instruction::LD_NNA),
        0xEE => Ok(Instruction::XOR_N),
        0xEF => Ok(Instruction::RST),
        0xF0 => Ok(Instruction::LD_AN),
        0xF1 => Ok(Instruction::POP_RR),
        0xF2 => Ok(Instruction::LD_AFF00C),
        0xF3 => Ok(Instruction::DI),
        0xF5 => Ok(Instruction::PUSH_RR),
        0xF6 => Ok(Instruction::OR_N),
        0xF7 => Ok(Instruction::RST),
        0xF8 => Ok(Instruction::LDHL),
        0xF9 => Ok(Instruction::LD_SPHL),
        0xFA => Ok(Instruction::LD_ANN),
        0xFB => Ok(Instruction::EI),
        0xFE => Ok(Instruction::CP_N),
        0xFF => Ok(Instruction::RST),
        _ => Err(OpcodeError::InvalidOpcodeInput),
    }
}
