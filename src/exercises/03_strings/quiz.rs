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
    // Construct a TextAnalyzer holding an OWNED copy of `text`.
    //
    // Inputs:  text — &str (caller still owns the original).
    // Returns: TextAnalyzer whose `text` field stores an owned String.
    pub fn new(text: &str) -> Self {
        todo!()
    }

    // Number of whitespace-separated words in the stored text.
    //
    // Returns: usize. "" → 0; "the quick brown fox" → 4.
    pub fn word_count(&self) -> usize {
        todo!()
    }

    // Frequency of every CHARACTER in the stored text (spaces and punctuation included).
    //
    // Returns: HashMap<char, usize>. Characters never seen are absent (not zero).
    //
    // Edge cases the tests check:
    //   - "aab" → {'a':2, 'b':1}
    pub fn char_frequency(&self) -> HashMap<char, usize> {
        todo!()
    }

    // Longest word in the text by char count, or None if there are no words.
    //
    // Returns: Option<&str> referencing into self.text. Ties resolve to whichever
    //          word the iteration encounters first.
    //
    // Edge cases the tests check:
    //   - "I love Rust programming" → Some("programming")
    //   - ""                        → None
    pub fn longest_word(&self) -> Option<&str> {
        todo!()
    }

    // Pangram check: does the text contain every English letter a-z (case-insensitive)?
    //
    // Returns: bool.
    //
    // Edge cases the tests check:
    //   - "the quick brown fox jumps over the lazy dog" → true
    //   - "hello world"                                  → false
    pub fn is_pangram(&self) -> bool {
        todo!()
    }

    // Run-length encode the stored text: maximal runs of one character become
    // <count><char>. Counts of 1 are kept (so "abc" → "1a1b1c"), making the
    // encoding fully reversible.
    //
    // Returns: String.
    //
    // Edge cases the tests check:
    //   - "aaabbc" → "3a2b1c"
    //   - "abc"    → "1a1b1c"
    //   - ""       → ""
    pub fn run_length_encode(&self) -> String {
        todo!()
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
