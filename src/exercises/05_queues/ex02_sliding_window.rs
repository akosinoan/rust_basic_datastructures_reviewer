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

// --- Exercise 2a ---
// Maximum sum of any contiguous subarray of length exactly `k`. O(n).
//
// Inputs:  nums — borrowed slice of i32; k — usize, 1 ≤ k ≤ nums.len().
// Returns: i32 — the largest sum among all windows of size k.
//
// Examples:
//   [2,1,5,1,3,2], k=3 → 9   (window [5,1,3])
//   [1,2,3,4,5],   k=2 → 9   (window [4,5])
//   [5],           k=1 → 5
pub fn max_sum_fixed_window(nums: &[i32], k: usize) -> i32 {
    todo!()
}

// --- Exercise 2b ---
// Length of the longest run of 1s after deleting EXACTLY one element.
// (LeetCode #1493 variant — at most one zero allowed in the window;
//  the deleted slot always costs 1, even if no zeros exist.)
//
// Inputs:  nums — borrowed slice of 0/1 values.
// Returns: usize — max(window_size) - 1 (we always pay for the deletion).
//
// Examples:
//   [1,1,0,1]                 → 3
//   [0,1,1,1,0,1,1,0,1]       → 5
//   [1,1,1]                   → 2   (must delete one, even though all are 1)
pub fn longest_ones_after_one_deletion(nums: &[i32]) -> usize {
    todo!()
}

// --- Exercise 2c ---
// LeetCode #3. Length of the longest substring of `s` with no repeating chars.
//
// Inputs:  s — borrowed &str (may be empty).
// Returns: usize.
//
// Examples:
//   "abcabcbb" → 3   ("abc")
//   "bbbbb"    → 1
//   "pwwkew"   → 3   ("wke")
//   ""         → 0
pub fn longest_unique_substring(s: &str) -> usize {
    todo!()
}

// --- Exercise 2d ---
// LeetCode #239. For each window of size `k` over `nums`, return the max value.
// Brute force is O(n·k); aim for O(n) using a deque of indices.
//
// Inputs:  nums — borrowed slice of i32; k — usize, 1 ≤ k ≤ nums.len().
// Returns: Vec<i32> of length nums.len() - k + 1.
//          result[i] is the max of nums[i..i+k].
//
// Example:
//   nums = [1,3,-1,-3,5,3,6,7], k = 3
//   windows:  [1,3,-1] [3,-1,-3] [-1,-3,5] [-3,5,3] [5,3,6] [3,6,7]
//   maxes:       3        3         5         5        6        7
//   answer:   [3, 3, 5, 5, 6, 7]
pub fn sliding_window_maximum(nums: &[i32], k: usize) -> Vec<i32> {
    todo!()
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
