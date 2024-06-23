pub mod naive_hash_table;

fn circular_shift_left_u32(input: u32, digit: u8) -> u32 {
    (input << digit) | (input >> (32 - digit))
}

fn circular_shift_right_u32(input: u32, digit: u8) -> u32 {
    (input >> digit) | (input << (32 - digit))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_circular_shift_left_u32() {
        assert_eq!(circular_shift_left_u32(0b0000_0000_0000_0000_0000_0000_0000_0010, 1), 0b0000_0000_0000_0000_0000_0000_0000_0100);
        assert_eq!(circular_shift_left_u32(0b0000_0000_0000_0000_0000_0000_0000_1000, 1), 0b0000_0000_0000_0000_0000_0000_0001_0000);
        assert_eq!(circular_shift_left_u32(0b0000_0000_0000_0000_0000_0000_0001_0000, 1), 0b0000_0000_0000_0000_0000_0000_0010_0000);
        assert_eq!(circular_shift_left_u32(0b0000_0000_0000_0000_0000_0000_0010_0000, 1), 0b0000_0000_0000_0000_0000_0000_0100_0000);
        assert_eq!(circular_shift_left_u32(0b0000_1111_0000_0000_0000_0000_0010_0000, 1), 0b0001_1110_0000_0000_0000_0000_0100_0000);
    }
}
