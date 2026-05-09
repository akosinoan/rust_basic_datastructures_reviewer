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
// APPROACH (high-level): O(n) is achievable. The brute-force O(n²)
// would re-scan the slice for every element. Aim for one pass.
// ---------------------------------------------------------------

// --- two_sum ---
// Inputs:  nums — owned Vec<i32>; target — desired sum.
// Returns: Vec<i32> of length 2 with the two indices i, j (any order is
//          accepted by the tests as long as nums[i] + nums[j] == target;
//          the asserts below are written assuming i < j).
//
// Edge cases the tests check:
//   - [2,7,11,15] target 9        → [0,1]
//   - [3,2,4]     target 6        → [1,2]
//   - [3,3]       target 6        → [0,1]   (two distinct positions, same value)
//   - [-1,-2,-3,-4,-5] target -8  → [2,4]   (negative values)
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    todo!()
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
