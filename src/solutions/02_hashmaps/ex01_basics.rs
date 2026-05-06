use std::collections::HashMap;

pub fn build_map(pairs: &[(&str, i32)]) -> HashMap<String, i32> {
    pairs.iter().map(|&(k, v)| (k.to_string(), v)).collect()
}

pub fn get_or_zero(map: &HashMap<String, i32>, key: &str) -> i32 {
    map.get(key).copied().unwrap_or(0)
}

pub fn remove_key(map: &mut HashMap<String, i32>, key: &str) -> Option<i32> {
    map.remove(key)
}

pub fn sorted_keys(map: &HashMap<String, i32>) -> Vec<String> {
    let mut keys: Vec<String> = map.keys().cloned().collect();
    keys.sort();
    keys
}

pub fn merge_maps(mut a: HashMap<String, i32>, b: HashMap<String, i32>) -> HashMap<String, i32> {
    for (k, v) in b {
        *a.entry(k).or_insert(0) += v;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_map() {
        let m = build_map(&[("a", 1), ("b", 2), ("c", 3)]);
        assert_eq!(m.get("a"), Some(&1));
        assert_eq!(m.get("b"), Some(&2));
        assert_eq!(m.get("c"), Some(&3));
        assert_eq!(m.len(), 3);
    }

    #[test]
    fn test_get_or_zero() {
        let mut m = HashMap::new();
        m.insert("x".to_string(), 10);
        assert_eq!(get_or_zero(&m, "x"), 10);
        assert_eq!(get_or_zero(&m, "y"), 0);
    }

    #[test]
    fn test_remove_key() {
        let mut m = HashMap::new();
        m.insert("a".to_string(), 1);
        assert_eq!(remove_key(&mut m, "a"), Some(1));
        assert_eq!(remove_key(&mut m, "a"), None);
        assert!(m.is_empty());
    }

    #[test]
    fn test_sorted_keys() {
        let mut m = HashMap::new();
        m.insert("banana".to_string(), 1);
        m.insert("apple".to_string(), 2);
        m.insert("cherry".to_string(), 3);
        assert_eq!(sorted_keys(&m), vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_merge_maps() {
        let mut a = HashMap::new();
        a.insert("x".to_string(), 1);
        a.insert("y".to_string(), 2);
        let mut b = HashMap::new();
        b.insert("y".to_string(), 3);
        b.insert("z".to_string(), 4);
        let merged = merge_maps(a, b);
        assert_eq!(merged["x"], 1);
        assert_eq!(merged["y"], 5);
        assert_eq!(merged["z"], 4);
    }
}
