pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let code: Vec<char> = self.to_string().chars().filter(|&c| c != ' ').collect();
        let code_len = code.len();
        if !code.iter().all(|c| c.is_digit(10)) || code_len < 2 {
            return false;
        }
        let code: Vec<u32> = code.iter().filter_map(|&c| c.to_digit(10)).collect();
        let (_, sum) = (0..code_len).rev().fold((0, 0), move |(counter, acc), idx| {
            if counter & 1 == 0 {
                (counter + 1, acc + code[idx])
            } else {
                let mut tmp = 2 * code[idx];
                if tmp >= 10 {
                    tmp -= 9;
                }
                (counter + 1, acc + tmp)
            }
        });
        return sum % 10 == 0;
    }
}


mod luhn_test {
    use super::Luhn;

    #[test]
    fn you_can_validate_from_a_str() {
        assert!("046 454 286".valid_luhn());
        assert!(!"046 454 287".valid_luhn());
    }

    #[test]
    fn you_can_validate_from_a_string() {
        assert!(String::from("046 454 286").valid_luhn());
        assert!(!String::from("046 454 287").valid_luhn());
    }

    #[test]
    fn you_can_validate_from_a_u8() {
        assert!(240u8.valid_luhn());
        assert!(!241u8.valid_luhn());
    }

    #[test]
    fn you_can_validate_from_a_u16() {
        let valid = 64_436u16;
        let invalid = 64_437u16;
        assert!(valid.valid_luhn());
        assert!(!invalid.valid_luhn());
    }

    #[test]
    fn you_can_validate_from_a_u32() {
        let valid = 46_454_286u32;
        let invalid = 46_454_287u32;
        assert!(valid.valid_luhn());
        assert!(!invalid.valid_luhn());
    }

    #[test]
    fn you_can_validate_from_a_u64() {
        let valid = 8273_1232_7352_0562u64;
        let invalid = 8273_1232_7352_0569u64;
        assert!(valid.valid_luhn());
        assert!(!invalid.valid_luhn());
    }

    #[test]
    fn you_can_validate_from_a_usize() {
        let valid = 8273_1232_7352_0562usize;
        let invalid = 8273_1232_7352_0569usize;
        assert!(valid.valid_luhn());
        assert!(!invalid.valid_luhn());
    }

    #[test]
    fn input_digit_9_is_still_correctly_converted_to_output_digit_9() {
        assert!("091".valid_luhn());
    }
}