// VECTORS - Exercise 2: Iteration
//
// Rust iterators are lazy and composable. Key methods:
//   .iter()             — iterate by reference (&T)
//   .into_iter()        — iterate by value (consumes the vec)
//   .iter_mut()         — iterate by mutable reference (&mut T)
//   .enumerate()        — yields (index, &value) pairs
//   .map(|x| ...)       — transform each element
//   .filter(|x| ...)    — keep elements matching predicate
//   .sum::<i32>()       — sum all elements
//   .collect::<Vec<_>>()— turn iterator back into a collection
//   .any(|x| ...)       — true if any element matches
//   .all(|x| ...)       — true if all elements match
//   .position(|x| ...) — index of first match (Option<usize>)

// --- Exercise 2a ---
// Return the sum of every element in `v`.
//
// Inputs:  v — a borrowed slice of i32.
// Returns: the integer total of all elements.
//
// Edge cases the tests check:
//   - several positives        → ordinary sum
//   - empty slice              → 0
//   - mix of positive/negative → values cancel correctly (e.g. [-1, 1] → 0)
pub fn sum(v: &[i32]) -> i32 {
    todo!()
}

// --- Exercise 2b ---
// Return a NEW Vec where every element of `v` is doubled.
//
// Inputs:  v — a borrowed slice of i32 (left untouched).
// Returns: Vec<i32> of the same length with each value multiplied by 2.
//
// Edge cases the tests check:
//   - non-empty slice → element-wise double
//   - empty slice     → empty vec
pub fn double_all(v: &[i32]) -> Vec<i32> {
    todo!()
}

// --- Exercise 2c ---
// Return only the EVEN values from `v`, preserving order.
//
// Inputs:  v — a borrowed slice of i32.
// Returns: Vec<i32> containing exactly the elements x where x % 2 == 0.
//
// Edge cases the tests check:
//   - mixed even and odd values → only the evens, in original order
//   - all odd values            → empty vec
pub fn keep_evens(v: &[i32]) -> Vec<i32> {
    todo!()
}

// --- Exercise 2d ---
// Return the index of the FIRST negative number in `v`, or None if there is none.
//
// Inputs:  v — a borrowed slice of i32.
// Returns: Some(i) where i is the smallest index with v[i] < 0, else None.
//
// Edge cases the tests check:
//   - first negative is in the middle  → that index
//   - all elements non-negative        → None
//   - the very first element is negative → Some(0)
pub fn first_negative_index(v: &[i32]) -> Option<usize> {
    todo!()
}

// --- Exercise 2e ---
// Concatenate every String in `v` into one new String, in order, with no separator.
//
// Inputs:  v — a borrowed slice of String.
// Returns: a single String formed by joining every element back-to-back.
//
// Edge cases the tests check:
//   - ["foo","bar","baz"] → "foobarbaz"
//   - empty slice         → ""
pub fn concat_all(v: &[String]) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum(&[]), 0);
        assert_eq!(sum(&[-1, 1]), 0);
    }

    #[test]
    fn test_double_all() {
        assert_eq!(double_all(&[1, 2, 3]), vec![2, 4, 6]);
        assert_eq!(double_all(&[]), vec![]);
    }

    #[test]
    fn test_keep_evens() {
        assert_eq!(keep_evens(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
        assert_eq!(keep_evens(&[1, 3, 5]), vec![]);
    }

    #[test]
    fn test_first_negative_index() {
        assert_eq!(first_negative_index(&[1, 2, -3, 4]), Some(2));
        assert_eq!(first_negative_index(&[1, 2, 3]), None);
        assert_eq!(first_negative_index(&[-1]), Some(0));
    }

    #[test]
    fn test_concat_all() {
        let v = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];
        assert_eq!(concat_all(&v), "foobarbaz");
        assert_eq!(concat_all(&[]), "");
    }
}
