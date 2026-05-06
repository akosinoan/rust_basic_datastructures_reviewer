// ============================================================
// LeetCode #1 — Two Sum
// https://leetcode.com/problems/two-sum/
// Difficulty: Easy
// ============================================================
//
// Given an array of integers `nums` and an integer `target`,
// return the indices of the two numbers such that they add up to target.
//
// You may assume each input has exactly one solution.
// You may not use the same element twice.
//
// Example:
//   Input: nums = [2,7,11,15], target = 9
//   Output: [0,1]  (because nums[0] + nums[1] = 9)
//
// Constraints:
//   2 <= nums.len() <= 10^4
//   -10^9 <= nums[i] <= 10^9
//   Exactly one valid answer exists.
//
// ---------------------------------------------------------------
// APPROACH: HashMap — O(n) time, O(n) space
//
// For each nums[i], compute complement = target - nums[i].
// If complement is already in the map, we found the pair.
// Otherwise, store nums[i] → i in the map for future lookups.
// ---------------------------------------------------------------

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    todo!("Implement the O(n) HashMap solution")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn example_2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn example_3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(two_sum(vec![-1, -2, -3, -4, -5], -8), vec![2, 4]);
    }
}
