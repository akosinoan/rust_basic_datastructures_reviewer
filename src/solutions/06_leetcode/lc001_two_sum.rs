use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        let complement = target - n;
        if let Some(&j) = map.get(&complement) {
            return vec![j, i as i32];
        }
        map.insert(n, i as i32);
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() { assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]); }

    #[test]
    fn example_2() { assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]); }

    #[test]
    fn example_3() { assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]); }

    #[test]
    fn negative_numbers() { assert_eq!(two_sum(vec![-1, -2, -3, -4, -5], -8), vec![2, 4]); }
}
