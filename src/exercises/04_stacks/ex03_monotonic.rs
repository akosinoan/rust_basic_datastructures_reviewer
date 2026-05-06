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
// For each element, find the next GREATER element to its right.
// If none exists, use -1.
// Example: [2, 1, 2, 4, 3] → [4, 2, 4, -1, -1]
pub fn next_greater_element(nums: &[i32]) -> Vec<i32> {
    todo!(
        "Init result vec with -1s. Use a stack of indices.\
         For each i, while stack top's value < nums[i], pop and set result[popped] = nums[i].\
         Then push i."
    )
}

// --- Exercise 3b ---
// LeetCode #739: Daily Temperatures.
// Return an array where result[i] = days until a warmer temperature.
// If no warmer day, result[i] = 0.
// Example: [73,74,75,71,69,72,76,73] → [1,1,4,2,1,1,0,0]
pub fn daily_temperatures(temps: &[i32]) -> Vec<i32> {
    todo!(
        "Same pattern as next_greater_element but track indices.\
         result[popped_index] = current_index - popped_index"
    )
}

// --- Exercise 3c ---
// LeetCode #84: Largest Rectangle in Histogram.
// Given bar heights, find the area of the largest rectangle.
// Example: [2,1,5,6,2,3] → 10
// HINT: monotonic increasing stack. When a bar is shorter than the top,
//       pop and compute area using the popped height and current width.
pub fn largest_rectangle_in_histogram(heights: &[i32]) -> i32 {
    todo!(
        "Use a stack of indices. Push if heights[i] >= stack top's height.\
         When heights[i] < top: pop, compute area = height * (i - stack.last() - 1).\
         After the loop, drain remaining stack elements."
    )
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
