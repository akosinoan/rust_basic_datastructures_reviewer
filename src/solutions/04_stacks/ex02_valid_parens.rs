pub fn is_valid(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

pub fn min_add_to_make_valid(s: &str) -> i32 {
    let (mut open, mut close) = (0i32, 0i32);
    for c in s.chars() {
        if c == '(' {
            open += 1;
        } else if open > 0 {
            open -= 1;
        } else {
            close += 1;
        }
    }
    open + close
}

pub fn check_valid_string(s: &str) -> bool {
    let (mut lo, mut hi) = (0i32, 0i32);
    for c in s.chars() {
        match c {
            '(' => {
                lo += 1;
                hi += 1;
            }
            ')' => {
                lo = (lo - 1).max(0);
                hi -= 1;
                if hi < 0 {
                    return false;
                }
            }
            '*' => {
                lo = (lo - 1).max(0);
                hi += 1;
            }
            _ => {}
        }
    }
    lo == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(is_valid("()"));
        assert!(is_valid("()[]{}"));
        assert!(is_valid("{[()]}"));
        assert!(!is_valid("(]"));
        assert!(!is_valid("([)]"));
        assert!(is_valid(""));
        assert!(!is_valid("("));
    }

    #[test]
    fn test_min_add_to_make_valid() {
        assert_eq!(min_add_to_make_valid("())"), 1);
        assert_eq!(min_add_to_make_valid("((("), 3);
        assert_eq!(min_add_to_make_valid("()"), 0);
        assert_eq!(min_add_to_make_valid("()))(("), 4);
    }

    #[test]
    fn test_check_valid_string() {
        assert!(check_valid_string("()"));
        assert!(check_valid_string("(*)"));
        assert!(check_valid_string("(*))"));
        assert!(!check_valid_string("((*"));
        assert!(check_valid_string("*"));
    }
}
