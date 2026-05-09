// ============================================================
// LeetCode #217 — Contains Duplicate
// https://leetcode.com/problems/contains-duplicate/
// Difficulty: Easy
// ============================================================
//
// Given an integer array, return true if any value appears
// at least twice. Return false if every element is distinct.
//
// Example:
//   [1,2,3,1] → true
//   [1,2,3,4] → false
//
// Three approaches are exercised, in increasing order of efficiency.
// You implement all three so you can compare them — the tests treat
// them as separate functions.
//   1. Brute force      O(n²) time, O(1) space
//   2. Sort + scan      O(n log n) time, O(1) extra space (sort in place)
//   3. HashSet (best)   O(n) time, O(n) space

// --- contains_duplicate_brute ---
// O(n²) brute force.
//
// Inputs:  nums — borrowed slice.
// Returns: true iff any pair of distinct positions holds equal values.
//
// Edge cases the tests check:
//   - [1,2,3,1] → true
//   - [1,2,3,4] → false
//   - [1]       → false (a single element can't duplicate itself)
pub fn contains_duplicate_brute(nums: &[i32]) -> bool {
    todo!()
}

// --- contains_duplicate_sort ---
// O(n log n) approach: sort, then look for adjacent equal values.
//
// Inputs:  nums — owned Vec<i32> (you may sort it in place).
// Returns: true iff any duplicate exists in nums.
//
// Edge cases the tests check:
//   - [1,2,3,1] sorted → adjacent dupes → true
//   - [1,2,3,4]        → false
//   - long list with duplicates anywhere → true
pub fn contains_duplicate_sort(mut nums: Vec<i32>) -> bool {
    todo!()
}

// --- contains_duplicate ---
// O(n) optimal approach using a hash-based set.
//
// Inputs:  nums — owned Vec<i32>.
// Returns: true iff any duplicate exists.
//
// Edge cases the tests check (same shape as the other two approaches).
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brute() {
        assert!(contains_duplicate_brute(&[1, 2, 3, 1]));
        assert!(!contains_duplicate_brute(&[1, 2, 3, 4]));
        assert!(!contains_duplicate_brute(&[1]));
    }

    #[test]
    fn test_sort() {
        assert!(contains_duplicate_sort(vec![1, 2, 3, 1]));
        assert!(!contains_duplicate_sort(vec![1, 2, 3, 4]));
        assert!(contains_duplicate_sort(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }

    #[test]
    fn test_hashset() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
        assert!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }
}
