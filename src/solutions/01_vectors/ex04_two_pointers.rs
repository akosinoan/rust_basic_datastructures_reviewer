pub fn reverse_in_place(v: &mut Vec<i32>) {
    let mut left = 0;
    let mut right = v.len().saturating_sub(1);
    while left < right {
        v.swap(left, right);
        left += 1;
        right -= 1;
    }
}

pub fn two_sum_sorted(v: &[i32], target: i32) -> (usize, usize) {
    let mut left = 0;
    let mut right = v.len() - 1;
    while left < right {
        let sum = v[left] + v[right];
        if sum == target {
            return (left, right);
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    unreachable!("guaranteed one solution")
}

pub fn remove_duplicates_in_place(v: &mut Vec<i32>) -> usize {
    if v.is_empty() {
        return 0;
    }
    let mut k = 1;
    for i in 1..v.len() {
        if v[i] != v[k - 1] {
            v[k] = v[i];
            k += 1;
        }
    }
    k
}

pub fn two_sum_unsorted_brute(nums: &[i32], target: i32) -> (usize, usize) {
    for i in 0..nums.len() {
        for j in (i + 1)..nums.len() {
            if nums[i] + nums[j] == target {
                return (i, j);
            }
        }
    }
    unreachable!("guaranteed one solution")
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
        assert_eq!(two_sum_sorted(&[1, 2, 5, 9], 7), (1, 2));
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
        assert_eq!(two_sum_unsorted_brute(&[2, 7, 11, 15], 9), (0, 1));
        assert_eq!(two_sum_unsorted_brute(&[3, 2, 4], 6), (1, 2));
    }
}
