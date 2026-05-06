// HASHMAPS - Exercise 1: Basics
//
// HashMap<K, V> stores key-value pairs with O(1) average get/insert.
//
//   use std::collections::HashMap;
//   let mut map = HashMap::new();
//   map.insert("key", 42);          — insert or overwrite
//   map.get("key")                  — Option<&V>
//   map.get("key").copied()         — Option<V> when V: Copy
//   map.contains_key("key")         — bool
//   map.remove("key")               — Option<V>
//   map.len()                       — bool
//   for (k, v) in &map { ... }      — iterate (unordered!)

use std::collections::HashMap;

// --- Exercise 1a ---
// Build a HashMap from the given pairs and return it.
pub fn build_map(pairs: &[(&str, i32)]) -> HashMap<String, i32> {
    todo!("Insert each (key, value) pair into a HashMap and return it")
}

// --- Exercise 1b ---
// Return the value for a key, or 0 if the key doesn't exist.
pub fn get_or_zero(map: &HashMap<String, i32>, key: &str) -> i32 {
    todo!("Use .get() and unwrap with a default, or .copied().unwrap_or(0)")
}

// --- Exercise 1c ---
// Remove a key from the map and return its value, or None if absent.
pub fn remove_key(map: &mut HashMap<String, i32>, key: &str) -> Option<i32> {
    todo!("Use .remove()")
}

// --- Exercise 1d ---
// Return a sorted Vec of all keys in the map.
// HINT: .keys() returns an iterator; collect into Vec<&String>, then sort
pub fn sorted_keys(map: &HashMap<String, i32>) -> Vec<String> {
    todo!("Collect keys, sort them, return owned Strings")
}

// --- Exercise 1e ---
// Merge two maps. If a key appears in both, sum the values.
pub fn merge_maps(a: HashMap<String, i32>, b: HashMap<String, i32>) -> HashMap<String, i32> {
    todo!("Start with one map, then for each entry in the other, add or insert")
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
        assert_eq!(merged["y"], 5); // 2 + 3
        assert_eq!(merged["z"], 4);
    }
}
