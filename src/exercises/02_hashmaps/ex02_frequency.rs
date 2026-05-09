// HASHMAPS - Exercise 2: Frequency Counting
//
// Counting occurrences is one of the most common HashMap patterns.
// The idiomatic Rust way:
//
//   *map.entry(key).or_insert(0) += 1;
//
// This means: "get the entry for key; if missing, insert 0; then add 1 to whatever is there."
// You'll use this pattern constantly in LeetCode problems.

use std::collections::HashMap;

// --- Exercise 2a ---
// Count how many times each character appears in `s`.
//
// Inputs:  s — a borrowed &str.
// Returns: HashMap<char, usize> mapping each character that appears in s
//          to the number of occurrences. Characters never seen are ABSENT
//          from the map (not present with count 0).
//
// Edge cases the tests check:
//   - "aabbcc" → every key has count 2
//   - "hello"  → 'l' has count 2, 'h' has count 1
pub fn char_frequency(s: &str) -> HashMap<char, usize> {
    todo!()
}

// --- Exercise 2b ---
// Return the character with the highest frequency in `s`. Ties may resolve to any winner.
//
// Inputs:  s — a borrowed &str (test inputs are non-empty).
// Returns: a single char that has the maximum count.
//
// Edge cases the tests check:
//   - "aabbbcc" → 'b' (the unique max)
//   - "a"       → 'a'
pub fn most_frequent_char(s: &str) -> char {
    todo!()
}

// --- Exercise 2c ---
// Return true iff `a` and `b` use exactly the same characters with exactly the
// same multiplicities (an anagram, case-sensitive, all characters count).
//
// Inputs:  a, b — borrowed &str values.
// Returns: bool.
//
// Edge cases the tests check:
//   - "listen" / "silent"  → true
//   - "anagram" / "nagaram" → true
//   - "rat" / "car"         → false (different characters)
//   - "ab" / "a"            → false (different multiplicities)
pub fn is_anagram(a: &str, b: &str) -> bool {
    todo!()
}

// --- Exercise 2d ---
// Group `words` by their FIRST character.
//
// Inputs:  words — a borrowed slice of &str (each word is non-empty).
// Returns: HashMap<char, Vec<String>>. Each entry's Vec lists every input word
//          whose first char is that key. Each word stored as an owned String.
//          The within-bucket order does not matter (the test sorts before checking).
//
// Edge cases the tests check:
//   - ["apple","avocado","banana","blueberry","cherry"]
//     → groups[&'a'] sorts to ["apple","avocado"]
//     → groups[&'c'].len() == 1
pub fn group_by_first_char(words: &[&str]) -> HashMap<char, Vec<String>> {
    todo!()
}

// --- Exercise 2e ---
// Return every value of `nums` that appears MORE THAN ONCE, with no duplicates in the output.
//
// Inputs:  nums — a borrowed slice of i32.
// Returns: Vec<i32> containing each duplicated value once. Output order is
//          unspecified — the test sorts before comparing.
//
// Edge cases the tests check:
//   - [1,2,3,2,4,3,5] sorted output → [2, 3]
//   - [1,2,3]                       → []
pub fn find_duplicates(nums: &[i32]) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_frequency() {
        let f = char_frequency("aabbcc");
        assert_eq!(f[&'a'], 2);
        assert_eq!(f[&'b'], 2);
        assert_eq!(f[&'c'], 2);

        let f2 = char_frequency("hello");
        assert_eq!(f2[&'l'], 2);
        assert_eq!(f2[&'h'], 1);
    }

    #[test]
    fn test_most_frequent_char() {
        assert_eq!(most_frequent_char("aabbbcc"), 'b');
        assert_eq!(most_frequent_char("a"), 'a');
    }

    #[test]
    fn test_is_anagram() {
        assert!(is_anagram("listen", "silent"));
        assert!(is_anagram("anagram", "nagaram"));
        assert!(!is_anagram("rat", "car"));
        assert!(!is_anagram("ab", "a"));
    }

    #[test]
    fn test_group_by_first_char() {
        let words = ["apple", "avocado", "banana", "blueberry", "cherry"];
        let groups = group_by_first_char(&words);
        let mut a_group = groups[&'a'].clone();
        a_group.sort();
        assert_eq!(a_group, vec!["apple", "avocado"]);
        assert_eq!(groups[&'c'].len(), 1);
    }

    #[test]
    fn test_find_duplicates() {
        let mut result = find_duplicates(&[1, 2, 3, 2, 4, 3, 5]);
        result.sort();
        assert_eq!(result, vec![2, 3]);

        assert_eq!(find_duplicates(&[1, 2, 3]), vec![]);
    }
}
