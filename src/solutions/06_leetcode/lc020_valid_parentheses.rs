pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        if let Some(opener) = matching_opener(c) {
            if stack.pop() != Some(opener) {
                return false;
            }
        } else if matches!(c, '(' | '[' | '{') {
            stack.push(c);
        }
    }
    stack.is_empty()
}

fn matching_opener(closer: char) -> Option<char> {
    match closer {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() { assert!(is_valid("()".to_string())); }

    #[test]
    fn example_2() { assert!(is_valid("()[]{}".to_string())); }

    #[test]
    fn example_3() { assert!(!is_valid("(]".to_string())); }

    #[test]
    fn nested_valid() { assert!(is_valid("{[()]}".to_string())); }

    #[test]
    fn nested_invalid() { assert!(!is_valid("([)]".to_string())); }

    #[test]
    fn empty() { assert!(is_valid("".to_string())); }

    #[test]
    fn single_open() { assert!(!is_valid("(".to_string())); }

    #[test]
    fn single_close() { assert!(!is_valid("]".to_string())); }
}
