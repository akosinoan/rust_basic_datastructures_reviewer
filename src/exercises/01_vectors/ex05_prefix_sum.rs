// VECTORS - Exercise 5: Prefix Sums
//
// A prefix sum array lets you answer range-sum queries in O(1)
// after O(n) preprocessing.
//
//   prefix[0] = 0
//   prefix[i] = prefix[i-1] + nums[i-1]
//
// Sum of nums[l..=r] = prefix[r+1] - prefix[l]
//
// This pattern appears constantly in LeetCode problems tagged
// "subarray", "range sum", "product except self".

// --- Exercise 5a ---
// Build the prefix-sum array of `nums`.
//
// Inputs:  nums — a borrowed slice of i32.
// Returns: a Vec<i32> of length nums.len() + 1, where:
//          result[0] == 0
//          result[i] == sum of nums[0..i]   (for i >= 1)
//
// Example:
//   nums = [1, 2, 3, 4]   →   result = [0, 1, 3, 6, 10]
//
// Edge cases the tests check:
//   - empty slice    → [0]
//   - single element → [0, nums[0]]
pub fn build_prefix_sum(nums: &[i32]) -> Vec<i32> {
    todo!()
}

// --- Exercise 5b ---
// Given a prefix-sum array, return the sum of nums[l..=r] (inclusive on both ends).
//
// Inputs:  prefix — the array built by build_prefix_sum;
//          l, r — usize indices with l <= r and r < (prefix.len() - 1).
// Returns: the integer sum of the original nums between l and r inclusive.
//
// Edge cases the tests check:
//   - l == 0 and r == last index   → sum of the whole array
//   - l == r                       → just that single element
pub fn range_sum(prefix: &[i32], l: usize, r: usize) -> i32 {
    todo!()
}

// --- Exercise 5c ---
// Return true if `nums` contains ANY contiguous subarray whose elements sum to `target`.
//
// Inputs:  nums — a borrowed slice; target — the desired sum.
// Returns: true iff there exist indices i <= j such that nums[i..=j].iter().sum() == target.
//          Aim for an O(n) solution.
//
// Edge cases the tests check:
//   - sum hits a strict middle slice    → e.g. [1,2,3] target 5 → true (the [2,3] slice)
//   - whole array sums to target        → e.g. [1,2,3] target 6 → true
//   - target unreachable                → false
pub fn subarray_sum_exists(nums: &[i32], target: i32) -> bool {
    todo!()
}

// --- Exercise 5d ---
// Return a Vec where result[i] is the product of every nums[j] for j != i.
// You may NOT use division.
//
// Inputs:  nums — a borrowed slice of i32.
// Returns: Vec<i32> of the same length, with result[i] == ∏(nums[j] for j != i).
//
// Example:
//   nums = [1, 2, 3, 4]   →   result = [24, 12, 8, 6]
//
// Edge cases the tests check:
//   - all 1s → all 1s
//   - several values, no zeros → element-wise product-except-self
pub fn product_except_self(nums: &[i32]) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_prefix_sum() {
        assert_eq!(build_prefix_sum(&[1, 2, 3, 4]), vec![0, 1, 3, 6, 10]);
        assert_eq!(build_prefix_sum(&[]), vec![0]);
        assert_eq!(build_prefix_sum(&[5]), vec![0, 5]);
    }

    #[test]
    fn test_range_sum() {
        let prefix = build_prefix_sum(&[1, 2, 3, 4, 5]);
        assert_eq!(range_sum(&prefix, 0, 4), 15); // whole array
        assert_eq!(range_sum(&prefix, 1, 3), 9);  // 2+3+4
        assert_eq!(range_sum(&prefix, 2, 2), 3);  // just 3
    }

    #[test]
    fn test_subarray_sum_exists() {
        assert!(subarray_sum_exists(&[1, 2, 3], 5));       // [2,3]
        assert!(subarray_sum_exists(&[1, 2, 3], 6));       // [1,2,3]
        assert!(!subarray_sum_exists(&[1, 2, 3], 7));
        assert!(subarray_sum_exists(&[-1, 2, 3, -2], 0));
        assert!(subarray_sum_exists(&[3, 1, 4, 1, 5], 10)); // [4,1,5]
    }

    #[test]
    fn test_product_except_self() {
        assert_eq!(product_except_self(&[1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(product_except_self(&[2, 3, 4, 5]), vec![60, 40, 30, 24]);
        assert_eq!(product_except_self(&[1, 1, 1, 1]), vec![1, 1, 1, 1]);
    }
}
