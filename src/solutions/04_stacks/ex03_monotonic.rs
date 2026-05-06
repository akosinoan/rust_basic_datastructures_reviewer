pub fn next_greater_element(nums: &[i32]) -> Vec<i32> {
    let mut result = vec![-1; nums.len()];
    let mut stack: Vec<usize> = Vec::new();
    for i in 0..nums.len() {
        while let Some(&top) = stack.last() {
            if nums[top] < nums[i] {
                result[top] = nums[i];
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }
    result
}

pub fn daily_temperatures(temps: &[i32]) -> Vec<i32> {
    let mut result = vec![0; temps.len()];
    let mut stack: Vec<usize> = Vec::new();
    for i in 0..temps.len() {
        while let Some(&top) = stack.last() {
            if temps[top] < temps[i] {
                result[top] = (i - top) as i32;
                stack.pop();
            } else {
                break;
            }
        }
        stack.push(i);
    }
    result
}

pub fn largest_rectangle_in_histogram(heights: &[i32]) -> i32 {
    let mut stack: Vec<usize> = Vec::new();
    let mut max_area = 0;
    let n = heights.len();

    for i in 0..=n {
        let h = if i == n { 0 } else { heights[i] };
        while let Some(&top) = stack.last() {
            if heights[top] <= h {
                break;
            }
            stack.pop();
            let width = match stack.last() {
                Some(&prev) => i - prev - 1,
                None => i,
            };
            max_area = max_area.max(heights[top] * width as i32);
        }
        stack.push(i);
    }
    max_area
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
    }

    #[test]
    fn test_largest_rectangle() {
        assert_eq!(largest_rectangle_in_histogram(&[2, 1, 5, 6, 2, 3]), 10);
        assert_eq!(largest_rectangle_in_histogram(&[2, 4]), 4);
        assert_eq!(largest_rectangle_in_histogram(&[1, 1, 1, 1]), 4);
    }
}
