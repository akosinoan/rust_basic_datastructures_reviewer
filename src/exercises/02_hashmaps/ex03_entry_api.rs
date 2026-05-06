// HASHMAPS - Exercise 3: The Entry API
//
// The entry API lets you read and write in one lookup (no double-hashing).
//
//   map.entry(key).or_insert(value)        — insert default if missing, return &mut V
//   map.entry(key).or_default()            — insert Default::default() if missing
//   map.entry(key).or_insert_with(|| ...) — insert lazily if missing
//   map.entry(key).and_modify(|v| ...)    — modify only if present
//   map.entry(key).and_modify(|v| *v += 1).or_insert(1) — increment or initialize
//
// This is the IDIOMATIC way to do conditional inserts. Avoid get+insert pairs.

use std::collections::HashMap;

// --- Exercise 3a ---
// Increment the count for `key`. If not present, initialize to 1.
// This is the single most important entry API pattern.
pub fn increment(map: &mut HashMap<String, i32>, key: &str) {
    todo!("Use .entry().and_modify().or_insert() to increment or initialize")
}

// --- Exercise 3b ---
// For each number in `nums`, track the running sum keyed by "even" or "odd".
// Return the map with two keys: "even" and "odd".
pub fn sum_by_parity(nums: &[i32]) -> HashMap<String, i32> {
    todo!(
        "For each num, if even add to map[\"even\"], else add to map[\"odd\"].\
         Use entry().or_insert(0) then add."
    )
}

// --- Exercise 3c ---
// Given a list of (student, score) pairs, keep only the HIGHEST score per student.
// If a student appears multiple times, keep the max.
pub fn highest_score(records: &[(&str, i32)]) -> HashMap<String, i32> {
    todo!(
        "Use entry().and_modify(|old| if score > *old {{ *old = score }}).or_insert(score)"
    )
}

// --- Exercise 3d ---
// Given a sentence, return the first word that appears more than once.
// Return None if all words are unique.
pub fn first_repeated_word(sentence: &str) -> Option<String> {
    todo!(
        "Split on whitespace, count each word, return the first where count becomes 2"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let mut m: HashMap<String, i32> = HashMap::new();
        increment(&mut m, "a");
        assert_eq!(m["a"], 1);
        increment(&mut m, "a");
        assert_eq!(m["a"], 2);
        increment(&mut m, "b");
        assert_eq!(m["b"], 1);
    }

    #[test]
    fn test_sum_by_parity() {
        let m = sum_by_parity(&[1, 2, 3, 4, 5]);
        assert_eq!(m["even"], 6);  // 2+4
        assert_eq!(m["odd"], 9);   // 1+3+5
    }

    #[test]
    fn test_highest_score() {
        let records = [("alice", 90), ("bob", 85), ("alice", 95), ("bob", 80)];
        let m = highest_score(&records);
        assert_eq!(m["alice"], 95);
        assert_eq!(m["bob"], 85);
    }

    #[test]
    fn test_first_repeated_word() {
        assert_eq!(
            first_repeated_word("the cat sat on the mat"),
            Some("the".to_string())
        );
        assert_eq!(
            first_repeated_word("one two three"),
            None
        );
        assert_eq!(
            first_repeated_word("hello hello world"),
            Some("hello".to_string())
        );
    }
}
