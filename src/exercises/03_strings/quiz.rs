// QUIZ — Section 3: Text Analyzer
//
// Build a TextAnalyzer that wraps a string and answers questions about it.
// You will use &str vs String ownership, chars() iteration, split_whitespace,
// frequency maps, and run-length encoding — everything from this section.

use std::collections::HashMap;

pub struct TextAnalyzer {
    text: String,
}

impl TextAnalyzer {
    // Store the text (convert &str to owned String).
    pub fn new(text: &str) -> Self {
        todo!("Store text as an owned String")
    }

    // Number of whitespace-separated words.
    pub fn word_count(&self) -> usize {
        todo!("self.text.split_whitespace().count()")
    }

    // Frequency of each character (including spaces and punctuation).
    pub fn char_frequency(&self) -> HashMap<char, usize> {
        todo!("iterate chars(), build frequency HashMap")
    }

    // The longest word by character count, or None if the text is empty.
    // Ties: return whichever comes first.
    // HINT: split_whitespace().max_by_key(|w| w.len())
    pub fn longest_word(&self) -> Option<&str> {
        todo!("split_whitespace, find max by length")
    }

    // True if the text contains every letter a–z at least once (case-insensitive).
    // HINT: collect lowercase chars into a HashSet, check len == 26
    pub fn is_pangram(&self) -> bool {
        todo!("collect unique lowercase alpha chars, is the set size 26?")
    }

    // Run-length encode self.text: consecutive identical characters are
    // replaced by their count followed by the character.
    // Example: "aaabbc" → "3a2b1c"
    // HINT: iterate chars(), track (current_char, run_length), flush on change
    pub fn run_length_encode(&self) -> String {
        todo!("count consecutive chars, write count+char to output String")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        let ta = TextAnalyzer::new("the quick brown fox");
        assert_eq!(ta.word_count(), 4);
    }

    #[test]
    fn test_word_count_empty() {
        let ta = TextAnalyzer::new("");
        assert_eq!(ta.word_count(), 0);
    }

    #[test]
    fn test_char_frequency() {
        let ta = TextAnalyzer::new("aab");
        let freq = ta.char_frequency();
        assert_eq!(freq[&'a'], 2);
        assert_eq!(freq[&'b'], 1);
    }

    #[test]
    fn test_longest_word() {
        let ta = TextAnalyzer::new("I love Rust programming");
        assert_eq!(ta.longest_word(), Some("programming"));
    }

    #[test]
    fn test_longest_word_empty() {
        let ta = TextAnalyzer::new("");
        assert_eq!(ta.longest_word(), None);
    }

    #[test]
    fn test_is_pangram_true() {
        let ta = TextAnalyzer::new(
            "the quick brown fox jumps over the lazy dog",
        );
        assert!(ta.is_pangram());
    }

    #[test]
    fn test_is_pangram_false() {
        let ta = TextAnalyzer::new("hello world");
        assert!(!ta.is_pangram());
    }

    #[test]
    fn test_run_length_encode() {
        let ta = TextAnalyzer::new("aaabbc");
        assert_eq!(ta.run_length_encode(), "3a2b1c");
    }

    #[test]
    fn test_run_length_encode_no_repeats() {
        let ta = TextAnalyzer::new("abc");
        assert_eq!(ta.run_length_encode(), "1a1b1c");
    }

    #[test]
    fn test_run_length_encode_empty() {
        let ta = TextAnalyzer::new("");
        assert_eq!(ta.run_length_encode(), "");
    }
}
