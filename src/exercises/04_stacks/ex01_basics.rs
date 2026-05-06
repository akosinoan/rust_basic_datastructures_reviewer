// STACKS - Exercise 1: Vec as Stack
//
// Rust has no dedicated Stack type. Vec<T> is the stack:
//   v.push(x)     — push onto top
//   v.pop()       — pop from top (Option<T>)
//   v.last()      — peek top (Option<&T>)
//   v.is_empty()  — check if empty
//   v.len()       — height of stack
//
// Stack property: LIFO — Last In, First Out.

// --- Exercise 1a ---
// Implement a simple postfix (RPN) calculator.
// Tokens are either numbers or operators: "+", "-", "*", "/"
// Example: ["2", "3", "+", "4", "*"] → (2+3)*4 = 20
// Rules: push numbers, pop two for operators, push result.
pub fn eval_rpn(tokens: &[&str]) -> i32 {
    todo!(
        "Use a Vec<i32> as stack. For each token:\
         if it parses as a number, push it.\
         Otherwise pop two values (b then a), apply the operator, push the result."
    )
}

// --- Exercise 1b ---
// Implement a stack that also tracks the minimum element in O(1).
// You'll build a struct with two vecs: one for values, one for running minimums.
pub struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        todo!("Return MinStack with two empty Vecs")
    }

    pub fn push(&mut self, val: i32) {
        todo!(
            "Push val onto stack. Push min(val, current_min) onto min_stack.\
             If min_stack is empty, push val as the first minimum."
        )
    }

    pub fn pop(&mut self) {
        todo!("Pop from both stacks")
    }

    pub fn top(&self) -> i32 {
        todo!("Return last element of stack")
    }

    pub fn get_min(&self) -> i32 {
        todo!("Return last element of min_stack")
    }
}

// --- Exercise 1c ---
// Check if a string of digits and operators is balanced using a stack.
// Rule: each '(' must be closed by ')' in correct order.
// (This is a preview — covered more deeply in ex02.)
pub fn balanced_parens(s: &str) -> bool {
    todo!(
        "Use a counter or stack. Increment for '(', decrement for ')'.\
         If counter goes negative, return false. At end, return counter == 0."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_rpn() {
        assert_eq!(eval_rpn(&["2", "1", "+", "3", "*"]), 9);
        assert_eq!(eval_rpn(&["4", "13", "5", "/", "+"]), 6);
        assert_eq!(eval_rpn(&["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]), 22);
    }

    #[test]
    fn test_min_stack() {
        let mut ms = MinStack::new();
        ms.push(5);
        ms.push(3);
        ms.push(7);
        ms.push(1);
        assert_eq!(ms.get_min(), 1);
        ms.pop();
        assert_eq!(ms.get_min(), 3);
        assert_eq!(ms.top(), 7);
        ms.pop();
        assert_eq!(ms.get_min(), 3);
    }

    #[test]
    fn test_balanced_parens() {
        assert!(balanced_parens("(())"));
        assert!(balanced_parens("()()"));
        assert!(balanced_parens(""));
        assert!(!balanced_parens("(()"));
        assert!(!balanced_parens(")("));
        assert!(!balanced_parens(")"));
    }
}
