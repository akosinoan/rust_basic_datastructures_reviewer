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

// --- is_valid ---
// Inputs:  s — owned String of bracket characters only.
// Returns: bool. The empty string is considered valid.
//
// Edge cases the tests check:
//   - "()", "()[]{}"        → true
//   - "{[()]}"              → true (nested correctly)
//   - "(]"                  → false (mismatched type)
//   - "([)]"                → false (interleaved, broken nesting)
//   - ""                    → true
//   - "("                   → false (unclosed)
//   - "]"                   → false (close-with-empty-stack)
pub fn is_valid(s: String) -> bool {
    todo!()
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
