#![no_std]

#[macro_export]
macro_rules! bitslice {
    ($left_most_end:expr, $right_most_start:expr, $value:expr) => {{
        let value = $value;
        let width = $left_most_end - $right_most_start + 1;

        let mask = (1 << width) - 1;

        (value >> $right_most_start) & mask
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test_value: u8 = 0b1111_1111;
        let result: u8 = !bitslice!(4_u8, 2_u8, test_value);
        assert_eq!(result, 0b0011_0000);
    }
}
