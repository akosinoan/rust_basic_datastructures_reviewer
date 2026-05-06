pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut k = 1;
    for i in 1..nums.len() {
        if nums[i] != nums[k - 1] {
            nums[k] = nums[i];
            k += 1;
        }
    }
    k as i32
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
    fn empty() {
        let mut nums: Vec<i32> = vec![];
        assert_eq!(remove_duplicates(&mut nums), 0);
    }
}
