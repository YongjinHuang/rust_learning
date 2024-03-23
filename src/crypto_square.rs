pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    let binding = input.to_ascii_lowercase();
    let mut normalized_input = binding
        .chars()
        .filter(|ch| ch.is_alphanumeric());
    let (r, c) = calculate_rc(normalized_input.clone().count());
    let mut string_set = vec![String::new(); c];
    for idx in 0..c * r {
        match normalized_input.next() {
            Some(ch) => string_set[idx % c].push(ch),
            None => string_set[idx % c].push(' '),
        }
    }
    string_set.join(" ")
}

pub fn calculate_rc(w: usize) -> (usize, usize) {
    let square_root = (w as f64).sqrt().ceil() as usize;
    if (square_root - 1) * square_root >= w {
        (square_root - 1, square_root)
    } else {
        (square_root, square_root)
    }
}

mod crypto_square_test {
    use crate::crypto_square::encrypt;

    fn test(input: &str, output: &str) {
        assert_eq!(&encrypt(input), output);
    }

    #[test]
    fn empty_input() {
        test("", "")
    }

    #[test]
    fn encrypt_also_decrypts_square() {
        // note that you only get the exact input back if:
        // 1. no punctuation
        // 2. even spacing
        // 3. all lowercase
        // 4. square input
        let example = "lime anda coco anut";
        assert_eq!(example, &encrypt(&encrypt(example)));
    }

    #[test]
    fn example() {
        test(
            "If man was meant to stay on the ground, god would have given us roots.",
            "imtgdvs fearwer mayoogo anouuio ntnnlvt wttddes aohghn  sseoau ",
        )
    }

    #[test]
    fn empty_last_line() {
        test("congratulate", "crl oaa ntt gue")
    }

    #[test]
    fn spaces_are_reorganized() {
        test("abet", "ae bt");
        test("a bet", "ae bt");
        test("     a  b     e      t             ", "ae bt");
    }

    #[test]
    fn everything_becomes_lowercase() {
        test("caSe", "cs ae");
        test("cAsE", "cs ae");
        test("CASE", "cs ae");
    }

    #[test]
    fn long() {
        test(
            r#"
We choose to go to the moon.

We choose to go to the moon in this decade and do the other things,
not because they are easy, but because they are hard, because that
goal will serve to organize and measure the best of our energies and
skills, because that challenge is one that we are willing to accept,
one we are unwilling to postpone, and one which we intend to win,
and the others, too.

-- John F. Kennedy, 12 September 1962
        "#,
            &(String::from("womdbudlmecsgwdwob enooetbsenaotioihe ")
                + "cwotcbeeaeunolnnnr henhaecrsrsealeaf1 ocieucavugetciwnk9 "
                + "ohnosauerithcnhde6 sotteusteehaegitn2 eohhtseotsatptchn  "
                + "tsiehetohatwtohee  oesrethrenceopwod  gtdtyhagbdhanoety  "
                + "ooehaetaesaresih1  tgcirygnsklewtne2  ooaneaoitilweptrs  "
                + "ttdgerazoleiaoese  hoesaeleflnlrnntp  etanshwaosgleedot  "
                + "mhnoyainubeiuatoe  oedtbrldreinnnojm "),
        )
    }
}