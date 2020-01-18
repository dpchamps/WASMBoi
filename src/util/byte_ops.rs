#[allow(exceeding_bitshifts)]
pub fn left_shift (b : u8) -> u16 {
    ((b as u16) << 8)
}

pub fn hi_lo_combine(hi : u8, lo : u8) -> u16{
    left_shift(hi) | (lo as u16)
}

#[cfg(test)]
mod byte_ops_test{
    use crate::util::byte_ops::{left_shift, hi_lo_combine};

    #[test]
    fn test_left_shift(){
        assert_eq!(
            left_shift(0xF),
            0xF00
        )
    }

    #[test]
    fn test_hi_lo_combine(){
        let hi : u8 = 0xA;
        let lo : u8 = 0xCB;

        assert_eq!(
            hi_lo_combine(hi, lo),
            0xACB
        )
    }
}