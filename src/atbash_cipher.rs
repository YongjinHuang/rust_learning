use std::collections::HashMap;

const PLAIN: &'static str = "abcdefghijklmnopqrstuvwxyz";
const CIPHER: &'static str = "zyxwvutsrqponmlkjihgfedcba";

pub fn create_hmap() -> HashMap<char, char> {
    PLAIN.chars()
        .zip(CIPHER.chars())
        .collect()
}

pub fn encode(plain: &str) -> String {
    let hmap = create_hmap();
    plain.to_ascii_lowercase().chars()
        .filter_map(|c| hmap.get(&c).copied().or_else(|| if c.is_digit(10) { Some(c) } else { None }))
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn decode(cipher: &str) -> String {
    let hmap = create_hmap();
    cipher.chars()
        .filter_map(|c| hmap.get(&c.to_ascii_lowercase()).copied().or_else(|| if c.is_digit(10) { Some(c) } else { None }))
        .collect()
}

mod atbash_cipher_test {
    use crate::atbash_cipher::{decode, encode};

    #[test]
    fn encode_yes() {
        assert_eq!(encode("yes"), "bvh");
    }

    #[test]
    fn encode_no() {
        assert_eq!(encode("no"), "ml");
    }

    #[test]
    fn encode_omg() {
        assert_eq!(encode("OMG"), "lnt");
    }

    #[test]
    fn encode_spaces() {
        assert_eq!(encode("O M G"), "lnt");
    }

    #[test]
    fn encode_mindblowingly() {
        assert_eq!(encode("mindblowingly"), "nrmwy oldrm tob");
    }

    #[test]
    fn encode_numbers() {
        assert_eq!(
            encode("Testing,1 2 3, testing."),
            "gvhgr mt123 gvhgr mt"
        );
    }

    #[test]
    fn encode_deep_thought() {
        assert_eq!(encode("Truth is fiction."), "gifgs rhurx grlm");
    }

    #[test]
    fn encode_all_the_letters() {
        assert_eq!(
            encode("The quick brown fox jumps over the lazy dog."),
            "gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt"
        );
    }

    #[test]
    fn decode_exercism() {
        assert_eq!(decode("vcvix rhn"), "exercism");
    }

    #[test]
    fn decode_a_sentence() {
        assert_eq!(
            decode("zmlyh gzxov rhlug vmzhg vkkrm thglm v"),
            "anobstacleisoftenasteppingstone"
        );
    }

    #[test]
    fn decode_numbers() {
        assert_eq!(decode("gvhgr mt123 gvhgr mt"), "testing123testing");
    }

    #[test]
    fn decode_all_the_letters() {
        assert_eq!(
            decode("gsvjf rxpyi ldmul cqfnk hlevi gsvoz abwlt"),
            "thequickbrownfoxjumpsoverthelazydog"
        );
    }

    #[test]
    fn decode_with_too_many_spaces() {
        assert_eq!(decode("vc vix    r hn"), "exercism");
    }

    #[test]
    fn decode_with_no_spaces() {
        assert_eq!(
        decode("zmlyhgzxovrhlugvmzhgvkkrmthglmv"),
        "anobstacleisoftenasteppingstone",
    );
    }
}