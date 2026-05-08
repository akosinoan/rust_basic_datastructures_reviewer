pub fn build_vec() -> Vec<i32> {
    (1..=5).collect()
}

pub fn last_element(mut v: Vec<i32>) -> i32 {
    v.pop().unwrap_or(-1)
}

pub fn safe_get(v: &[i32], i: usize) -> Option<i32> {
    v.get(i).copied()
}

pub fn drain_and_count(v: &mut Vec<i32>) -> usize {
    let count = v.len();
    v.clear();
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_vec() {
        assert_eq!(build_vec(), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_last_element() {
        let (v1, v2, v3) = (vec![10, 20, 30], vec![], vec![42]);
        assert_eq!(last_element(&v1), 30);
        assert_eq!(last_element(&v2), -1);
        assert_eq!(last_element(&v3), 42);
    }

    #[test]
    fn test_safe_get() {
        let v: Vec<i32> = vec![10, 20, 30];
        assert_eq!(safe_get(&v, 0), Some(10));
        assert_eq!(safe_get(&v, 2), Some(30));
        assert_eq!(safe_get(&v, 5), None);
    }

    #[test]
    fn test_drain_and_count() {
        let mut v = vec![1, 2, 3, 4];
        let count = drain_and_count(&mut v);
        assert_eq!(count, 4);
        assert!(v.is_empty());
    }
}
