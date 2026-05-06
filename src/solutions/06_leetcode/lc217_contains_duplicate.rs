pub fn contains_duplicate_brute(nums: &[i32]) -> bool {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] == nums[j] {
                return true;
            }
        }
    }
    false
}

pub fn contains_duplicate_sort(mut nums: Vec<i32>) -> bool {
    nums.sort();
    nums.windows(2).any(|w| w[0] == w[1])
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    nums.iter().any(|n| !seen.insert(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brute() {
        assert!(contains_duplicate_brute(&[1, 2, 3, 1]));
        assert!(!contains_duplicate_brute(&[1, 2, 3, 4]));
    }

    #[test]
    fn test_sort() {
        assert!(contains_duplicate_sort(vec![1, 2, 3, 1]));
        assert!(!contains_duplicate_sort(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_hashset() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
    }
}
