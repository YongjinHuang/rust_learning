pub fn encode(source: &str) -> String {
    let mut res = Vec::new();
    let mut chars = source.chars().peekable();
    while let Some(c) = chars.next() {
        let mut count = 1;
        while chars.peek() == Some(&c) {
            chars.next();
            count += 1;
        }
        if count > 1 {
            res.extend(count.to_string().chars());
        }
        res.push(c);
    }
    res.iter().collect()
}

pub fn decode(source: &str) -> String {
    let mut i = 0;
    let source_chars = source
        .chars()
        .collect::<Vec<char>>();
    let source_chars_len = source_chars.len();
    let mut result: Vec<char> = Vec::new();
    while i < source_chars_len {
        let mut length = 0;
        while let Some(d) = source_chars[i].to_digit(10) {
            if i == source_chars_len {
                break
            }
            length = length * 10 + d;
            i += 1;
        }
        if length == 0 {
            result.push(source_chars[i]);
        } else {
            while length > 0 {
                result.push(source_chars[i]);
                length -= 1;
            }
        }
        i += 1;
    }
    result.iter().collect()
}


mod rle_test {
    use super::encode;
    use super::decode;

    #[test]
    fn encode_empty_string() {
        let input = "";
        let output = encode(input);
        let expected = "";
        assert_eq!(output, expected);
    }

    #[test]
    fn encode_single_characters_only_are_encoded_without_count() {
        let input = "XYZ";
        let output = encode(input);
        let expected = "XYZ";
        assert_eq!(output, expected);
    }

    #[test]
    fn encode_string_with_no_single_characters() {
        let input = "AABBBCCCC";
        let output = encode(input);
        let expected = "2A3B4C";
        assert_eq!(output, expected);
    }

    #[test]
    fn encode_single_characters_mixed_with_repeated_characters() {
        let input = "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB";
        let output = encode(input);
        let expected = "12WB12W3B24WB";
        assert_eq!(output, expected);
    }

    #[test]
    fn encode_multiple_whitespace_mixed_in_string() {
        let input = "  hsqq qww  ";
        let output = encode(input);
        let expected = "2 hs2q q2w2 ";
        assert_eq!(output, expected);
    }

    #[test]
    fn encode_lowercase_characters() {
        let input = "aabbbcccc";
        let output = encode(input);
        let expected = "2a3b4c";
        assert_eq!(output, expected);
    }

    #[test]
    fn decode_empty_string() {
        let input = "";
        let output = decode(input);
        let expected = "";
        assert_eq!(output, expected);
    }

    #[test]
    fn decode_single_characters_only() {
        let input = "XYZ";
        let output = decode(input);
        let expected = "XYZ";
        assert_eq!(output, expected);
    }

    #[test]
    fn decode_string_with_no_single_characters() {
        let input = "2A3B4C";
        let output = decode(input);
        let expected = "AABBBCCCC";
        assert_eq!(output, expected);
    }

    #[test]
    fn decode_single_characters_with_repeated_characters() {
        let input = "12WB12W3B24WB";
        let output = decode(input);
        let expected = "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB";
        assert_eq!(output, expected);
    }

    #[test]
    fn decode_multiple_whitespace_mixed_in_string() {
        let input = "2 hs2q q2w2 ";
        let output = decode(input);
        let expected = "  hsqq qww  ";
        assert_eq!(output, expected);
    }

    #[test]
    fn decode_lowercase_string() {
        let input = "2a3b4c";
        let output = decode(input);
        let expected = "aabbbcccc";
        assert_eq!(output, expected);
    }

    #[test]
    fn consistency_encode_followed_by_decode_gives_original_string() {
        let input = "zzz ZZ  zZ";
        let output = decode(&encode(input));
        let expected = "zzz ZZ  zZ";
        assert_eq!(output, expected);
    }
}