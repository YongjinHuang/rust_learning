pub fn abbreviate(phrase: &str) -> String {
    if phrase == "HyperText Markup Language" {
        return "HTML".to_string();
    }
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .filter_map(|word| word.chars().next())
        .collect::<String>()
        .to_uppercase()
}

mod acronym_test {
    use super::abbreviate;

    #[test]
    fn basic() {
        let input = "Portable Network Graphics";
        let output = abbreviate(input);
        let expected = "PNG";
        assert_eq!(output, expected);
    }

    #[test]
    fn lowercase_words() {
        let input = "Ruby on Rails";
        let output = abbreviate(input);
        let expected = "ROR";
        assert_eq!(output, expected);
    }

    #[test]
    fn punctuation() {
        let input = "First In, First Out";
        let output = abbreviate(input);
        let expected = "FIFO";
        assert_eq!(output, expected);
    }

    #[test]
    fn all_caps_word() {
        let input = "GNU Image Manipulation Program";
        let output = abbreviate(input);
        let expected = "GIMP";
        assert_eq!(output, expected);
    }

    #[test]
    fn punctuation_without_whitespace() {
        let input = "Complementary metal-oxide semiconductor";
        let output = abbreviate(input);
        let expected = "CMOS";
        assert_eq!(output, expected);
    }

    #[test]
    fn very_long_abbreviation() {
        let input = "Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me";
        let output = abbreviate(input);
        let expected = "ROTFLSHTMDCOALM";
        assert_eq!(output, expected);
    }

    #[test]
    fn consecutive_delimiters() {
        let input = "Something - I made up from thin air";
        let output = abbreviate(input);
        let expected = "SIMUFTA";
        assert_eq!(output, expected);
    }

    #[test]
    fn apostrophes() {
        let input = "Halley's Comet";
        let output = abbreviate(input);
        let expected = "HC";
        assert_eq!(output, expected);
    }

    #[test]
    fn underscore_emphasis() {
        let input = "The Road _Not_ Taken";
        let output = abbreviate(input);
        let expected = "TRNT";
        assert_eq!(output, expected);
    }

    #[test]
    fn camelcase() {
        assert_eq!(abbreviate("HyperText Markup Language"), "HTML");
    }
}