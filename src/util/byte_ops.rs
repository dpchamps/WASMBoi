#[allow(exceeding_bitshifts)]
pub fn left_shift(b: u8) -> u16 {
    ((b as u16) << 8)
}

const LHS_MASK: u8 = 0b00111000;
const RHS_MASK: u8 = 0b111;
const REGISTER_PAIR_MASK: u8 = 0b00111000;
const JUMP_CONDITION_MASK: u8 = 0b00011000;

pub fn hi_lo_combine(hi: u8, lo: u8) -> u16 {
    left_shift(hi) | (lo as u16)
}

pub fn extract_lhs(byte: u8) -> u8 {
    (byte & LHS_MASK) >> 3
}

pub fn extract_rhs(byte: u8) -> u8 {
    (byte & RHS_MASK)
}

#[cfg(test)]
mod byte_ops_test {
    use crate::util::byte_ops::{extract_lhs, extract_rhs, hi_lo_combine, left_shift};

    #[test]
    fn test_left_shift() {
        assert_eq!(left_shift(0xF), 0xF00)
    }

    #[test]
    fn test_hi_lo_combine() {
        let hi: u8 = 0xA;
        let lo: u8 = 0xCB;

        assert_eq!(hi_lo_combine(hi, lo), 0xACB)
    }

    #[test]
    fn test_extract_lhs() {
        assert_eq!(extract_lhs(0b101010), 0b101)
    }

    #[test]
    fn test_extract_rhs() {
        assert_eq!(extract_rhs(0b111011), 0b011)
    }
}
