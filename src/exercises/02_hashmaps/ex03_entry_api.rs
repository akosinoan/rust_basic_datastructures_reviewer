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
// Increment the count stored under `key`. Initialize to 1 if `key` is not in the map yet.
//
// Inputs:  map — exclusive reference; key — &str to bump.
// Returns: nothing. After N successive calls with the same key, map[key] == N.
//
// Edge cases the tests check:
//   - first call on a fresh key            → value becomes 1
//   - second call on same key              → value becomes 2
//   - call on a different key (existing map) → that key starts at 1
pub fn increment(map: &mut HashMap<String, i32>, key: &str) {
    todo!()
}

// --- Exercise 3b ---
// Sum every i32 in `nums`, bucketed by parity, into a HashMap<String, i32>.
//
// Inputs:  nums — borrowed slice of i32.
// Returns: a HashMap with exactly two keys, "even" and "odd". Each value is
//          the sum of all input numbers in that parity bucket.
//
// Example:
//   nums = [1, 2, 3, 4, 5]
//   → {"even": 2 + 4 = 6, "odd": 1 + 3 + 5 = 9}
pub fn sum_by_parity(nums: &[i32]) -> HashMap<String, i32> {
    todo!()
}

// --- Exercise 3c ---
// Given a list of (student, score) records, keep only the HIGHEST score per student.
//
// Inputs:  records — borrowed slice of (&str, i32). Students may appear multiple times.
// Returns: HashMap<String, i32> mapping student name → max score seen.
//
// Edge cases the tests check:
//   - [("alice",90),("bob",85),("alice",95),("bob",80)]
//     → {"alice": 95, "bob": 85}
pub fn highest_score(records: &[(&str, i32)]) -> HashMap<String, i32> {
    todo!()
}

// --- Exercise 3d ---
// Return the FIRST word that occurs more than once in `sentence`, in scan order.
//
// Inputs:  sentence — &str split on whitespace into tokens.
// Returns: Some(word) when one repeats, otherwise None.
//          The "winner" is the word whose SECOND occurrence is earliest.
//
// Edge cases the tests check:
//   - "the cat sat on the mat" → Some("the")
//   - "one two three"          → None  (every word unique)
//   - "hello hello world"      → Some("hello")
pub fn first_repeated_word(sentence: &str) -> Option<String> {
    todo!()
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
