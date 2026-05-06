pub fn sort_asc(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();
    v
}

pub fn sort_desc(mut v: Vec<i32>) -> Vec<i32> {
    v.sort_by(|a, b| b.cmp(a));
    v
}

pub fn sort_by_length(mut v: Vec<String>) -> Vec<String> {
    v.sort_by_key(|s| s.len());
    v
}

pub fn binary_search(v: &[i32], target: i32) -> bool {
    v.binary_search(&target).is_ok()
}

pub fn remove_duplicates(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();
    v.dedup();
    v
}

pub fn max_window_sum(v: &[i32], k: usize) -> i32 {
    v.windows(k)
        .map(|w| w.iter().sum())
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_asc() {
        assert_eq!(sort_asc(vec![3, 1, 4, 1, 5, 9]), vec![1, 1, 3, 4, 5, 9]);
    }

    #[test]
    fn test_sort_desc() {
        assert_eq!(sort_desc(vec![3, 1, 4, 1, 5]), vec![5, 4, 3, 1, 1]);
    }

    #[test]
    fn test_sort_by_length() {
        let input = vec!["banana".to_string(), "kiwi".to_string(), "fig".to_string()];
        let result = sort_by_length(input);
        assert_eq!(result[0], "fig");
        assert_eq!(result[2], "banana");
    }

    #[test]
    fn test_binary_search() {
        let v = vec![1, 3, 5, 7, 9];
        assert!(binary_search(&v, 5));
        assert!(!binary_search(&v, 4));
    }

    #[test]
    fn test_remove_duplicates() {
        let mut result = remove_duplicates(vec![3, 1, 4, 1, 5, 3]);
        result.sort();
        assert_eq!(result, vec![1, 3, 4, 5]);
    }

    #[test]
    fn test_max_window_sum() {
        assert_eq!(max_window_sum(&[1, 3, -1, -3, 5, 3, 6, 7], 3), 16);
        assert_eq!(max_window_sum(&[2, 1, 5, 1, 3, 2], 3), 9);
        assert_eq!(max_window_sum(&[1, 2, 3], 3), 6);
    }
}
