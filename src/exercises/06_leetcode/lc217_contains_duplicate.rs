// ============================================================
// LeetCode #217 — Contains Duplicate
// https://leetcode.com/problems/contains-duplicate/
// Difficulty: Easy
// ============================================================
//
// Given an integer array, return true if any value appears
// at least twice. Return false if all elements are distinct.
//
// Example:
//   [1,2,3,1] → true
//   [1,2,3,4] → false
//
// ---------------------------------------------------------------
// THREE APPROACHES — implement all three to compare:
//
// 1. Brute Force    O(n²) time, O(1) space — nested loops
// 2. Sort           O(n log n) time, O(1) space — adjacent duplicates
// 3. HashSet        O(n) time, O(n) space — fastest
// ---------------------------------------------------------------

// Approach 1: Brute force
pub fn contains_duplicate_brute(nums: &[i32]) -> bool {
    todo!("Two nested loops, compare every pair")
}

// Approach 2: Sort then check adjacent
pub fn contains_duplicate_sort(mut nums: Vec<i32>) -> bool {
    todo!("Sort nums, then check if any adjacent pair is equal using .windows(2)")
}

// Approach 3: HashSet (optimal)
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;
    todo!("Insert each element; if insert returns false (already present), return true")
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
