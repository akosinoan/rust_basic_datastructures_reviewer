pub fn sum(v: &[i32]) -> i32 {
    v.iter().sum()
}

pub fn double_all(v: &[i32]) -> Vec<i32> {
    v.iter().map(|&x| x * 2).collect()
}

pub fn keep_evens(v: &[i32]) -> Vec<i32> {
    v.iter().copied().filter(|&x| x % 2 == 0).collect()
}

pub fn first_negative_index(v: &[i32]) -> Option<usize> {
    v.iter().position(|&x| x < 0)
}

pub fn concat_all(v: &[String]) -> String {
    v.iter().map(|s| s.as_str()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum(&[]), 0);
        assert_eq!(sum(&[-1, 1]), 0);
    }

    #[test]
    fn test_double_all() {
        assert_eq!(double_all(&[1, 2, 3]), vec![2, 4, 6]);
        assert_eq!(double_all(&[]), vec![]);
    }

    #[test]
    fn test_keep_evens() {
        assert_eq!(keep_evens(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
        assert_eq!(keep_evens(&[1, 3, 5]), vec![]);
    }

    #[test]
    fn test_first_negative_index() {
        assert_eq!(first_negative_index(&[1, 2, -3, 4]), Some(2));
        assert_eq!(first_negative_index(&[1, 2, 3]), None);
        assert_eq!(first_negative_index(&[-1]), Some(0));
    }

    #[test]
    fn test_concat_all() {
        let v = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];
        assert_eq!(concat_all(&v), "foobarbaz");
        assert_eq!(concat_all(&[]), "");
    }
}
