// VECTORS - Exercise 3: Sorting & Searching
//
// Key operations:
//   v.sort()                    — sort in place (requires Ord)
//   v.sort_by(|a, b| a.cmp(b)) — custom comparator
//   v.sort_by_key(|x| ...)     — sort by derived key
//   v.binary_search(&x)        — O(log n) search on sorted vec → Result<usize, usize>
//   v.contains(&x)             — O(n) linear search → bool
//   v.dedup()                  — remove consecutive duplicates (sort first!)
//   v.windows(n)               — sliding windows of size n
//   v.chunks(n)                — non-overlapping chunks of size n

// --- Exercise 3a ---
// Sort `v` ascending and return it.
//
// Inputs:  v — an owned Vec<i32> (you may consume + mutate it).
// Returns: the same vec, now in non-decreasing order.
//
// Edge cases the tests check:
//   - random integers with duplicates → stable ascending order
pub fn sort_asc(mut v: Vec<i32>) -> Vec<i32> {
    todo!()
}

// --- Exercise 3b ---
// Sort `v` DESCENDING (largest first) and return it.
//
// Inputs:  v — an owned Vec<i32>.
// Returns: the same vec, now in non-increasing order.
//
// Edge cases the tests check:
//   - duplicate values appear consecutively in the output
pub fn sort_desc(mut v: Vec<i32>) -> Vec<i32> {
    todo!()
}

// --- Exercise 3c ---
// Sort `v` by string LENGTH, shortest first.
//
// Inputs:  v — an owned Vec<String>.
// Returns: same elements, ordered by .len() ascending. Tie-breaking order is unspecified.
//
// Edge cases the tests check:
//   - test asserts result[0] is the shortest and result[2] is the longest
//     (it does NOT pin tie-breaking, so any stable tie order is fine)
pub fn sort_by_length(mut v: Vec<String>) -> Vec<String> {
    todo!()
}

// --- Exercise 3d ---
// Return true if `target` exists in the SORTED slice `v`. Run in O(log n).
//
// Inputs:  v — a borrowed slice already sorted ascending; target — i32 to search for.
// Returns: true if target appears at any index, false otherwise.
//
// Edge cases the tests check:
//   - target in the middle / at edges → true
//   - target between two existing values → false
//   - target larger than every element  → false
pub fn binary_search(v: &[i32], target: i32) -> bool {
    todo!()
}

// --- Exercise 3e ---
// Return a Vec containing each distinct value of `v` exactly once.
//
// Inputs:  v — an owned Vec<i32> (you may sort/mutate it before returning).
// Returns: a Vec<i32> with no duplicates. The test sorts your output before
//          comparing, so the order of distinct values does not matter.
//
// Edge cases the tests check:
//   - vec with duplicates → no element appears twice in the output
pub fn remove_duplicates(mut v: Vec<i32>) -> Vec<i32> {
    todo!()
}

// --- Exercise 3f ---
// Return the maximum sum of any CONTIGUOUS sub-slice of `v` of length exactly `k`.
//
// Inputs:  v — a borrowed slice of i32; k — positive window length, k <= v.len().
// Returns: the largest sum found among all v.windows(k).
//
// Example:
//   v = [1, 3, -1, -3, 5, 3, 6, 7], k = 3
//   windows: [1,3,-1]=3, [3,-1,-3]=-1, [-1,-3,5]=1, [-3,5,3]=5, [5,3,6]=14, [3,6,7]=16
//   answer:  16
//
// Edge cases the tests check:
//   - window strictly inside the slice
//   - k equals v.len()  → only one window, return its sum
pub fn max_window_sum(v: &[i32], k: usize) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_asc() {
        assert_eq!(sort_asc(vec![3, 1, 4, 1, 5, 9]), vec![1, 1, 3, 4, 5, 9]);
    }

    #[test]
    fn test_sort_desc() {
        assert_eq!(sort_desc(vec![3, 1, 4, 1, 5]), vec![5, 4, 3, 1, 1]);
    }

    #[test]
    fn test_sort_by_length() {
        let input = vec!["banana".to_string(), "kiwi".to_string(), "fig".to_string()];
        let result = sort_by_length(input);
        assert_eq!(result[0], "fig");
        assert_eq!(result[2], "banana");
    }

    #[test]
    fn test_binary_search() {
        let v = vec![1, 3, 5, 7, 9];
        assert!(binary_search(&v, 5));
        assert!(!binary_search(&v, 4));
        assert!(binary_search(&v, 1));
        assert!(!binary_search(&v, 10));
    }

    #[test]
    fn test_remove_duplicates() {
        let mut result = remove_duplicates(vec![3, 1, 4, 1, 5, 3]);
        result.sort();
        assert_eq!(result, vec![1, 3, 4, 5]);
    }

    #[test]
    fn test_max_window_sum() {
        assert_eq!(max_window_sum(&[1, 3, -1, -3, 5, 3, 6, 7], 3), 16);
        assert_eq!(max_window_sum(&[2, 1, 5, 1, 3, 2], 3), 9);
        assert_eq!(max_window_sum(&[1, 2, 3], 3), 6);
    }
}
