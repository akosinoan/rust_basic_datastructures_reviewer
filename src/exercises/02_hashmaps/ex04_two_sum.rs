// HASHMAPS - Exercise 4: Two Sum (HashMap approach)
//
// This is LeetCode #1. The naive brute force is O(n²).
// The HashMap approach is O(n):
//
//   For each element nums[i]:
//     complement = target - nums[i]
//     if complement is in map → found the pair!
//     else → store map[nums[i]] = i
//
// Key insight: we're trading space for time. Instead of searching
// the array for the complement, we look it up in O(1).

use std::collections::HashMap;

// --- Exercise 4a ---
// Return the two indices that add up to target.
// Each input has exactly one solution. You may not use the same element twice.
pub fn two_sum(nums: &[i32], target: i32) -> (usize, usize) {
    todo!(
        "Iterate with index, compute complement = target - nums[i],\
         check the map, then insert nums[i] -> i into the map"
    )
}

// --- Exercise 4b ---
// Return TRUE if any two distinct elements sum to target.
// (Like two_sum but only needs a boolean.)
pub fn has_pair_with_sum(nums: &[i32], target: i32) -> bool {
    todo!("Same approach as two_sum but just return bool")
}

// --- Exercise 4c ---
// Return all unique pairs (a, b) where a < b and a + b == target.
// Each pair should appear only once.
pub fn all_pairs_with_sum(nums: &[i32], target: i32) -> Vec<(i32, i32)> {
    todo!(
        "Use a HashSet to track seen numbers. For each num, check if (target-num)\
         is already in the set. Collect as (min, max) tuples to avoid duplicates."
    )
}

// --- Exercise 4d ---
// Three Sum: find all unique triplets that sum to zero.
// Hint: sort the array, fix one element, then use two pointers for the remaining pair.
// This combines sorting + two pointers + HashMap deduplication.
pub fn three_sum(nums: &mut Vec<i32>) -> Vec<Vec<i32>> {
    todo!(
        "Sort nums. For each i, use two pointers left=i+1, right=end.\
         Skip duplicates by checking if nums[i] == nums[i-1].\
         If sum < 0 move left right, if sum > 0 move right left, else record and skip dupes."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let r = two_sum(&[2, 7, 11, 15], 9);
        assert_eq!(r, (0, 1));

        let r2 = two_sum(&[3, 2, 4], 6);
        assert_eq!(r2, (1, 2));

        let r3 = two_sum(&[3, 3], 6);
        assert_eq!(r3, (0, 1));
    }

    #[test]
    fn test_has_pair_with_sum() {
        assert!(has_pair_with_sum(&[1, 2, 3, 4], 5));
        assert!(!has_pair_with_sum(&[1, 2, 3, 4], 8));
        assert!(has_pair_with_sum(&[0, 0], 0));
    }

    #[test]
    fn test_all_pairs_with_sum() {
        let mut pairs = all_pairs_with_sum(&[1, 2, 3, 4, 5], 6);
        pairs.sort();
        assert_eq!(pairs, vec![(1, 5), (2, 4)]);

        let pairs2 = all_pairs_with_sum(&[1, 2, 3], 10);
        assert_eq!(pairs2, vec![]);
    }

    #[test]
    fn test_three_sum() {
        let mut nums = vec![-1, 0, 1, 2, -1, -4];
        let mut result = three_sum(&mut nums);
        result.sort();
        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);

        let mut nums2 = vec![0, 0, 0];
        assert_eq!(three_sum(&mut nums2), vec![vec![0, 0, 0]]);

        let mut nums3 = vec![1, 2, 3];
        assert_eq!(three_sum(&mut nums3), vec![] as Vec<Vec<i32>>);
    }
}
