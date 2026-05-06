// ============================================================
// LeetCode #26 — Remove Duplicates from Sorted Array
// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
// Difficulty: Easy
// ============================================================
//
// Given a sorted array, remove duplicates IN-PLACE so each element
// appears only once. Return the count k of unique elements.
// The first k elements of the modified array must be the unique values.
//
// Example:
//   [1,1,2] → k=2, array becomes [1,2,_]
//   [0,0,1,1,1,2,2,3,3,4] → k=5, array becomes [0,1,2,3,4,_,_,_,_,_]
//
// ---------------------------------------------------------------
// APPROACH: Two Pointers — O(n) time, O(1) space
//
// slow pointer k = write position (next slot for unique value)
// fast pointer i scans through the array
// When nums[i] != nums[k-1]: copy nums[i] to nums[k], increment k
// ---------------------------------------------------------------

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    todo!(
        "Handle empty case. Start k=1. For i in 1..nums.len():\
         if nums[i] != nums[k-1], set nums[k] = nums[i] and k += 1.\
         Return k as i32."
    )
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
