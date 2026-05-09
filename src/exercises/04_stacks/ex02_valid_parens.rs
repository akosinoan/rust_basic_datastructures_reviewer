// STACKS - Exercise 2: Valid Parentheses
//
// LeetCode #20. This is the classic stack problem.
//
// Algorithm:
//   For each char in s:
//     if it's an opening bracket → push onto stack
//     if it's a closing bracket:
//       if stack is empty OR top doesn't match → return false
//       else → pop the top
//   At the end: return stack.is_empty()
//
// The key insight: a stack naturally tracks the NESTING of brackets.

// --- Exercise 2a ---
// Validate a string of bracket characters '(' ')' '[' ']' '{' '}'.
//
// Inputs:  s — borrowed &str (assumed to contain only the six bracket chars).
// Returns: true iff every closing bracket matches the most recent unmatched
//          opener of the same type, AND no openers remain unmatched at end.
//
// Edge cases the tests check:
//   - "()", "()[]{}", "{[()]}"  → true
//   - "(]", "([)]"              → false (wrong type / wrong nesting)
//   - ""                        → true
//   - "("                       → false (open never closed)
//   - "]"                       → false (close with empty stack)
pub fn is_valid(s: &str) -> bool {
    todo!()
}

// --- Exercise 2b ---
// LeetCode #921. Minimum number of single-char insertions so `s` becomes valid.
// Only '(' and ')' appear in `s`.
//
// Inputs:  s — borrowed &str of '(' and ')'.
// Returns: i32 — the minimum count of insertions needed (each insertion adds
//          one '(' or one ')' anywhere) to make every paren matched.
//
// Example:
//   "())"     → 1   (need one '(' to balance the trailing ')')
//   "((("     → 3   (need three ')')
//   "()"      → 0
//   "()))(("  → 4   (two ')' unmatched, two '(' unmatched, sum is 4)
pub fn min_add_to_make_valid(s: &str) -> i32 {
    todo!()
}

// --- Exercise 2c ---
// LeetCode #678. `s` contains '(', ')', and '*'. Each '*' MAY be treated as
// '(' or ')' or as an empty character. Return true iff some interpretation
// makes the string a valid parenthesis sequence.
//
// Inputs:  s — borrowed &str of '(', ')', '*'.
// Returns: bool.
//
// Edge cases the tests check:
//   - "()"      → true
//   - "(*)"     → true  ('*' as ')' or as nothing both work)
//   - "(*))"    → true  ('*' as '(')
//   - "((*"     → false (only one wildcard, two unmatched opens)
//   - "*"       → true  ('*' as nothing)
pub fn check_valid_string(s: &str) -> bool {
    todo!()
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
        assert!(!is_valid("]"));
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
