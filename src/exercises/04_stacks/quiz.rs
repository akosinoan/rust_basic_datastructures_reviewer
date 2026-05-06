// QUIZ — Section 4: Expression Evaluator
//
// Implement three standalone functions that each use a Vec<T> as a stack.
// You will evaluate Reverse Polish Notation, count bracket mismatches, and
// compute the next-greater-element array with a monotonic stack — all patterns
// from this section.

// --- Quiz 4a ---
// Evaluate a Reverse Polish Notation (RPN) expression.
//
// Tokens are either integer strings ("3", "-7") or operators ("+","-","*","/").
// For each token:
//   • number  → parse and push onto the stack
//   • operator → pop b then a, push (a op b)
// Integer division truncates toward zero (Rust's default `/`).
// Guaranteed: valid expression, exactly one value remains at the end.
//
// HINT: use a Vec<i32> as the stack; parse integers with .parse::<i32>().unwrap()
pub fn eval_rpn(tokens: &[&str]) -> i32 {
    todo!("stack-based RPN evaluation")
}

// --- Quiz 4b ---
// Return the minimum number of bracket insertions needed to make s valid.
//
// A valid bracket string has matching '(' for every ')' and vice-versa.
// HINT: track `open` (unmatched '(') and `close` (unmatched ')') counters.
//   For '(': open += 1
//   For ')': if open > 0 { open -= 1 } else { close += 1 }
// Answer is open + close.
pub fn min_brackets_to_add(s: &str) -> usize {
    todo!("count unmatched '(' and ')' using two counters")
}

// --- Quiz 4c ---
// For each element, find the next element to its right that is strictly greater.
// If none exists, use -1.
//
// Example: [2, 1, 3, 5, 4] → [3, 3, 5, -1, -1]
//
// HINT: monotonic stack stores indices.
//   Iterate left-to-right. While the stack's top index points to a value
//   less than nums[i], pop it and set result[popped] = nums[i].
//   Push i. After the loop, remaining indices get -1.
pub fn next_greater_element(nums: &[i32]) -> Vec<i32> {
    todo!("monotonic stack of indices, fill result as you pop")
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
