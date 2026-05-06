// VECTORS - Exercise 4: Two Pointers
//
// The two-pointer pattern uses two indices that move toward each other
// (or in the same direction) to avoid a nested loop.
//
// Classic setup:
//   let mut left = 0;
//   let mut right = v.len() - 1;   // careful: usize wraps!
//   while left < right {
//       // ... decide to move left or right
//   }
//
// When to use:
//   - Sorted array + pair/triplet questions
//   - In-place reversal / palindrome check
//   - Removing elements satisfying a condition

// --- Exercise 4a ---
// Reverse a slice in place using two pointers.
// Do NOT use .reverse() — implement the swap loop yourself.
pub fn reverse_in_place(v: &mut Vec<i32>) {
    todo!(
        "Use left and right pointers, swap v[left] and v[right], move them inward"
    )
}

// --- Exercise 4b ---
// Given a SORTED vec and a target sum, return the indices of the two numbers
// that add up to target. Guaranteed exactly one solution.
// HINT: if v[left] + v[right] < target → move left right
//       if v[left] + v[right] > target → move right left
pub fn two_sum_sorted(v: &[i32], target: i32) -> (usize, usize) {
    todo!("Use two pointers on a sorted array to find the pair")
}

// --- Exercise 4c ---
// Given a sorted vec, remove all duplicates in-place and return the new length.
// The first k elements of the vec must be unique; the rest don't matter.
// HINT: slow pointer tracks the write position; fast pointer scans ahead.
pub fn remove_duplicates_in_place(v: &mut Vec<i32>) -> usize {
    todo!(
        "Use a slow pointer `k` that advances only when v[fast] != v[k-1]"
    )
}

// --- Exercise 4d ---
// Given an array of integers, return the indices of a pair that sums to target.
// The array is NOT sorted here. You may assume exactly one solution.
// HINT: this does NOT use two pointers — it uses a HashMap (preview of next module).
//       But try it with two pointers on a copy first to feel the O(n²) brute force.
pub fn two_sum_unsorted_brute(nums: &[i32], target: i32) -> (usize, usize) {
    todo!("Use two nested loops: for i in 0..n, for j in i+1..n, check if nums[i]+nums[j]==target")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_in_place() {
        let mut v = vec![1, 2, 3, 4, 5];
        reverse_in_place(&mut v);
        assert_eq!(v, vec![5, 4, 3, 2, 1]);

        let mut v2 = vec![1, 2];
        reverse_in_place(&mut v2);
        assert_eq!(v2, vec![2, 1]);

        let mut v3 = vec![1];
        reverse_in_place(&mut v3);
        assert_eq!(v3, vec![1]);
    }

    #[test]
    fn test_two_sum_sorted() {
        assert_eq!(two_sum_sorted(&[2, 7, 11, 15], 9), (0, 1));
        assert_eq!(two_sum_sorted(&[1, 3, 4, 6], 7), (1, 2));
        assert_eq!(two_sum_sorted(&[1, 2, 3, 4, 5], 9), (3, 4));
    }

    #[test]
    fn test_remove_duplicates_in_place() {
        let mut v = vec![1, 1, 2, 3, 3, 4];
        let k = remove_duplicates_in_place(&mut v);
        assert_eq!(k, 4);
        assert_eq!(&v[..k], &[1, 2, 3, 4]);
    }

    #[test]
    fn test_two_sum_unsorted_brute() {
        let r = two_sum_unsorted_brute(&[2, 7, 11, 15], 9);
        assert_eq!(r, (0, 1));
        let r2 = two_sum_unsorted_brute(&[3, 2, 4], 6);
        assert_eq!(r2, (1, 2));
    }
}
