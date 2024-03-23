pub fn nth(n: u32) -> u32 {
    (2..)
        .filter(|&num| is_prime(num))
        .nth(n as usize)
        .unwrap()
}

pub fn is_prime(n: u32) -> bool {
    !(2..)
        .take_while(|num| num * num <= n)
        .any(|num| n % num == 0)
}

mod nth_prime_test {
    use super::*;

    #[test]
    fn first_prime() {
        assert_eq!(nth(0), 2);
    }

    #[test]
    fn second_prime() {
        assert_eq!(nth(1), 3);
    }

    #[test]
    fn sixth_prime() {
        assert_eq!(nth(5), 13);
    }

    #[test]
    fn big_prime() {
        assert_eq!(nth(10_000), 104_743);
    }
}
