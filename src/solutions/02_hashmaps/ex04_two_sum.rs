use std::collections::HashMap;

pub fn two_sum(nums: &[i32], target: i32) -> (usize, usize) {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        let complement = target - n;
        if let Some(&j) = map.get(&complement) {
            return (j, i);
        }
        map.insert(n, i);
    }
    unreachable!("guaranteed one solution")
}

pub fn has_pair_with_sum(nums: &[i32], target: i32) -> bool {
    let mut seen = HashMap::new();
    for &n in nums {
        if seen.contains_key(&(target - n)) {
            return true;
        }
        seen.insert(n, true);
    }
    false
}

pub fn all_pairs_with_sum(nums: &[i32], target: i32) -> Vec<(i32, i32)> {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut result = HashSet::new();
    for &n in nums {
        let complement = target - n;
        if seen.contains(&complement) {
            let pair = (n.min(complement), n.max(complement));
            result.insert(pair);
        }
        seen.insert(n);
    }
    let mut v: Vec<_> = result.into_iter().collect();
    v.sort();
    v
}

pub fn three_sum(nums: &mut Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut result = Vec::new();
    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum == 0 {
                result.push(vec![nums[i], nums[left], nums[right]]);
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }
                left += 1;
                right -= 1;
            } else if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), (0, 1));
        assert_eq!(two_sum(&[3, 2, 4], 6), (1, 2));
        assert_eq!(two_sum(&[3, 3], 6), (0, 1));
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
