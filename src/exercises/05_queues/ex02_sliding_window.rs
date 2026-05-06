// QUEUES - Exercise 2: Sliding Window
//
// The sliding window pattern keeps a window [left, right] over an array.
// - Expand right to grow the window
// - Shrink left when a condition is violated
//
// VecDeque is useful here to maintain a window of indices in O(1).
//
// Two types:
//   Fixed size window  — window always has exactly k elements
//   Variable window    — window grows/shrinks based on a condition

use std::collections::VecDeque;

// --- Exercise 2a ---
// Fixed window: return the maximum sum of any subarray of size k.
// O(n) using sliding window (not O(n*k) brute force).
pub fn max_sum_fixed_window(nums: &[i32], k: usize) -> i32 {
    todo!(
        "Compute the sum of the first window. Then slide: add nums[i], subtract nums[i-k].\
         Track the running max."
    )
}

// --- Exercise 2b ---
// Variable window: return the length of the longest subarray
// containing only 1s after deleting exactly one element.
// LeetCode #1004 variant (with k=1 zero allowed).
pub fn longest_ones_after_one_deletion(nums: &[i32]) -> usize {
    todo!(
        "Sliding window. Track zeros_in_window. Expand right always.\
         When zeros_in_window > 1, shrink from left.\
         Answer is max window size - 1 (for the deleted element)."
    )
}

// --- Exercise 2c ---
// Variable window: length of the longest substring without repeating characters.
// LeetCode #3.
// HINT: use a HashMap to track the last seen index of each character.
//       When a duplicate is found, move left to max(left, last_seen + 1).
pub fn longest_unique_substring(s: &str) -> usize {
    use std::collections::HashMap;
    todo!(
        "HashMap<char, usize> tracks last index of each char.\
         Expand right, update left when duplicate found.\
         Track max of (right - left + 1)."
    )
}

// --- Exercise 2d ---
// Fixed window using a monotonic deque: return the maximum of each window of size k.
// LeetCode #239. Brute force is O(n*k); this should be O(n).
// HINT: maintain a VecDeque of indices in decreasing order of nums[i].
//       The front always has the index of the current window's max.
//       Before pushing i: pop from back while nums[back] <= nums[i].
//       Before reading: pop from front if front index is outside the window.
pub fn sliding_window_maximum(nums: &[i32], k: usize) -> Vec<i32> {
    todo!(
        "Use a VecDeque<usize> of indices. Maintain decreasing values.\
         For each i: remove out-of-window indices from front,\
         remove smaller values from back, then push i.\
         After filling first window, record deque.front() value each step."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_fixed_window() {
        assert_eq!(max_sum_fixed_window(&[2, 1, 5, 1, 3, 2], 3), 9); // [5,1,3]
        assert_eq!(max_sum_fixed_window(&[1, 2, 3, 4, 5], 2), 9);    // [4,5]
        assert_eq!(max_sum_fixed_window(&[5], 1), 5);
    }

    #[test]
    fn test_longest_ones_after_one_deletion() {
        assert_eq!(longest_ones_after_one_deletion(&[1, 1, 0, 1]), 3);
        assert_eq!(longest_ones_after_one_deletion(&[0, 1, 1, 1, 0, 1, 1, 0, 1]), 5);
        assert_eq!(longest_ones_after_one_deletion(&[1, 1, 1]), 2); // must delete one
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
        assert_eq!(sliding_window_maximum(&[1, -1], 1), vec![1, -1]);
    }
}
