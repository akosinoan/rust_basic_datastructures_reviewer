// ============================================================
// LeetCode #26 — Remove Duplicates from Sorted Array
// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
// Difficulty: Easy
// ============================================================
//
// Given a SORTED array, remove duplicates IN-PLACE so each value appears
// exactly once. Return the count k of unique values; the first k positions
// of `nums` must hold those unique values in their original (sorted) order.
// Whatever sits past index k is irrelevant — the LeetCode grader (and the
// tests below) only inspect nums[..k].
//
// Examples:
//   [1,1,2]                  → k=2, nums[..2] == [1,2]
//   [0,0,1,1,1,2,2,3,3,4]    → k=5, nums[..5] == [0,1,2,3,4]

// --- remove_duplicates ---
// Inputs:  nums — exclusive reference to a sorted Vec<i32>.
// Returns: i32 — the count k of unique values.
//          You must compute k IN PLACE; aim for O(n) time, O(1) extra space.
//
// Edge cases the tests check:
//   - [1,1,2]                  → k=2, nums[..2] == [1,2]
//   - [0,0,1,1,1,2,2,3,3,4]    → k=5, nums[..5] == [0,1,2,3,4]
//   - [7,7,7,7]                → k=1, nums[..1] == [7]
//   - [1,2,3,4,5]              → k=5 (already unique)
//   - []                       → k=0 (empty)
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![1, 1, 2];
        let k = remove_duplicates(&mut nums) as usize;
        assert_eq!(k, 2);
        assert_eq!(&nums[..k], &[1, 2]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let k = remove_duplicates(&mut nums) as usize;
        assert_eq!(k, 5);
        assert_eq!(&nums[..k], &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn all_same() {
        let mut nums = vec![7, 7, 7, 7];
        let k = remove_duplicates(&mut nums) as usize;
        assert_eq!(k, 1);
        assert_eq!(&nums[..k], &[7]);
    }

    #[test]
    fn already_unique() {
        let mut nums = vec![1, 2, 3, 4, 5];
        let k = remove_duplicates(&mut nums) as usize;
        assert_eq!(k, 5);
        assert_eq!(&nums[..k], &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn empty() {
        let mut nums: Vec<i32> = vec![];
        assert_eq!(remove_duplicates(&mut nums), 0);
    }
}
