pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for c in string.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ')' => if  stack.pop() != Some('(') { return false },
            '}' => if  stack.pop() != Some('{') { return false },
            ']' => if  stack.pop() != Some('[') { return false },
            _ => {}
        }
    }
    stack.is_empty()
}


mod brackets_test {
    use crate::brackets::brackets_are_balanced;

    #[test]
    fn paired_square_brackets() {
        assert!(brackets_are_balanced("[]"));
    }

    #[test]
    fn empty_string() {
        assert!(brackets_are_balanced(""));
    }

    #[test]
    fn unpaired_brackets() {
        assert!(!brackets_are_balanced("[["));
    }

    #[test]
    fn wrong_ordered_brackets() {
        assert!(!brackets_are_balanced("}{"));
    }

    #[test]
    fn wrong_closing_bracket() {
        assert!(!brackets_are_balanced("{]"));
    }

    #[test]
    fn paired_with_whitespace() {
        assert!(brackets_are_balanced("{ }"));
    }

    #[test]
    fn partially_paired_brackets() {
        assert!(!brackets_are_balanced("{[])"));
    }

    #[test]
    fn simple_nested_brackets() {
        assert!(brackets_are_balanced("{[]}"));
    }

    #[test]
    fn several_paired_brackets() {
        assert!(brackets_are_balanced("{}[]"));
    }

    #[test]
    fn paired_and_nested_brackets() {
        assert!(brackets_are_balanced("([{}({}[])])"));
    }

    #[test]
    fn unopened_closing_brackets() {
        assert!(!brackets_are_balanced("{[)][]}"));
    }

    #[test]
    fn unpaired_and_nested_brackets() {
        assert!(!brackets_are_balanced("([{])"));
    }

    #[test]
    fn paired_and_wrong_nested_brackets() {
        assert!(!brackets_are_balanced("[({]})"));
    }

    #[test]
    fn paired_and_incomplete_brackets() {
        assert!(!brackets_are_balanced("{}["));
    }

    #[test]
    fn too_many_closing_brackets() {
        assert!(!brackets_are_balanced("[]]"));
    }

    #[test]
    fn early_incomplete_brackets() {
        assert!(!brackets_are_balanced(")()"));
    }

    #[test]
    fn early_mismatched_brackets() {
        assert!(!brackets_are_balanced("{)()"));
    }

    #[test]
    fn math_expression() {
        assert!(brackets_are_balanced("(((185 + 223.85) * 15) - 543)/2"));
    }

    #[test]
    fn complex_latex_expression() {
        let input = "\\left(\\begin{array}{cc} \\frac{1}{3} & x\\\\ \\mathrm{e}^{x} &... x^2 \
                 \\end{array}\\right)";
        assert!(brackets_are_balanced(input));
    }
}