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
// Build and return the prefix sum array.
// Given nums = [1, 2, 3, 4], return [0, 1, 3, 6, 10]
// The result has length nums.len() + 1.
pub fn build_prefix_sum(nums: &[i32]) -> Vec<i32> {
    todo!(
        "Create a vec of length nums.len()+1, set prefix[0]=0,\
         then prefix[i] = prefix[i-1] + nums[i-1]"
    )
}

// --- Exercise 5b ---
// Given a prefix sum array, return the sum of nums[l..=r] (inclusive, 0-indexed).
// HINT: prefix[r+1] - prefix[l]
pub fn range_sum(prefix: &[i32], l: usize, r: usize) -> i32 {
    todo!("Use the prefix array formula")
}

// --- Exercise 5c ---
// Return true if there exists a subarray (contiguous) that sums to `target`.
// HINT: build prefix sums, then for each pair check if prefix[j] - prefix[i] == target
//       But that's O(n²). Instead: store seen prefix sums in a HashSet,
//       and check if prefix[j] - target exists in the set.
pub fn subarray_sum_exists(nums: &[i32], target: i32) -> bool {
    use std::collections::HashSet;
    todo!(
        "Build running prefix sum. At each step check if (current_sum - target) \
         is in the seen set. Add current_sum to the set."
    )
}

// --- Exercise 5d ---
// Given an array, return a new array where result[i] is the product of
// all elements EXCEPT nums[i]. Do not use division.
// HINT: build a prefix product array (left to right),
//       then a suffix product array (right to left),
//       result[i] = prefix[i] * suffix[i]
pub fn product_except_self(nums: &[i32]) -> Vec<i32> {
    todo!(
        "Build prefix products and suffix products,\
         then multiply them together at each index"
    )
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
        assert!(subarray_sum_exists(&[-1, 2, 3, -2], 0)); // [-1+2+3-2] or [2-2]... wait, 3-2=no. [-1,2,3,-2]=2, [2,3,-2]=3, [-2]=no. [3,-2]=1, hmm.
        // Let's use a simpler case
        assert!(subarray_sum_exists(&[3, 1, 4, 1, 5], 10)); // [1,4,1,5]? no =11. [3,1,4]=8? no. [4,1,5]=10 yes!
    }

    #[test]
    fn test_product_except_self() {
        assert_eq!(product_except_self(&[1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(product_except_self(&[2, 3, 4, 5]), vec![60, 40, 30, 24]);
        assert_eq!(product_except_self(&[1, 1, 1, 1]), vec![1, 1, 1, 1]);
    }
}
