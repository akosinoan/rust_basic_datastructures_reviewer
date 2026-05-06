pub fn eval_rpn(tokens: &[&str]) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    for &token in tokens {
        match token {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            n => stack.push(n.parse().unwrap()),
        }
    }
    stack[0]
}

pub struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        let min = self.min_stack.last().copied().unwrap_or(i32::MAX).min(val);
        self.min_stack.push(min);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
        self.min_stack.pop();
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

pub fn balanced_parens(s: &str) -> bool {
    let mut count: i32 = 0;
    for c in s.chars() {
        match c {
            '(' => count += 1,
            ')' => {
                count -= 1;
                if count < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    count == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_rpn() {
        assert_eq!(eval_rpn(&["2", "1", "+", "3", "*"]), 9);
        assert_eq!(eval_rpn(&["4", "13", "5", "/", "+"]), 6);
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
    }

    #[test]
    fn test_balanced_parens() {
        assert!(balanced_parens("(())"));
        assert!(balanced_parens("()()"));
        assert!(balanced_parens(""));
        assert!(!balanced_parens("(()"));
        assert!(!balanced_parens(")("));
    }
}
