// QUIZ — Section 2: Word Counter
//
// Build a WordCounter that tracks how often words appear in text.
// You will use HashMap insert/get, the entry API, frequency counting,
// and set-intersection — everything from this section.
//
// Words are case-insensitive: "Hello" and "hello" count as the same word.

use std::collections::HashMap;

pub struct WordCounter {
    counts: HashMap<String, usize>,
}

impl WordCounter {
    // Construct a fresh empty WordCounter.
    //
    // Returns: a WordCounter whose internal counts map is empty.
    pub fn new() -> Self {
        todo!()
    }

    // Record one occurrence of a single word.
    //
    // Inputs:  word — &str. Treat it case-insensitively (normalise before counting).
    // Returns: nothing. After three calls with "Hello"/"HELLO"/"hello",
    //          self.count("hello") must be 3.
    pub fn add_word(&mut self, word: &str) {
        todo!()
    }

    // Split `text` on whitespace and add_word every token.
    //
    // Inputs:  text — &str of zero or more whitespace-separated words.
    // Returns: nothing.
    pub fn add_text(&mut self, text: &str) {
        todo!()
    }

    // Look up how many times `word` has been seen (case-insensitive).
    //
    // Inputs:  word — &str.
    // Returns: 0 if never seen, else the recorded count.
    pub fn count(&self, word: &str) -> usize {
        todo!()
    }

    // Return the word with the highest count, or None if no words have been recorded.
    //
    // Returns: Option<&str> referring into the internal storage. Ties may resolve
    //          to any of the tied winners.
    //
    // Edge cases the tests check:
    //   - empty            → None
    //   - clear winner     → Some(that word)
    pub fn most_frequent(&self) -> Option<&str> {
        todo!()
    }

    // Return every recorded word whose count equals exactly `n`, sorted alphabetically.
    //
    // Inputs:  n — required count, usize.
    // Returns: Vec<String> of matching words in lexicographic order.
    //
    // Edge cases the tests check:
    //   - empty counter → empty vec (typed Vec<String>)
    //   - multiple words tied at count n → all of them, in alphabetical order
    pub fn words_with_count(&self, n: usize) -> Vec<String> {
        todo!()
    }

    // Return every word that exists in BOTH `self` and `other`, sorted alphabetically.
    //
    // Inputs:  other — &WordCounter, treated as a second multiset of words.
    // Returns: Vec<String> of words present in both, in lexicographic order.
    //          Counts are ignored — only key membership matters.
    //
    // Edge cases the tests check:
    //   - both have ("rust","python") in common → ["python","rust"]
    //   - one is empty                          → []
    pub fn intersection(&self, other: &WordCounter) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let wc = WordCounter::new();
        assert_eq!(wc.count("hello"), 0);
        assert_eq!(wc.most_frequent(), None);
        assert_eq!(wc.words_with_count(1), Vec::<String>::new());
    }

    #[test]
    fn test_add_word_case_insensitive() {
        let mut wc = WordCounter::new();
        wc.add_word("Hello");
        wc.add_word("HELLO");
        wc.add_word("hello");
        assert_eq!(wc.count("hello"), 3);
    }

    #[test]
    fn test_add_text() {
        let mut wc = WordCounter::new();
        wc.add_text("the cat sat on the mat");
        assert_eq!(wc.count("the"), 2);
        assert_eq!(wc.count("cat"), 1);
        assert_eq!(wc.count("mat"), 1);
    }

    #[test]
    fn test_most_frequent() {
        let mut wc = WordCounter::new();
        wc.add_text("apple banana apple cherry apple banana");
        assert_eq!(wc.most_frequent(), Some("apple"));
    }

    #[test]
    fn test_words_with_count() {
        let mut wc = WordCounter::new();
        wc.add_text("a b b c c c");
        assert_eq!(wc.words_with_count(1), vec!["a"]);
        assert_eq!(wc.words_with_count(2), vec!["b"]);
        assert_eq!(wc.words_with_count(3), vec!["c"]);
    }

    #[test]
    fn test_words_with_count_sorted() {
        let mut wc = WordCounter::new();
        wc.add_text("zebra apple mango apple zebra mango");
        let mut result = wc.words_with_count(2);
        result.sort();
        assert_eq!(result, vec!["apple", "mango", "zebra"]);
    }

    #[test]
    fn test_intersection() {
        let mut a = WordCounter::new();
        a.add_text("rust python go");

        let mut b = WordCounter::new();
        b.add_text("python java rust");

        let mut common = a.intersection(&b);
        common.sort();
        assert_eq!(common, vec!["python", "rust"]);
    }

    #[test]
    fn test_intersection_empty() {
        let mut a = WordCounter::new();
        a.add_word("foo");
        let b = WordCounter::new();
        assert_eq!(a.intersection(&b), Vec::<String>::new());
    }
}
