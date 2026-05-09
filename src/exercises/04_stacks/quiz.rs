// QUIZ — Section 4: Expression Evaluator
//
// Implement three standalone functions that each use a Vec<T> as a stack.
// You will evaluate Reverse Polish Notation, count bracket mismatches, and
// compute the next-greater-element array with a monotonic stack — all patterns
// from this section.

// --- Quiz 4a ---
// Evaluate a Reverse Polish Notation (RPN) expression and return its value.
//
// Inputs:  tokens — borrowed slice of &str. Each token is either:
//                  - an integer string ("3", "-7"), or
//                  - one of "+", "-", "*", "/".
//          Integer division truncates toward zero (Rust's default for i32 `/`).
//          Inputs are guaranteed valid; exactly one value remains at the end.
// Returns: i32.
//
// Examples:
//   ["2","3","+"]                       → 5
//   ["10","3","-"]                      → 7
//   ["6","2","/"]                       → 3
//   ["7","2","/"]                       → 3   (truncates toward zero)
//   ["-3","4","+"]                      → 1   (negative operand)
//   ["5","1","2","+","4","*","+","3","-"] → 14
pub fn eval_rpn(tokens: &[&str]) -> i32 {
    todo!()
}

// --- Quiz 4b ---
// Minimum number of bracket insertions needed to make `s` valid.
// Only '(' and ')' appear.
//
// Inputs:  s — borrowed &str.
// Returns: usize — the count.
//
// Examples:
//   "()", "(())", "()()"      → 0   (already valid)
//   "(("                      → 2   (two '(' need ')')
//   "(()"                     → 1
//   "))"                      → 2
//   "())"                     → 1
//   ")("                      → 2   (')' unmatched + '(' unmatched)
pub fn min_brackets_to_add(s: &str) -> usize {
    todo!()
}

// --- Quiz 4c ---
// Next-greater-element to the right (strictly greater). Return -1 if none.
//
// Inputs:  nums — borrowed slice of i32.
// Returns: Vec<i32> of the same length as nums. Aim for O(n).
//
// Examples:
//   [2, 1, 3, 5, 4] → [3, 3, 5, -1, -1]
//   [5, 4, 3, 2, 1] → [-1, -1, -1, -1, -1]
//   [1, 2, 3, 4]    → [2, 3, 4, -1]
//   [7]             → [-1]
pub fn next_greater_element(nums: &[i32]) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_rpn_basic() {
        assert_eq!(eval_rpn(&["2", "3", "+"]), 5);
        assert_eq!(eval_rpn(&["5", "1", "2", "+", "4", "*", "+", "3", "-"]), 14);
    }

    #[test]
    fn test_eval_rpn_subtraction() {
        assert_eq!(eval_rpn(&["10", "3", "-"]), 7);
    }

    #[test]
    fn test_eval_rpn_division() {
        assert_eq!(eval_rpn(&["6", "2", "/"]), 3);
        assert_eq!(eval_rpn(&["7", "2", "/"]), 3); // truncates toward zero
    }

    #[test]
    fn test_eval_rpn_negative() {
        assert_eq!(eval_rpn(&["-3", "4", "+"]), 1);
    }

    #[test]
    fn test_min_brackets_already_valid() {
        assert_eq!(min_brackets_to_add("()"), 0);
        assert_eq!(min_brackets_to_add("(())"), 0);
        assert_eq!(min_brackets_to_add("()()"), 0);
    }

    #[test]
    fn test_min_brackets_unmatched_open() {
        assert_eq!(min_brackets_to_add("(("), 2);
        assert_eq!(min_brackets_to_add("(()"), 1);
    }

    #[test]
    fn test_min_brackets_unmatched_close() {
        assert_eq!(min_brackets_to_add("))"), 2);
        assert_eq!(min_brackets_to_add("())"), 1);
    }

    #[test]
    fn test_min_brackets_mixed() {
        assert_eq!(min_brackets_to_add(")("), 2);
    }

    #[test]
    fn test_next_greater_element() {
        assert_eq!(next_greater_element(&[2, 1, 3, 5, 4]), vec![3, 3, 5, -1, -1]);
    }

    #[test]
    fn test_next_greater_element_descending() {
        assert_eq!(next_greater_element(&[5, 4, 3, 2, 1]), vec![-1, -1, -1, -1, -1]);
    }

    #[test]
    fn test_next_greater_element_ascending() {
        assert_eq!(next_greater_element(&[1, 2, 3, 4]), vec![2, 3, 4, -1]);
    }

    #[test]
    fn test_next_greater_element_single() {
        assert_eq!(next_greater_element(&[7]), vec![-1]);
    }
}
