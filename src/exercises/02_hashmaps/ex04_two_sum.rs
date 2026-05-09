// HASHMAPS - Exercise 4: Two Sum (HashMap approach)
//
// This is LeetCode #1. The naive brute force is O(n²).
// The HashMap approach is O(n):
//
//   For each element nums[i]:
//     complement = target - nums[i]
//     if complement is in map → found the pair!
//     else → store map[nums[i]] = i
//
// Key insight: we're trading space for time. Instead of searching
// the array for the complement, we look it up in O(1).

// --- Exercise 4a ---
// Find the indices of the two distinct elements of `nums` that sum to `target`.
//
// Inputs:  nums — borrowed slice; target — desired sum.
//          Exactly one solution is guaranteed; you may NOT use the same element twice.
// Returns: (i, j) with i < j and nums[i] + nums[j] == target.
//          Aim for O(n) time.
//
// Edge cases the tests check:
//   - [2,7,11,15] target 9 → (0,1)
//   - [3,2,4]     target 6 → (1,2)
//   - [3,3]       target 6 → (0,1)  (two distinct positions, same value — this is allowed)
pub fn two_sum(nums: &[i32], target: i32) -> (usize, usize) {
    todo!()
}

// --- Exercise 4b ---
// Return true iff some pair of DISTINCT positions in `nums` sums to `target`.
//
// Inputs:  nums — borrowed slice; target — desired sum.
// Returns: bool. Aim for O(n) time.
//
// Edge cases the tests check:
//   - [1,2,3,4] target 5 → true (1+4 or 2+3)
//   - [1,2,3,4] target 8 → false
//   - [0,0]     target 0 → true (two distinct positions)
pub fn has_pair_with_sum(nums: &[i32], target: i32) -> bool {
    todo!()
}

// --- Exercise 4c ---
// Return every UNIQUE pair (a, b) with a < b and a + b == target.
//
// Inputs:  nums — borrowed slice; target — desired sum.
// Returns: Vec<(i32, i32)>. Each pair appears only once; within each pair the
//          smaller value comes first. Output order does not matter — tests sort it.
//
// Edge cases the tests check:
//   - [1,2,3,4,5] target 6 → sorts to [(1,5),(2,4)]
//   - [1,2,3]     target 10 → []
pub fn all_pairs_with_sum(nums: &[i32], target: i32) -> Vec<(i32, i32)> {
    todo!()
}

// --- Exercise 4d ---
// Three-Sum: every unique TRIPLET (a, b, c) of values from `nums` (using each
// position at most once) such that a + b + c == 0.
//
// Inputs:  nums — exclusive reference; you are free to sort it.
// Returns: Vec<Vec<i32>>. Each inner Vec is exactly 3 sorted ascending values.
//          No duplicate triplets: if [-1,-1,2] appears, it appears once. The
//          tests sort the outer Vec before comparing.
//
// Edge cases the tests check:
//   - [-1,0,1,2,-1,-4] (sorted to [-4,-1,-1,0,1,2])
//     → triplets sorted: [[-1,-1,2], [-1,0,1]]
//   - [0,0,0]   → [[0,0,0]]
//   - [1,2,3]   → []
pub fn three_sum(nums: &mut Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let r = two_sum(&[2, 7, 11, 15], 9);
        assert_eq!(r, (0, 1));

        let r2 = two_sum(&[3, 2, 4], 6);
        assert_eq!(r2, (1, 2));

        let r3 = two_sum(&[3, 3], 6);
        assert_eq!(r3, (0, 1));
    }

    #[test]
    fn test_has_pair_with_sum() {
        assert!(has_pair_with_sum(&[1, 2, 3, 4], 5));
        assert!(!has_pair_with_sum(&[1, 2, 3, 4], 8));
        assert!(has_pair_with_sum(&[0, 0], 0));
    }

    #[test]
    fn test_all_pairs_with_sum() {
        let mut pairs = all_pairs_with_sum(&[1, 2, 3, 4, 5], 6);
        pairs.sort();
        assert_eq!(pairs, vec![(1, 5), (2, 4)]);

        let pairs2 = all_pairs_with_sum(&[1, 2, 3], 10);
        assert_eq!(pairs2, vec![]);
    }

    #[test]
    fn test_three_sum() {
        let mut nums = vec![-1, 0, 1, 2, -1, -4];
        let mut result = three_sum(&mut nums);
        result.sort();
        assert_eq!(result, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);

        let mut nums2 = vec![0, 0, 0];
        assert_eq!(three_sum(&mut nums2), vec![vec![0, 0, 0]]);

        let mut nums3 = vec![1, 2, 3];
        assert_eq!(three_sum(&mut nums3), vec![] as Vec<Vec<i32>>);
    }
}
