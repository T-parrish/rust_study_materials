// Given a string containing just the characters '(', ')', '{', '}', '[' and ']'
// determine if the input string is valid. Open brackets must be closed by
// the same type of brackets && Open brackets must be closed in the correct order.
pub fn bracket_checker(input: String) -> bool {
    // base case returns true if input is an empty String
    if input == String::from("") {
        return true;
    }

    let mut brackets: Vec<char> = Vec::with_capacity(input.len());

    for c in input.chars() {
        match c {
            '[' => brackets.push(c),
            '(' => brackets.push(c),
            '{' => brackets.push(c),
            _ => {
                match brackets.pop().unwrap() {
                    '[' => {
                        if c == ']' {
                            continue;
                        } else {
                            return false;
                        }
                    }
                    '{' => {
                        if c == '}' {
                            continue;
                        } else {
                            return false;
                        }
                    }
                    '(' => {
                        if c == ')' {
                            continue;
                        } else {
                            return false;
                        }
                    }
                    _ => return false, // return false if the character isn't allowed
                }
            }
        }
    }

    // evaluate to true if all brackets are properly closed
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bracket_checker() {
        let test1 = String::from("({})");
        let test2 = String::from("({[]})");
        let test3 = String::from("(){}[]");
        let test4 = String::from("([]){}");
        let test5 = String::from("({)}[]");

        assert!(bracket_checker(test1));
        assert!(bracket_checker(test2));
        assert!(bracket_checker(test3));
        assert!(bracket_checker(test4));
        assert!(!bracket_checker(test5));
    }

}
