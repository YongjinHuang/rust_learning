use std::cmp::Ordering;
use std::collections::{BTreeSet};

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let mut reverse_tmp = value;
        let mut tmp = 0;
        while reverse_tmp > 0 {
            tmp = tmp * 10 + reverse_tmp % 10;
            reverse_tmp /= 10;
        }
        if tmp == value {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

impl PartialOrd<Self> for Palindrome {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Palindrome {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let btree = (min..=max)
        .into_iter()
        .flat_map(|num1| (num1..=max).into_iter().map(move |num2| num1 * num2))
        .filter_map(|v| Palindrome::new(v))
        .collect::<BTreeSet<Palindrome>>();
    let min_value = btree.first();
    let max_value = btree.last();
    match (min_value, max_value) {
        (Some(&min_v), Some(&max_v)) => Some((min_v, max_v)),
        _ => None
    }
}

mod palindrome_products_test {
    use super::palindrome_products;
    use super::Palindrome;

    /// Process a single test case for the property `smallest`
    ///
    /// All cases for the `smallest` property are implemented
    /// in terms of this function.
    fn process_smallest_case((from, to): (u64, u64), expected: Option<u64>) {
        let min = palindrome_products(from, to).map(|(min, _)| min);
        assert_eq!(min.map(|newtype| newtype.into_inner()), expected);
    }

    /// Process a single test case for the property `largest`
    ///
    /// All cases for the `largest` property are implemented
    /// in terms of this function.
    fn process_largest_case((from, to): (u64, u64), expected: Option<u64>) {
        let max = palindrome_products(from, to).map(|(_, max)| max);
        assert_eq!(max.map(|newtype| newtype.into_inner()), expected);
    }

    #[test]
    /// test `Palindrome::new` with valid input
    fn palindrome_new_return_some() {
        for v in [1, 11, 121, 12321, 1234321, 123454321, 543212345] {
            assert_eq!(Palindrome::new(v).expect("is a palindrome").into_inner(), v);
        }
    }

    #[test]
    /// test `Palindrome::new` with invalid input
    fn palindrome_new_return_none() {
        for v in [12, 2322, 23443, 1233211, 8932343] {
            assert_eq!(Palindrome::new(v), None);
        }
    }

    #[test]
    /// finds the smallest palindrome from single digit factors
    fn finds_the_smallest_palindrome_from_single_digit_factors() {
        process_smallest_case((1, 9), Some(1));
    }

    #[test]
    /// finds the largest palindrome from single digit factors
    fn finds_the_largest_palindrome_from_single_digit_factors() {
        process_largest_case((1, 9), Some(9));
    }

    #[test]
    /// find the smallest palindrome from double digit factors
    fn find_the_smallest_palindrome_from_double_digit_factors() {
        process_smallest_case((10, 99), Some(121));
    }

    #[test]
    /// find the largest palindrome from double digit factors
    fn find_the_largest_palindrome_from_double_digit_factors() {
        process_largest_case((10, 99), Some(9009));
    }

    #[test]
    /// find smallest palindrome from triple digit factors
    fn find_smallest_palindrome_from_triple_digit_factors() {
        process_smallest_case((100, 999), Some(10201));
    }

    #[test]
    /// find the largest palindrome from triple digit factors
    fn find_the_largest_palindrome_from_triple_digit_factors() {
        process_largest_case((100, 999), Some(906609));
    }

    #[test]
    /// find smallest palindrome from four digit factors
    fn find_smallest_palindrome_from_four_digit_factors() {
        process_smallest_case((1000, 9999), Some(1002001));
    }

    #[test]
    /// find the largest palindrome from four digit factors
    fn find_the_largest_palindrome_from_four_digit_factors() {
        process_largest_case((1000, 9999), Some(99000099));
    }

    #[test]
    /// empty result for smallest if no palindrome in the range
    fn empty_result_for_smallest_if_no_palindrome_in_the_range() {
        process_smallest_case((1002, 1003), None);
    }

    #[test]
    /// empty result for largest if no palindrome in the range
    fn empty_result_for_largest_if_no_palindrome_in_the_range() {
        process_largest_case((15, 15), None);
    }

    #[test]
    /// error result for smallest if min is more than max
    fn error_result_for_smallest_if_min_is_more_than_max() {
        process_smallest_case((10000, 1), None);
    }

    #[test]
    /// error result for largest if min is more than max
    fn error_result_for_largest_if_min_is_more_than_max() {
        process_largest_case((2, 1), None);
    }
}