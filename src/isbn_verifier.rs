/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let (count, sum) = isbn
        .chars()
        .into_iter()
        .fold((10i32, 0i32), |(count, sum), c| {
            if c.is_digit(10) || (c == 'X' && count == 1) {
                (count - 1, sum + count * c.to_digit(10).unwrap_or(10) as i32)
            } else {
                (count, sum)
            }
        });
    count == 0 && sum % 11 == 0
}

mod isbn_valid_test {
    use super::is_valid_isbn;

    #[test]
    fn valid() {
        assert!(is_valid_isbn("3-598-21508-8"));
    }

    #[test]
    fn invalid_check_digit() {
        assert!(!is_valid_isbn("3-598-21508-9"));
    }

    #[test]
    fn valid_check_digit_of_10() {
        assert!(is_valid_isbn("3-598-21507-X"));
    }

    #[test]
    fn invalid_character_as_check_digit() {
        assert!(!is_valid_isbn("3-598-21507-A"));
    }

    #[test]
    fn invalid_character_in_isbn() {
        assert!(!is_valid_isbn("3-598-P1581-X"));
    }

    #[test]
    #[allow(non_snake_case)]
    fn invalid_isbn_with_invalid_X() {
        assert!(!is_valid_isbn("3-598-2X507-9"));
    }

    #[test]
    fn valid_isbn_without_dashes() {
        assert!(is_valid_isbn("3598215088"));
    }

    #[test]
    #[allow(non_snake_case)]
    fn valid_isbn_without_dashes_and_X_as_check() {
        assert!(is_valid_isbn("359821507X"));
    }

    #[test]
    fn invalid_isbn_without_dashes_and_no_check_digit() {
        assert!(!is_valid_isbn("359821507"));
    }

    #[test]
    fn invalid_isbn_without_dashes_and_too_long() {
        assert!(!is_valid_isbn("3598215078X"));
    }

    #[test]
    fn too_short_isbn() {
        assert!(!is_valid_isbn("00"));
    }

    #[test]
    fn invalid_isbn_without_check_digit() {
        assert!(!is_valid_isbn("3-598-21507"));
    }

    #[test]
    fn valid_digits_invalid_length() {
        assert!(!is_valid_isbn("35982150881"));
    }

    #[test]
    fn special_characters() {
        assert!(!is_valid_isbn("!@#%!@"));
    }

    #[test]
    #[allow(non_snake_case)]
    fn invalid_isbn_with_check_digit_X_instead_of_0() {
        assert!(!is_valid_isbn("3-598-21515-X"));
    }

    #[test]
    fn empty_isbn() {
        assert!(!is_valid_isbn(""));
    }

    #[test]
    fn input_is_9_characters() {
        assert!(!is_valid_isbn("134456729"));
    }

    #[test]
    fn invalid_characters_are_not_ignored() {
        assert!(!is_valid_isbn("3132P34035"));
    }

    #[test]
    fn too_long_but_contains_a_valid_isbn() {
        assert!(!is_valid_isbn("98245726788"));
    }
}