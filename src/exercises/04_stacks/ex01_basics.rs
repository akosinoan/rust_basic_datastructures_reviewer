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
// Evaluate a postfix (Reverse Polish Notation) expression.
//
// Inputs:  tokens — borrowed slice of &str. Each token is either an integer
//          (e.g. "42", "-3") or one of the operators "+", "-", "*", "/".
//          Inputs are guaranteed valid (every operator finds two operands).
// Returns: i32 — the final value left on the stack.
//
// Example:
//   ["2","3","+","4","*"]   → (2+3)*4 = 20
//   ["2","1","+","3","*"]   → (2+1)*3 = 9
//   ["4","13","5","/","+"]  → 4 + (13/5) = 4 + 2 = 6   (integer division)
//
// Edge cases the tests check:
//   - mixed +, *, / (longer expression)
//   - operands of multiple digits ("13")
//   - negative-result intermediates (e.g. "10","6","9","3","+","-11","*", ...)
pub fn eval_rpn(tokens: &[&str]) -> i32 {
    todo!()
}

// --- Exercise 1b ---
// Build a stack of i32 that ALSO answers `get_min` in O(1).
//
// Two parallel Vecs are provided: `stack` for values, `min_stack` for the
// running minimum at each depth. Maintain both so popping/pushing always
// keeps min_stack.last() == min(stack[..]).
pub struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    // Construct an empty MinStack.
    //
    // Returns: MinStack with both internal Vecs empty.
    pub fn new() -> Self {
        todo!()
    }

    // Push `val` onto the stack. The min_stack must reflect the running minimum.
    //
    // Inputs:  val — any i32.
    // Returns: nothing. After the call, .top() returns val and .get_min() returns
    //          the new minimum among all currently-pushed values.
    pub fn push(&mut self, val: i32) {
        todo!()
    }

    // Pop the top of the stack. Tests don't require a return value.
    //
    // Note: tests assume there is always something to pop when this is called.
    pub fn pop(&mut self) {
        todo!()
    }

    // Peek the top value (the most recently pushed, currently still on the stack).
    //
    // Returns: i32. Tests only call this when the stack is non-empty.
    pub fn top(&self) -> i32 {
        todo!()
    }

    // Return the minimum of every value currently in the stack, in O(1).
    //
    // Returns: i32.
    //
    // Edge cases the tests check (after a sequence of pushes/pops):
    //   - push 5,3,7,1 → get_min == 1, then pop → get_min == 3, top == 7, pop → get_min == 3
    pub fn get_min(&self) -> i32 {
        todo!()
    }
}

// --- Exercise 1c ---
// Balanced-parentheses check using a counter or stack. ONLY '(' and ')' appear.
//
// Inputs:  s — borrowed &str.
// Returns: true iff every '(' has a matching ')' that closes AFTER it.
//
// Edge cases the tests check:
//   - "(())"   → true
//   - "()()"   → true
//   - ""       → true
//   - "(()"    → false (unmatched open)
//   - ")("     → false (close before open)
//   - ")"      → false
pub fn balanced_parens(s: &str) -> bool {
    todo!()
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
