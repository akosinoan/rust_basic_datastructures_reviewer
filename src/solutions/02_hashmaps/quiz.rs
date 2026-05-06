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
    // Create an empty WordCounter.
    pub fn new() -> Self {
        WordCounter { counts: HashMap::new() }
    }

    // Record one occurrence of a single word (lowercase it first).
    // HINT: self.counts.entry(...).or_insert(0) += 1
    pub fn add_word(&mut self, word: &str) {
        *self.counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    // Split text on whitespace and call add_word for each token.
    pub fn add_text(&mut self, text: &str) {
        text.split_whitespace().for_each(|w| self.add_word(w));
    }

    // Return how many times this word has been seen (0 if never).
    pub fn count(&self, word: &str) -> usize {
        *self.counts.get(&word.to_lowercase()).unwrap_or(&0)
    }

    // Return the word with the highest count, or None if empty.
    // Ties are fine — return any winner.
    // HINT: self.counts.iter().max_by_key(|(_, &v)| v)
    pub fn most_frequent(&self) -> Option<&str> {
        self.counts.iter().max_by_key(|(_, v)| *v).map(|(k, _)| k.as_str())
    }

    // Return all words whose count equals exactly n, sorted alphabetically.
    pub fn words_with_count(&self, n: usize) -> Vec<String> {
        let mut result: Vec<String> = self.counts.iter()
            .filter(|(_, v)| **v == n)
            .map(|(k, _)| k.clone())
            .collect();
        result.sort();
        result
    }

    // Return words that appear in both self and other, sorted alphabetically.
    pub fn intersection(&self, other: &WordCounter) -> Vec<String> {
        let mut result: Vec<String> = self.counts.keys()
            .filter(|k| other.counts.contains_key(*k))
            .cloned()
            .collect();
        result.sort();
        result
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
