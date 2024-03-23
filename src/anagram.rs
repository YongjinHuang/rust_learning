use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
   possible_anagrams
       .iter()
       .filter(|&&w | is_anagram(word, w))
       .copied()
       .collect()
}

pub fn is_anagram(word: &str, candidate_word: &str) -> bool {
    if word.len() != candidate_word.len() || word.to_lowercase() == candidate_word.to_lowercase() {
        false
    } else {
        count_chars(word) == count_chars(candidate_word)
    }
}

pub fn count_chars(s: &str) -> HashMap<char, usize> {
    s.to_lowercase().chars().fold(HashMap::new(), |mut stat, c| {
        stat.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
        stat
    })
}

mod anagram_test {
    use std::collections::HashSet;
    use crate::anagram::anagrams_for;

    fn process_anagram_case(word: &str, inputs: &[&str], expected: &[&str]) {
        let result = anagrams_for(word, inputs);
        let expected: HashSet<&str> = expected.iter().cloned().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn no_matches() {
        let word = "diaper";
        let inputs = ["hello", "world", "zombies", "pants"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }

    #[test]
    fn detect_simple_anagram() {
        let word = "ant";
        let inputs = ["tan", "stand", "at"];
        let outputs = vec!["tan"];
        process_anagram_case(word, &inputs, &outputs);
    }

    #[test]
    fn does_not_confuse_different_duplicates() {
        let word = "galea";
        let inputs = ["eagle"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }

    #[test]
    fn eliminate_anagram_subsets() {
        let word = "good";
        let inputs = ["dog", "goody"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }

    #[test]
    fn detect_anagram() {
        let word = "listen";
        let inputs = ["enlists", "google", "inlets", "banana"];
        let outputs = vec!["inlets"];
        process_anagram_case(word, &inputs, &outputs);
    }

    #[test]
    fn multiple_anagrams() {
        let word = "allergy";
        let inputs = [
            "gallery",
            "ballerina",
            "regally",
            "clergy",
            "largely",
            "leading",
        ];
        let outputs = vec!["gallery", "regally", "largely"];
        process_anagram_case(word, &inputs, &outputs);
    }

    #[test]
    fn case_insensitive_anagrams() {
        let word = "Orchestra";
        let inputs = ["cashregister", "Carthorse", "radishes"];
        let outputs = vec!["Carthorse"];
        process_anagram_case(word, &inputs, &outputs);
    }

    #[test]
    fn unicode_anagrams() {
        let word = "ΑΒΓ";
        // These words don't make sense, they're just greek letters cobbled together.
        let inputs = ["ΒΓΑ", "ΒΓΔ", "γβα"];
        let outputs = vec!["ΒΓΑ", "γβα"];
        process_anagram_case(word, &inputs, &outputs);
    }

    #[test]
    fn misleading_unicode_anagrams() {
        // Despite what a human might think these words contain different letters, the input uses Greek
        // A and B while the list of potential anagrams uses Latin A and B.
        let word = "ΑΒΓ";
        let inputs = ["ABΓ"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }

    #[test]
    fn does_not_detect_a_word_as_its_own_anagram() {
        let word = "banana";
        let inputs = ["banana"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }

    #[test]
    fn does_not_detect_a_differently_cased_word_as_its_own_anagram() {
        let word = "banana";
        let inputs = ["bAnana"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }

    #[test]
    fn does_not_detect_a_differently_cased_unicode_word_as_its_own_anagram() {
        let word = "ΑΒΓ";
        let inputs = ["ΑΒγ"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }

    #[test]
    fn same_bytes_different_chars() {
        let word = "a⬂"; // 61 E2 AC 82
        let inputs = ["€a"]; // E2 82 AC 61
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }

    #[test]
    fn different_words_but_same_ascii_sum() {
        let word = "bc";
        let inputs = ["ad"];
        let outputs = vec![];
        process_anagram_case(word, &inputs, &outputs);
    }
}