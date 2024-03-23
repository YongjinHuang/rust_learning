use unicode_segmentation::UnicodeSegmentation;

fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
    // input.chars().rev().collect()
}


fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1),
    }
}

fn square_of_sum(n: u32) -> u32 {
    (1..=n)
        .into_iter()
        .sum::<u32>()
        .pow(2)
}

fn sum_of_squares(n: u32) -> u32 {
    (1..=n)
        .into_iter()
        .map(|ele| ele.pow(2))
        .sum()
}

fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .into_iter()
        .map(|idx| verse(idx))
        .rev()
        .collect::<Vec<String>>()
        .join("\n")
}

fn is_lear_year(year: u64) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    string
        .chars()
        .into_iter()
        .for_each(|ele| match ele {
            '(' | '[' | '{' => {
                stack.push(ele);
            }
            ')' | ']' | '}' => {
                if stack.is_empty() {
                    stack.push(ele);
                } else {
                    match (stack.pop().unwrap(), ele) {
                        ('(', ')') | ('[', ']') | ('{', '}') => (),
                        _ => stack.push(ele),
                    }
                }
            }
            _ => (),
        });
    stack.len() == 0
}


mod exercism_tests {
    use super::*;

    #[test]
    fn test_plants() {
        let diagram = "RC
GG";
        println!("{:?}", diagram);
    }

    #[test]
    fn test_square_of_sum() {
        println!("{} - {}", square_of_sum(10), sum_of_squares(10))
    }

    #[test]
    fn test_sing() {
        println!("{}", sing(8, 6))
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("hello"), "olleh");
        assert_eq!(reverse("uüu"), "uüu");
    }
}
