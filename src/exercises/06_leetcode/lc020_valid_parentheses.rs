// ============================================================
// LeetCode #20 — Valid Parentheses
// https://leetcode.com/problems/valid-parentheses/
// Difficulty: Easy
// ============================================================
//
// Given a string s containing '(', ')', '{', '}', '[', ']',
// determine if the input string is valid.
//
// Valid means:
//   - Open brackets must be closed by the same type.
//   - Open brackets must be closed in the correct order.
//   - Every close bracket has a corresponding open bracket.
//
// Example:
//   "()" → true
//   "()[]{}" → true
//   "(]" → false
//   "([)]" → false
//   "{[]}" → true
//
// ---------------------------------------------------------------
// APPROACH: Stack — O(n) time, O(n) space
//
// Push opening brackets. For each closing bracket,
// check if it matches the top of the stack. If not → invalid.
// At end: stack must be empty.
// ---------------------------------------------------------------

pub fn is_valid(s: String) -> bool {
    todo!(
        "Use Vec<char> as stack. Match ')' with '(', ']' with '[', '}}' with '{{'.\
         Return false if top doesn't match. Return stack.is_empty() at end."
    )
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
