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
// Build a HashMap<String, i32> from an array of (&str, i32) pairs.
//
// Inputs:  pairs — a borrowed slice of tuples; the &str values are short-lived
//          so you must convert them into owned Strings for the map keys.
// Returns: a HashMap whose entries match the input pairs one-for-one.
//
// Edge cases the tests check:
//   - several distinct keys → all present, .len() equals number of pairs
pub fn build_map(pairs: &[(&str, i32)]) -> HashMap<String, i32> {
    todo!()
}

// --- Exercise 1b ---
// Look up `key` in `map`. Return the stored value, or 0 if absent.
//
// Inputs:  map — borrowed HashMap; key — &str to look up.
// Returns: the i32 value associated with key, or 0 if no such entry exists.
//
// Edge cases the tests check:
//   - key present  → its stored value
//   - key absent   → 0
pub fn get_or_zero(map: &HashMap<String, i32>, key: &str) -> i32 {
    todo!()
}

// --- Exercise 1c ---
// Remove `key` from `map` and return its old value.
//
// Inputs:  map — exclusive reference; key — &str to remove.
// Returns: Some(value) if the key existed, else None.
//          A second call with the same key (already removed) must return None.
//
// Edge cases the tests check:
//   - first call on present key  → Some(value), key gone afterwards
//   - second call on same key    → None
//   - map is empty after removal → assert!(map.is_empty())
pub fn remove_key(map: &mut HashMap<String, i32>, key: &str) -> Option<i32> {
    todo!()
}

// --- Exercise 1d ---
// Return every key in `map`, sorted alphabetically.
//
// Inputs:  map — borrowed HashMap (NOT consumed).
// Returns: Vec<String> of every key, in lexicographic ascending order.
//          Note keys must be owned (clone them); the map keeps its ownership.
//
// Edge cases the tests check:
//   - inserts in scrambled order  → output is alphabetically sorted
pub fn sorted_keys(map: &HashMap<String, i32>) -> Vec<String> {
    todo!()
}

// --- Exercise 1e ---
// Merge two maps. If a key appears in both, the result stores the SUM of values.
//
// Inputs:  a, b — owned HashMaps (you may consume both).
// Returns: a new HashMap containing every key from either input. For overlapping
//          keys, the value is a + b's contributions added together.
//
// Edge cases the tests check:
//   - keys only in a              → unchanged
//   - keys only in b              → carried in
//   - keys in both                → sum of the two values (e.g. y: 2 + 3 = 5)
pub fn merge_maps(a: HashMap<String, i32>, b: HashMap<String, i32>) -> HashMap<String, i32> {
    todo!()
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
