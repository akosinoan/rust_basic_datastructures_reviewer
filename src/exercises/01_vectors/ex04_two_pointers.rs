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
// Reverse `v` in place. Do NOT use Vec::reverse — implement the swap loop yourself.
//
// Inputs:  v — a unique reference to a Vec<i32> you must mutate.
// Returns: nothing. After the call, v reads in reverse order.
//
// Edge cases the tests check:
//   - even length              → every position swaps
//   - 2-element vec [1,2]      → [2,1]
//   - single element           → unchanged
pub fn reverse_in_place(v: &mut Vec<i32>) {
    todo!()
}

// --- Exercise 4b ---
// Given a SORTED slice `v`, return the indices (i, j) where v[i] + v[j] == target.
// Exactly one solution is guaranteed.
//
// Inputs:  v — borrowed slice already in non-decreasing order;
//          target — the desired sum.
// Returns: (left, right) with left < right and v[left] + v[right] == target.
//
// Edge cases the tests check:
//   - pair at the very ends             → e.g. [2,7,11,15], target 9 → (0,1)
//   - pair in the interior              → e.g. [1,3,4,6], target 7  → (1,2)
//   - pair at the very tail of the slice → e.g. [1,2,3,4,5], target 9 → (3,4)
pub fn two_sum_sorted(v: &[i32], target: i32) -> (usize, usize) {
    todo!()
}

// --- Exercise 4c ---
// Given a SORTED `v`, remove duplicates IN PLACE. The first k positions of `v`
// must contain the unique values in their original order. Return k.
//
// Inputs:  v — a unique reference to a sorted Vec<i32> you must mutate.
// Returns: k — the count of unique values now stored at v[0..k].
//          The contents of v[k..] are irrelevant (the test only checks v[..k]).
//
// Edge cases the tests check:
//   - input [1,1,2,3,3,4] → k = 4, v[..4] == [1,2,3,4]
pub fn remove_duplicates_in_place(v: &mut Vec<i32>) -> usize {
    todo!()
}

// --- Exercise 4d ---
// UNSORTED brute-force two-sum: return any pair of indices (i, j) with i < j and
// nums[i] + nums[j] == target. Exactly one solution is guaranteed. O(n²) is fine.
//
// Inputs:  nums — a borrowed slice; target — the desired sum.
// Returns: (i, j) with i < j and nums[i] + nums[j] == target.
//
// Edge cases the tests check:
//   - [2,7,11,15] target 9 → (0,1)
//   - [3,2,4]     target 6 → (1,2)
pub fn two_sum_unsorted_brute(nums: &[i32], target: i32) -> (usize, usize) {
    todo!()
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
        assert_eq!(two_sum_sorted(&[1, 3, 5, 6], 7), (0, 3));
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
