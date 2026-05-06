pub fn max_sum_fixed_window(nums: &[i32], k: usize) -> i32 {
    let mut window_sum: i32 = nums[..k].iter().sum();
    let mut max = window_sum;
    for i in k..nums.len() {
        window_sum += nums[i] - nums[i - k];
        max = max.max(window_sum);
    }
    max
}

pub fn longest_ones_after_one_deletion(nums: &[i32]) -> usize {
    let mut left = 0;
    let mut zeros = 0;
    let mut max_len = 0;
    for right in 0..nums.len() {
        if nums[right] == 0 {
            zeros += 1;
        }
        while zeros > 1 {
            if nums[left] == 0 {
                zeros -= 1;
            }
            left += 1;
        }
        // window size - 1 because we must delete one element
        max_len = max_len.max(right - left);
    }
    max_len
}

pub fn longest_unique_substring(s: &str) -> usize {
    use std::collections::HashMap;
    let chars: Vec<char> = s.chars().collect();
    let mut last_seen: HashMap<char, usize> = HashMap::new();
    let mut left = 0;
    let mut max_len = 0;
    for right in 0..chars.len() {
        if let Some(&prev) = last_seen.get(&chars[right]) {
            left = left.max(prev + 1);
        }
        last_seen.insert(chars[right], right);
        max_len = max_len.max(right - left + 1);
    }
    max_len
}

pub fn sliding_window_maximum(nums: &[i32], k: usize) -> Vec<i32> {
    use std::collections::VecDeque;
    let mut deque: VecDeque<usize> = VecDeque::new();
    let mut result = Vec::new();
    for i in 0..nums.len() {
        // remove indices outside the window
        while deque.front().map_or(false, |&f| f + k <= i) {
            deque.pop_front();
        }
        // remove indices whose values are smaller than current
        while deque.back().map_or(false, |&b| nums[b] <= nums[i]) {
            deque.pop_back();
        }
        deque.push_back(i);
        if i + 1 >= k {
            result.push(nums[*deque.front().unwrap()]);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_fixed_window() {
        assert_eq!(max_sum_fixed_window(&[2, 1, 5, 1, 3, 2], 3), 9);
        assert_eq!(max_sum_fixed_window(&[1, 2, 3, 4, 5], 2), 9);
    }

    #[test]
    fn test_longest_ones_after_one_deletion() {
        assert_eq!(longest_ones_after_one_deletion(&[1, 1, 0, 1]), 3);
        assert_eq!(longest_ones_after_one_deletion(&[0, 1, 1, 1, 0, 1, 1, 0, 1]), 5);
        assert_eq!(longest_ones_after_one_deletion(&[1, 1, 1]), 2);
    }

    #[test]
    fn test_longest_unique_substring() {
        assert_eq!(longest_unique_substring("abcabcbb"), 3);
        assert_eq!(longest_unique_substring("bbbbb"), 1);
        assert_eq!(longest_unique_substring("pwwkew"), 3);
        assert_eq!(longest_unique_substring(""), 0);
    }

    #[test]
    fn test_sliding_window_maximum() {
        assert_eq!(
            sliding_window_maximum(&[1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(sliding_window_maximum(&[1], 1), vec![1]);
    }
}
