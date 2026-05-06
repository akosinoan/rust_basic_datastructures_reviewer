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
    // Create an empty StreamProcessor.
    pub fn new() -> Self {
        todo!("Return StreamProcessor with an empty VecDeque")
    }

    // Add a value to the back of the window.
    pub fn enqueue(&mut self, val: i32) {
        todo!("push_back onto self.window")
    }

    // Remove and return the value at the front of the window, or None if empty.
    pub fn dequeue(&mut self) -> Option<i32> {
        todo!("pop_front from self.window")
    }

    // Return the maximum value currently in the window, or None if empty.
    // Simple scan is fine here — O(n) is acceptable.
    pub fn window_max(&self) -> Option<i32> {
        todo!("self.window.iter().max() — copy the i32 out")
    }
}

// --- Quiz 5b ---
// Sliding window maximum (fixed size k).
//
// Return a Vec where each entry is the max of the k-element window
// ending at that position (starting once the first full window is filled).
// Result length: nums.len() - k + 1
//
// HINT: maintain a monotonic deque of *indices* (front = index of current max).
//   For each i:
//     1. Pop front while front index is out of the window (< i - k + 1)
//     2. Pop back while nums[back] <= nums[i]  (smaller values can never be max)
//     3. Push i to back
//     4. Once i >= k-1, record nums[deque.front()] as the window max
pub fn sliding_window_maxes(nums: &[i32], k: usize) -> Vec<i32> {
    todo!("monotonic deque of indices for O(n) window max")
}

// --- Quiz 5c ---
// Longest subarray whose sum is ≤ k (all elements are non-negative).
//
// Return the length of the longest contiguous subarray with sum ≤ k.
//
// HINT: two-pointer / shrinking window.
//   Expand right, adding nums[right] to the running sum.
//   While sum > k, subtract nums[left] and advance left.
//   Track max(right - left + 1) at each step.
pub fn longest_subarray_sum_le(nums: &[i32], k: i32) -> usize {
    todo!("two-pointer: expand right, shrink left when sum > k")
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
        assert_eq!(longest_subarray_sum_le(&[1, 2, 3, 4, 5], 11), 4); // [1,2,3,4] or [2,3,4] or [1,2,3,5]... → [1,2,3,4]=10 ≤ 11
        assert_eq!(longest_subarray_sum_le(&[1, 2, 3, 4, 5], 15), 5);
        assert_eq!(longest_subarray_sum_le(&[5, 5, 5], 4), 0);
    }

    #[test]
    fn test_longest_subarray_sum_le_single() {
        assert_eq!(longest_subarray_sum_le(&[3], 3), 1);
        assert_eq!(longest_subarray_sum_le(&[4], 3), 0);
    }
}
