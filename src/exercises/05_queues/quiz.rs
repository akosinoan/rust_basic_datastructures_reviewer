// QUIZ — Section 5: Stream Processor
//
// Build a StreamProcessor backed by a VecDeque, plus two free functions that
// solve sliding-window problems. You will use VecDeque push/pop, monotonic
// deque for O(n) window max, and a two-pointer shrink for variable windows —
// all patterns from this section.

use std::collections::VecDeque;

pub struct StreamProcessor {
    window: VecDeque<i32>,
}

impl StreamProcessor {
    // Construct an empty StreamProcessor.
    pub fn new() -> Self {
        todo!()
    }

    // Append `val` to the back of the window.
    pub fn enqueue(&mut self, val: i32) {
        todo!()
    }

    // Remove and return the value at the FRONT of the window.
    //
    // Returns: Some(value) if the window is non-empty, else None.
    pub fn dequeue(&mut self) -> Option<i32> {
        todo!()
    }

    // Return the maximum value currently in the window. A simple linear scan
    // is acceptable here (O(n)).
    //
    // Returns: Some(max) if non-empty, else None.
    pub fn window_max(&self) -> Option<i32> {
        todo!()
    }
}

// --- Quiz 5b ---
// Sliding window maximum (fixed window of size k).
//
// Inputs:  nums — borrowed slice; k — usize, 1 ≤ k ≤ nums.len().
// Returns: Vec<i32> of length nums.len() - k + 1, with result[i] equal to
//          the max of nums[i..i+k]. Aim for O(n).
//
// Examples:
//   nums = [1,3,-1,-3,5,3,6,7], k = 3 → [3,3,5,5,6,7]
//   nums = [4,2,7],             k = 1 → [4,2,7]
//   nums = [1,2,3],             k = 3 → [3]
pub fn sliding_window_maxes(nums: &[i32], k: usize) -> Vec<i32> {
    todo!()
}

// --- Quiz 5c ---
// Length of the longest contiguous subarray of `nums` whose sum is ≤ k.
// You may assume every element of `nums` is non-negative.
//
// Inputs:  nums — borrowed slice of non-negative i32; k — i32 budget.
// Returns: usize length of the longest qualifying subarray. 0 if none qualify.
//
// Examples:
//   nums = [1,2,3,4,5], k = 11  → 4   ([1,2,3,4]=10 ≤ 11)
//   nums = [1,2,3,4,5], k = 15  → 5   (whole array sums to 15)
//   nums = [5,5,5],     k = 4   → 0   (every single element exceeds 4)
//   nums = [3],         k = 3   → 1
//   nums = [4],         k = 3   → 0
pub fn longest_subarray_sum_le(nums: &[i32], k: i32) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut sp = StreamProcessor::new();
        sp.enqueue(1);
        sp.enqueue(2);
        sp.enqueue(3);
        assert_eq!(sp.dequeue(), Some(1));
        assert_eq!(sp.dequeue(), Some(2));
        assert_eq!(sp.dequeue(), Some(3));
        assert_eq!(sp.dequeue(), None);
    }

    #[test]
    fn test_window_max_empty() {
        let sp = StreamProcessor::new();
        assert_eq!(sp.window_max(), None);
    }

    #[test]
    fn test_window_max() {
        let mut sp = StreamProcessor::new();
        sp.enqueue(3);
        sp.enqueue(1);
        sp.enqueue(5);
        sp.enqueue(2);
        assert_eq!(sp.window_max(), Some(5));
    }

    #[test]
    fn test_window_max_after_dequeue() {
        let mut sp = StreamProcessor::new();
        sp.enqueue(9);
        sp.enqueue(3);
        sp.dequeue(); // remove 9
        assert_eq!(sp.window_max(), Some(3));
    }

    #[test]
    fn test_sliding_window_maxes() {
        assert_eq!(
            sliding_window_maxes(&[1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
    }

    #[test]
    fn test_sliding_window_maxes_k1() {
        assert_eq!(
            sliding_window_maxes(&[4, 2, 7], 1),
            vec![4, 2, 7]
        );
    }

    #[test]
    fn test_sliding_window_maxes_full_window() {
        assert_eq!(
            sliding_window_maxes(&[1, 2, 3], 3),
            vec![3]
        );
    }

    #[test]
    fn test_longest_subarray_sum_le() {
        assert_eq!(longest_subarray_sum_le(&[1, 2, 3, 4, 5], 11), 4);
        assert_eq!(longest_subarray_sum_le(&[1, 2, 3, 4, 5], 15), 5);
        assert_eq!(longest_subarray_sum_le(&[5, 5, 5], 4), 0);
    }

    #[test]
    fn test_longest_subarray_sum_le_single() {
        assert_eq!(longest_subarray_sum_le(&[3], 3), 1);
        assert_eq!(longest_subarray_sum_le(&[4], 3), 0);
    }
}
