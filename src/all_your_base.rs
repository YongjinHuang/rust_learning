#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    let mut accu = number.iter().try_fold(0, |accu, &digit| {
        if digit >= from_base {
            Err(Error::InvalidDigit(digit.clone()))
        } else {
            Ok(accu * from_base + digit)
        }
    })?;
    if accu == 0 {
        return Ok(vec![0]);
    }
    let mut res = vec![];
    while accu > 0 {
        res.push(accu % to_base);
        accu /= to_base;
    }
    res.reverse();
    Ok(res)
}

mod all_your_base_test {
    use crate::all_your_base::{convert, Error};


    #[test]
    fn single_bit_one_to_decimal() {
        let input_base = 2;
        let input_digits = &[1];
        let output_base = 10;
        let output_digits = vec![1];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }

    #[test]
    fn binary_to_single_decimal() {
        let input_base = 2;
        let input_digits = &[1, 0, 1];
        let output_base = 10;
        let output_digits = vec![5];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }

    #[test]
    fn single_decimal_to_binary() {
        let input_base = 10;
        let input_digits = &[5];
        let output_base = 2;
        let output_digits = vec![1, 0, 1];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }

    #[test]
    fn binary_to_multiple_decimal() {
        let input_base = 2;
        let input_digits = &[1, 0, 1, 0, 1, 0];
        let output_base = 10;
        let output_digits = vec![4, 2];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }

    #[test]
    fn decimal_to_binary() {
        let input_base = 10;
        let input_digits = &[4, 2];
        let output_base = 2;
        let output_digits = vec![1, 0, 1, 0, 1, 0];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }

    #[test]
    fn trinary_to_hexadecimal() {
        let input_base = 3;
        let input_digits = &[1, 1, 2, 0];
        let output_base = 16;
        let output_digits = vec![2, 10];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }

    #[test]
    fn hexadecimal_to_trinary() {
        let input_base = 16;
        let input_digits = &[2, 10];
        let output_base = 3;
        let output_digits = vec![1, 1, 2, 0];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }

    #[test]
    fn fifteen_bit_integer() {
        let input_base = 97;
        let input_digits = &[3, 46, 60];
        let output_base = 73;
        let output_digits = vec![6, 10, 45];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }

    #[test]
    fn empty_list() {
        let input_base = 2;
        let input_digits = &[];
        let output_base = 10;
        let output_digits = vec![0];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }

    #[test]
    fn single_zero() {
        let input_base = 10;
        let input_digits = &[0];
        let output_base = 2;
        let output_digits = vec![0];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }

    #[test]
    fn multiple_zeros() {
        let input_base = 10;
        let input_digits = &[0, 0, 0];
        let output_base = 2;
        let output_digits = vec![0];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }

    #[test]
    fn leading_zeros() {
        let input_base = 7;
        let input_digits = &[0, 6, 0];
        let output_base = 10;
        let output_digits = vec![4, 2];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }

    #[test]
    fn invalid_positive_digit() {
        let input_base = 2;
        let input_digits = &[1, 2, 1, 0, 1, 0];
        let output_base = 10;
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidDigit(2))
        );
    }

    #[test]
    fn input_base_is_one() {
        let input_base = 1;
        let input_digits = &[];
        let output_base = 10;
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidInputBase)
        );
    }

    #[test]
    fn output_base_is_one() {
        let input_base = 2;
        let input_digits = &[1, 0, 1, 0, 1, 0];
        let output_base = 1;
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidOutputBase)
        );
    }

    #[test]
    fn input_base_is_zero() {
        let input_base = 0;
        let input_digits = &[];
        let output_base = 10;
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidInputBase)
        );
    }

    #[test]
    fn output_base_is_zero() {
        let input_base = 10;
        let input_digits = &[7];
        let output_base = 0;
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidOutputBase)
        );
    }
}