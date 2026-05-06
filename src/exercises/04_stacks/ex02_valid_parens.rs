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
// Return true if the string of brackets is valid.
// Valid bracket types: () [] {}
pub fn is_valid(s: &str) -> bool {
    todo!(
        "Use a Vec<char> as stack. Match each closing bracket against the top.\
         Use a match or helper to find the expected opener for each closer."
    )
}

// --- Exercise 2b ---
// Return the minimum number of parentheses to add to make the string valid.
// Only '(' and ')' characters are in the string.
// LeetCode #921.
// HINT: track open count (unmatched '(') and close count (unmatched ')').
//       For each '(': open += 1
//       For each ')': if open > 0: open -= 1 (matched!) else close += 1 (unmatched)
//       Return open + close
pub fn min_add_to_make_valid(s: &str) -> i32 {
    todo!(
        "Track unmatched open and close counts. Return their sum."
    )
}

// --- Exercise 2c ---
// Given a string with '(', ')', and '*' where '*' can be '(', ')' or empty,
// return true if the string can be valid. LeetCode #678.
// HINT: track a range [lo, hi] of possible unmatched '(' counts.
//       '(' → both lo and hi increase
//       ')' → both decrease (lo = max(lo-1, 0), hi -= 1)
//       '*' → lo decreases (could be ')'), hi increases (could be '(')
//       If hi < 0 at any point → false. At end: lo == 0.
pub fn check_valid_string(s: &str) -> bool {
    todo!(
        "Track lo and hi (range of open parens). \
         Return false if hi < 0, return lo == 0 at end."
    )
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
