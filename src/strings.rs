use std::collections::HashMap;

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

// Reverses a string in place
pub fn reverse_string(input: String) -> String {
    input.chars().rev().collect::<String>()
}

// Reverses the digits of an integer
pub fn reverse_integer(input: i32) -> i32 {
    match input < 0 {
        // if the input is negative, multiply it by -1 before collecting
        // the reversed digit chars into a String that can then be parsed back
        // into a signed 32 bit integer. Multiply that by -1 to get the result
        true => {
            let temp = (input * -1)
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            temp * -1
        }
        // otherwise, collect the reversed input chars into a String and parse back into i32
        false => input
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<i32>()
            .unwrap(),
    }
}

pub fn first_unique_character(input: String) -> i32 {
    // Allocate space for a vector with capacity == length of input String
    let mut mapped = HashMap::new();

    // use HashMap to create a counter that keeps track of
    // how often each char from the input String occurs
    for c in input.chars() {
        let counter = mapped.entry(c).or_insert(0);
        *counter += 1
    }

    // Enumerate the input chars so you have access to each char's index
    for (i, c) in input.chars().enumerate() {
        // Get from the mapped counter by using each char as a key
        match mapped.get(&c) {
            // if the key's value is 1, or rather, if the character only
            // occurs once in the input String, return that char's index
            Some(1) => return i as i32,
            // otherwise, continue until all input String chars are exhausted
            _ => continue,
        }
    }

    // If none of the characters are unique, return -1
    -1
}

// takes an input string and returns a hashmap counter of char occurrences
pub fn char_counter(a: String) -> HashMap<char, u32> {
    let mut mapped = HashMap::new();
    for c in a.chars() {
        let counter = mapped.entry(c).or_insert(0);
        *counter += 1
    }

    mapped
}

// Return true if a and b are anagrams of one another
pub fn is_anagram(a: String, b: String) -> bool {
    let mapped_a = char_counter(a);
    let mapped_b = char_counter(b);

    // for each key: value pair received from iterating over
    // the a String counter HashMap
    for (k, v_a) in mapped_a.iter() {
        // get the key from the b String counter
        match mapped_b.get(k) {
            // if the value from the b String counter == the value (count)
            // from the a String counter, continue
            Some(v_b) => {
                if v_b == v_a {
                    continue;
                }
            }
            // if they are different, the Strings are not anagrams -> return false
            _ => return false,
        }
    }

    // if all the chars occur at the same frequency in both counters, return true
    true
}

// function that checks if the input string is a valid palindrome
pub fn is_palindrome(input: String) -> bool {
    if input == String::from("") { return true };
    // first, clean the string so we only have lowercase alphanumeric characters
    let clean_string = input
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();

    // create a second String that is a reverse of the first String
    let rev_string = &clean_string.chars().rev().collect::<String>();

    // if the clean string and the reversed string are the same, return true
    clean_string == *rev_string
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

    #[test]
    fn test_string_reversal() {
        let test1 = String::from("Jello");
        let test2 = String::from("mellow");
        let test3 = String::from("purple");
        let test4 = String::from("");
        let test5 = String::from(" ");

        assert_eq!(reverse_string(test1), String::from("olleJ"));
        assert_eq!(reverse_string(test2), String::from("wollem"));
        assert_eq!(reverse_string(test3), String::from("elprup"));
        assert_eq!(reverse_string(test4), String::from(""));
        assert_eq!(reverse_string(test5), String::from(" "));
    }

    #[test]
    fn test_number_reversal() {
        let test1 = 32;
        let test2 = 42;
        let test3 = 1;
        let test4 = -20;
        let test5 = -30001;

        assert_eq!(reverse_integer(test1), 23);
        assert_eq!(reverse_integer(test2), 24);
        assert_eq!(reverse_integer(test3), 1);
        assert_eq!(reverse_integer(test4), -2);
        assert_eq!(reverse_integer(test5), -10003);
    }

    #[test]
    fn test_unique_char() {
        let test1 = String::from("foobar");
        let test2 = String::from("bazbar");
        let test3 = String::from("loopzoop");
        let test4 = String::from("zzzzzz");
        let test5 = String::from("zzzzzzzooom");

        assert_eq!(first_unique_character(test1), 0);
        assert_eq!(first_unique_character(test2), 2);
        assert_eq!(first_unique_character(test3), 0);
        assert_eq!(first_unique_character(test4), -1);
        assert_eq!(first_unique_character(test5), 10);
    }

    #[test]
    fn test_anagram() {
        let test1 = (String::from("loop"), String::from("pool"));
        let test2 = (String::from("zing"), String::from("zang"));
        let test3 = (String::from("gooloog"), String::from("loogoog"));
        let test4 = (String::from("al"), String::from("la"));
        let test5 = (String::from("po"), String::from("p"));

        assert!(is_anagram(test1.0, test1.1));
        assert!(!is_anagram(test2.0, test2.1));
        assert!(is_anagram(test3.0, test3.1));
        assert!(is_anagram(test4.0, test4.1));
        assert!(!is_anagram(test5.0, test5.1));
    }

    #[test]
    fn test_palindrome() {
        let test1 = String::from("ado://oda");
        let test2 = String::from("123zing gniz321");
        let test3 = String::from(":laBodO /a/  odobal:");
        let test4 = String::from("zoop loop");
        let test5 = String::from("");

        assert!(is_palindrome(test1));
        assert!(is_palindrome(test2));
        assert!(is_palindrome(test3));
        assert!(!is_palindrome(test4));
        assert!(is_palindrome(test5));

    }
}
