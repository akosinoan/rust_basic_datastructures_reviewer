pub fn build_prefix_sum(nums: &[i32]) -> Vec<i32> {
    let mut prefix = vec![0; nums.len() + 1];
    for i in 0..nums.len() {
        prefix[i + 1] = prefix[i] + nums[i];
    }
    prefix
}

pub fn range_sum(prefix: &[i32], l: usize, r: usize) -> i32 {
    prefix[r + 1] - prefix[l]
}

pub fn subarray_sum_exists(nums: &[i32], target: i32) -> bool {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    seen.insert(0);
    let mut current = 0;
    for &n in nums {
        current += n;
        if seen.contains(&(current - target)) {
            return true;
        }
        seen.insert(current);
    }
    false
}

pub fn product_except_self(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let mut prefix = vec![1; n];
    let mut suffix = vec![1; n];

    for i in 1..n {
        prefix[i] = prefix[i - 1] * nums[i - 1];
    }
    for i in (0..n - 1).rev() {
        suffix[i] = suffix[i + 1] * nums[i + 1];
    }

    (0..n).map(|i| prefix[i] * suffix[i]).collect()
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
        assert_eq!(range_sum(&prefix, 1, 3), 9); // 2+3+4
        assert_eq!(range_sum(&prefix, 2, 2), 3); // just 3
    }

    #[test]
    fn test_subarray_sum_exists() {
        assert!(subarray_sum_exists(&[1, 2, 3], 5)); // [2, 3]
        assert!(subarray_sum_exists(&[1, 2, 3], 6)); // [1, 2, 3]
        assert!(!subarray_sum_exists(&[1, 2, 3], 7)); // no subarray sums to 7
        assert!(subarray_sum_exists(&[1, -1, 3], 0)); // [1, -1]
        assert!(subarray_sum_exists(&[3, 4, -7, 1, 2], 0)); // [3, 4, -7]
        assert!(!subarray_sum_exists(&[1, 2, -1, 4], 0)); // no zero-sum subarray exists
        assert!(subarray_sum_exists(&[3, 1, 4, 1, 5], 10)); // [4, 1, 5]
        assert!(subarray_sum_exists(&[-2, 3, -1, 4], 4)); // [3, -1, 4-1] etc — [-2,3,-1,4]=4
        assert!(!subarray_sum_exists(&[1, 2, 3], 100));
    }

    #[test]
    fn test_product_except_self() {
        assert_eq!(product_except_self(&[1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(product_except_self(&[2, 3, 4, 5]), vec![60, 40, 30, 24]);
        assert_eq!(product_except_self(&[1, 1, 1, 1]), vec![1, 1, 1, 1]);
    }
}
