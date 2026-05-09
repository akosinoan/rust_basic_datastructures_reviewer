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
// Build and return a Vec<i32> containing the values 1, 2, 3, 4, 5 in that order.
//
// Inputs:  (none)
// Returns: Vec<i32> with exactly five elements [1, 2, 3, 4, 5].
//
// Edge cases the tests check:
//   - the returned vec equals vec![1, 2, 3, 4, 5] exactly (order matters).
pub fn build_vec() -> Vec<i32> {
    todo!()
}

// --- Exercise 1b ---
// Return the LAST element of `v`, or `-1` if the vec is empty.
//
// Inputs:  v — an owned Vec<i32> (you may consume it).
// Returns: the value at the last index, or -1 as a sentinel for empty input.
//
// Edge cases the tests check:
//   - non-empty vec        → the value at the last index
//   - empty vec            → -1
//   - single-element vec   → that one element
pub fn last_element(v: &[i32]) -> i32 {
    todo!()
}

// --- Exercise 1c ---
// Return the element at index `i` safely. Out-of-bounds must NOT panic.
//
// Inputs:  v — a borrowed slice of i32; i — a usize index.
// Returns: Some(value) if 0 <= i < v.len(), else None.
//
// Edge cases the tests check:
//   - first index (i == 0)            → Some(v[0])
//   - last in-bounds index            → Some(v[len-1])
//   - i >= v.len()                    → None (do not panic)
pub fn safe_get(v: &[i32], i: usize) -> Option<i32> {
    todo!()
}

// --- Exercise 1d ---
// Empty `v` in place and return how many elements were removed.
//
// Inputs:  v — a unique reference to a Vec<i32> you must mutate.
// Returns: the number of elements that existed BEFORE the function ran.
//          After the call, v.is_empty() must be true.
//
// Edge cases the tests check:
//   - non-empty vec  → returns prior length, vec is now empty
//   - empty vec      → returns 0, vec is still empty
pub fn drain_and_count(v: &mut Vec<i32>) -> usize {
    todo!()
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
        let (v1, v2, v3) = (vec![10, 20, 30], vec![], vec![42]);
        assert_eq!(last_element(&v1), 30);
        assert_eq!(last_element(&v2), -1);
        assert_eq!(last_element(&v3), 42);
    }

    #[test]
    fn test_safe_get() {
        let v: Vec<i32> = vec![10, 20, 30];
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
