#[derive(Debug, Eq, PartialEq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}


pub fn classify(num: u64) -> Option<Classification> {
    let aliquot_sum: u64 = (1..num)
        .filter(|i| num % i == 0)
        .sum();
    match aliquot_sum {
        _ if num == 0 => None,
        _ if aliquot_sum == num => Some(Classification::Perfect),
        _ if aliquot_sum > num => Some(Classification::Abundant),
        _ => Some(Classification::Deficient),
    }
}


mod perfect_number_test {
    use super::classify;
    use crate::perfect_numbers::Classification;

    #[test]
    fn smallest_perfect_number_is_classified_correctly() {
        let input = 6;
        let output = classify(input);
        let expected = Some(Classification::Perfect);
        assert_eq!(output, expected);
    }

    #[test]
    fn medium_perfect_number_is_classified_correctly() {
        let input = 28;
        let output = classify(input);
        let expected = Some(Classification::Perfect);
        assert_eq!(output, expected);
    }

    #[test]
    fn large_perfect_number_is_classified_correctly() {
        let input = 33550336;
        let output = classify(input);
        let expected = Some(Classification::Perfect);
        assert_eq!(output, expected);
    }

    #[test]
    fn smallest_abundant_number_is_classified_correctly() {
        let input = 12;
        let output = classify(input);
        let expected = Some(Classification::Abundant);
        assert_eq!(output, expected);
    }

    #[test]
    fn medium_abundant_number_is_classified_correctly() {
        let input = 30;
        let output = classify(input);
        let expected = Some(Classification::Abundant);
        assert_eq!(output, expected);
    }

    #[test]
    fn large_abundant_number_is_classified_correctly() {
        let input = 33550335;
        let output = classify(input);
        let expected = Some(Classification::Abundant);
        assert_eq!(output, expected);
    }

    #[test]
    fn smallest_prime_deficient_number_is_classified_correctly() {
        let input = 2;
        let output = classify(input);
        let expected = Some(Classification::Deficient);
        assert_eq!(output, expected);
    }

    #[test]
    fn smallest_non_prime_deficient_number_is_classified_correctly() {
        let input = 4;
        let output = classify(input);
        let expected = Some(Classification::Deficient);
        assert_eq!(output, expected);
    }

    #[test]
    fn medium_deficient_number_is_classified_correctly() {
        let input = 32;
        let output = classify(input);
        let expected = Some(Classification::Deficient);
        assert_eq!(output, expected);
    }

    #[test]
    fn large_deficient_number_is_classified_correctly() {
        let input = 33550337;
        let output = classify(input);
        let expected = Some(Classification::Deficient);
        assert_eq!(output, expected);
    }

    #[test]
    fn edge_case_no_factors_other_than_itself_is_classified_correctly() {
        let input = 1;
        let output = classify(input);
        let expected = Some(Classification::Deficient);
        assert_eq!(output, expected);
    }

    #[test]
    fn zero_is_rejected_as_it_is_not_a_positive_integer() {
        let input = 0;
        let output = classify(input);
        assert!(output.is_none());
    }
}