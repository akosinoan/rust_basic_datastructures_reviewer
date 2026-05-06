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
// Return the vec sorted in ascending order.
// NOTE: you receive ownership, sort it, return it.
pub fn sort_asc(mut v: Vec<i32>) -> Vec<i32> {
    todo!("Sort the vec in ascending order")
}

// --- Exercise 3b ---
// Return the vec sorted in descending order.
// HINT: sort_by with b.cmp(a) reverses the order
pub fn sort_desc(mut v: Vec<i32>) -> Vec<i32> {
    todo!("Sort the vec in descending order")
}

// --- Exercise 3c ---
// Sort a vec of strings by their length (shortest first).
pub fn sort_by_length(mut v: Vec<String>) -> Vec<String> {
    todo!("Sort strings by length using sort_by_key")
}

// --- Exercise 3d ---
// Given a SORTED vec, return true if target exists using binary search.
// HINT: binary_search returns Ok(index) if found, Err(_) if not
pub fn binary_search(v: &[i32], target: i32) -> bool {
    todo!("Use v.binary_search(&target).is_ok()")
}

// --- Exercise 3e ---
// Return a vec with all duplicates removed. Order does not matter.
// HINT: sort first, then dedup()
pub fn remove_duplicates(mut v: Vec<i32>) -> Vec<i32> {
    todo!("Sort then dedup to remove all duplicates")
}

// --- Exercise 3f ---
// Return the maximum sum of any contiguous subarray of size `k`.
// HINT: use .windows(k) to iterate all windows of that size
pub fn max_window_sum(v: &[i32], k: usize) -> i32 {
    todo!("Use .windows(k) and find the max sum across all windows")
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
