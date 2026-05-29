#![no_std]

#[macro_export]
macro_rules! bitslice {
    ($left_most_end:expr, $right_most_start:expr, $zero_of_value_type:expr, $value:expr) => {{
        let bits = $zero_of_value_type;
        let width = $left_most_end - $right_most_start + 1;
        let mask = bits ^ (width << $right_most_start);
        !($value & mask)
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test_value: u8 = 0b1011_0111;
        let result: u8 = !bitslice!(4_u8, 2_u8, 0_u8, test_value);
        assert_eq!(result, 0b0000_0100);
    }
}
