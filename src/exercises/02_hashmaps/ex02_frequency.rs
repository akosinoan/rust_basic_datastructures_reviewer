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
// Count how many times each character appears in the string.
// Return a HashMap<char, usize>.
pub fn char_frequency(s: &str) -> HashMap<char, usize> {
    todo!("Iterate over s.chars() and count each character")
}

// --- Exercise 2b ---
// Return the most frequent character. If there's a tie, return any one of them.
// HINT: build the frequency map first, then find the entry with max value
pub fn most_frequent_char(s: &str) -> char {
    todo!("Build char_frequency, then use .max_by_key(|(_, count)| *count)")
}

// --- Exercise 2c ---
// Given two strings, return true if they are anagrams of each other.
// An anagram uses the same characters the same number of times.
// HINT: their character frequency maps must be equal
pub fn is_anagram(a: &str, b: &str) -> bool {
    todo!("Compare the frequency maps of both strings")
}

// --- Exercise 2d ---
// Given a list of words, group them by their first character.
// Return HashMap<char, Vec<String>>.
// HINT: use .entry(first_char).or_insert_with(Vec::new).push(word)
pub fn group_by_first_char(words: &[&str]) -> HashMap<char, Vec<String>> {
    todo!("For each word, get its first char and push into the correct group")
}

// --- Exercise 2e ---
// Return all elements that appear more than once.
// Order does not matter.
pub fn find_duplicates(nums: &[i32]) -> Vec<i32> {
    todo!(
        "Count frequencies, then collect elements where count > 1.\
         Remember to deduplicate the output."
    )
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
