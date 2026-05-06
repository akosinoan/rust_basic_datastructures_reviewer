// VECTORS - Exercise 1: Basics
//
// A Vec<T> is Rust's growable array. It lives on the heap.
// Key operations:
//   Vec::new()          — create empty
//   vec![1, 2, 3]       — create with values
//   v.push(x)           — append to end
//   v.pop()             — remove and return last element (Option<T>)
//   v.len()             — number of elements
//   v.is_empty()        — true if len == 0
//   v[i]                — index (panics if out of bounds)
//   v.get(i)            — safe index (returns Option<&T>)
//   v.first() / v.last()— Option<&T>

// --- Exercise 1a ---
// Build a Vec containing [1, 2, 3, 4, 5] using push(), then return it.
pub fn build_vec() -> Vec<i32> {
    todo!("Create an empty Vec and push 1 through 5 into it")
}

// --- Exercise 1b ---
// Return the last element of a Vec, or -1 if empty.
// HINT: use .pop() or .last()
pub fn last_element(mut v: Vec<i32>) -> i32 {
    todo!("Return the last element or -1 if the vec is empty")
}

// --- Exercise 1c ---
// Return the element at index `i` safely. Return None if out of bounds.
// HINT: v.get(i) returns Option<&T>. You need to copy the value out.
pub fn safe_get(v: &[i32], i: usize) -> Option<i32> {
    todo!("Use .get() to safely index, then copy the value with copied()")
}

// --- Exercise 1d ---
// Given a vec, remove all elements and return how many were removed.
// HINT: .len() before clearing, .clear() to empty it
pub fn drain_and_count(v: &mut Vec<i32>) -> usize {
    todo!("Count elements, clear the vec, return the count")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_vec() {
        assert_eq!(build_vec(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_last_element() {
        assert_eq!(last_element(vec![10, 20, 30]), 30);
        assert_eq!(last_element(vec![]), -1);
        assert_eq!(last_element(vec![42]), 42);
    }

    #[test]
    fn test_safe_get() {
        let v = vec![10, 20, 30];
        assert_eq!(safe_get(&v, 0), Some(10));
        assert_eq!(safe_get(&v, 2), Some(30));
        assert_eq!(safe_get(&v, 5), None);
    }

    #[test]
    fn test_drain_and_count() {
        let mut v = vec![1, 2, 3, 4];
        let count = drain_and_count(&mut v);
        assert_eq!(count, 4);
        assert!(v.is_empty());
    }
}
