#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    }
    if let Some(idx) = string_digits.chars().position(|c| !c.is_ascii_digit()) {
        return Err(Error::InvalidDigit(string_digits.chars().collect::<Vec<char>>()[idx]));
    }
    if let Some(result) = string_digits
        .chars()
        .collect::<Vec<char>>()
        .windows(span)
        .map(
            |window| window
                .iter()
                .map(|&c| c as u64 - '0' as u64)
                .product()
        )
        .max() {
        Ok(result)
    } else {
        Err(Error::SpanTooLong)
    }
}

mod lsp_test {
    use crate::largest_series_product::{Error, lsp};

    #[test]
    fn return_is_a_result() {
        assert!(lsp("29", 2).is_ok());
    }

    #[test]
    fn find_the_largest_product_when_span_equals_length() {
        assert_eq!(Ok(18), lsp("29", 2));
    }

    #[test]
    fn find_the_largest_product_of_two_with_numbers_in_order() {
        assert_eq!(Ok(72), lsp("0123456789", 2));
    }

    #[test]
    fn find_the_largest_product_of_two_with_numbers_not_in_order() {
        assert_eq!(Ok(48), lsp("576802143", 2));
    }

    #[test]
    fn find_the_largest_product_of_three_with_numbers_in_order() {
        assert_eq!(Ok(504), lsp("0123456789", 3));
    }

    #[test]
    fn find_the_largest_product_of_three_with_numbers_not_in_order() {
        assert_eq!(Ok(270), lsp("1027839564", 3));
    }

    #[test]
    fn find_the_largest_product_of_five_with_numbers_in_order() {
        assert_eq!(Ok(15_120), lsp("0123456789", 5));
    }

    #[test]
    fn span_of_six_in_a_large_number() {
        assert_eq!(
            Ok(23_520),
            lsp("73167176531330624919225119674426574742355349194934", 6)
        );
    }

    #[test]
    fn returns_zero_if_number_is_zeros() {
        assert_eq!(Ok(0), lsp("0000", 2));
    }

    #[test]
    fn returns_zero_if_all_products_are_zero() {
        assert_eq!(Ok(0), lsp("99099", 3));
    }

    #[test]
    fn a_span_is_longer_than_number_is_an_error() {
        assert_eq!(Err(Error::SpanTooLong), lsp("123", 4));
    }

    #[test]
    fn empty_string_and_non_zero_span_is_an_error() {
        assert_eq!(Err(Error::SpanTooLong), lsp("", 1));
    }

    #[test]
    fn a_string_with_non_digits_is_an_error() {
        assert_eq!(Err(Error::InvalidDigit('a')), lsp("1234a5", 2));
    }
}