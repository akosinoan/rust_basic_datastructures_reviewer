// STACKS - Exercise 3: Monotonic Stack
//
// A monotonic stack maintains elements in sorted order (increasing or decreasing).
// Before pushing, pop elements that violate the order.
//
// Pattern for "next greater element":
//   for each element:
//     while stack is not empty AND stack.top < current:
//       result[stack.pop()] = current   ← current is the answer for that index
//     stack.push(current_index)
//
// When to use: "next greater/smaller element", "daily temperatures",
//              "largest rectangle in histogram"

// --- Exercise 3a ---
// Next-greater-element: for each index i, find the FIRST index j > i with
// nums[j] > nums[i]. Return -1 when no such j exists.
//
// Inputs:  nums — borrowed slice of i32.
// Returns: Vec<i32> of the same length as nums. Aim for O(n).
//
// Examples:
//   [2, 1, 2, 4, 3] → [4, 2, 4, -1, -1]
//   [1, 2, 3, 4]    → [2, 3, 4, -1]
//   [4, 3, 2, 1]    → [-1, -1, -1, -1]
pub fn next_greater_element(nums: &[i32]) -> Vec<i32> {
    todo!()
}

// --- Exercise 3b ---
// LeetCode #739. For each day, how many days until a strictly warmer one?
//
// Inputs:  temps — borrowed slice of daily temperatures.
// Returns: Vec<i32> where result[i] is the number of days you must wait after
//          day i for a warmer temperature, or 0 if no future day is warmer.
//
// Examples:
//   [73,74,75,71,69,72,76,73] → [1,1,4,2,1,1,0,0]
//   [30,40,50,60]             → [1,1,1,0]
//   [30,60,90]                → [1,1,0]
pub fn daily_temperatures(temps: &[i32]) -> Vec<i32> {
    todo!()
}

// --- Exercise 3c ---
// LeetCode #84. Largest rectangle in a histogram of bar heights.
//
// Inputs:  heights — borrowed slice of i32 ≥ 0. Each entry is one bar of width 1.
// Returns: i32 — the area of the largest axis-aligned rectangle that fits
//          inside the histogram. Aim for O(n).
//
// Examples:
//   [2, 1, 5, 6, 2, 3] → 10   (the 5×2 block from indices 2..=3)
//   [2, 4]             → 4
//   [1, 1, 1, 1]       → 4
//   [5]                → 5
pub fn largest_rectangle_in_histogram(heights: &[i32]) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_greater_element() {
        assert_eq!(next_greater_element(&[2, 1, 2, 4, 3]), vec![4, 2, 4, -1, -1]);
        assert_eq!(next_greater_element(&[1, 2, 3, 4]), vec![2, 3, 4, -1]);
        assert_eq!(next_greater_element(&[4, 3, 2, 1]), vec![-1, -1, -1, -1]);
    }

    #[test]
    fn test_daily_temperatures() {
        assert_eq!(
            daily_temperatures(&[73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(daily_temperatures(&[30, 40, 50, 60]), vec![1, 1, 1, 0]);
        assert_eq!(daily_temperatures(&[30, 60, 90]), vec![1, 1, 0]);
    }

    #[test]
    fn test_largest_rectangle() {
        assert_eq!(largest_rectangle_in_histogram(&[2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(largest_rectangle_in_histogram(&[2, 4]), 4);
        assert_eq!(largest_rectangle_in_histogram(&[1, 1, 1, 1]), 4);
        assert_eq!(largest_rectangle_in_histogram(&[5]), 5);
    }
}
