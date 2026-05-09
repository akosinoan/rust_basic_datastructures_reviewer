// STRINGS - Exercise 1: String vs &str
//
// Rust has two string types:
//   &str   — borrowed string slice (immutable view, stored anywhere)
//   String — owned, heap-allocated, growable
//
// Convert between them:
//   "hello".to_string()    — &str → String
//   s.as_str()             — String → &str
//   &s                     — String → &str (auto-deref)
//
// Key methods (work on both via &str):
//   .len()           — byte length (not char count!)
//   .is_empty()
//   .contains("x")
//   .starts_with("x") / .ends_with("x")
//   .to_uppercase() / .to_lowercase()  — return new String
//   .trim()          — strip leading/trailing whitespace → &str
//   .repeat(n)       — repeat n times → String

// --- Exercise 1a ---
// Return true iff `s` is a valid identifier in the C/Rust sense.
//
// Inputs:  s — borrowed &str.
// Returns: true iff
//   1. s is non-empty
//   2. the first character is a letter or '_'
//   3. every remaining character is alphanumeric or '_'
//
// Edge cases the tests check:
//   - "hello"      → true
//   - "_private"   → true (leading underscore allowed)
//   - "foo_bar2"   → true (digits allowed AFTER first char)
//   - ""           → false (empty)
//   - "2bad"       → false (starts with a digit)
//   - "has space"  → false (whitespace)
//   - "has-dash"   → false ('-' is not alphanumeric or '_')
pub fn is_valid_identifier(s: &str) -> bool {
    todo!()
}

// --- Exercise 1b ---
// Capitalize: first character to UPPERCASE, every other character to lowercase.
//
// Inputs:  s — borrowed &str.
// Returns: a new owned String. The empty string maps to "".
//
// Edge cases the tests check:
//   - "hello world" → "Hello world"
//   - "HELLO"       → "Hello"
//   - ""            → ""
//   - "a"           → "A"
pub fn capitalize(s: &str) -> String {
    todo!()
}

// --- Exercise 1c ---
// Count whitespace-separated words. Runs of whitespace count as one delimiter,
// and leading/trailing whitespace does not produce empty words.
//
// Inputs:  s — borrowed &str.
// Returns: number of words.
//
// Edge cases the tests check:
//   - "hello world"            → 2
//   - "  spaces   everywhere  "→ 2  (double spaces and edge whitespace ignored)
//   - "one"                    → 1
//   - ""                       → 0
pub fn word_count(s: &str) -> usize {
    todo!()
}

// --- Exercise 1d ---
// Return true iff `s` ends with at least one of `suffixes`.
//
// Inputs:  s — borrowed &str; suffixes — borrowed slice of &str.
// Returns: bool.
//
// Edge cases the tests check:
//   - "image.png" with ["png","jpg","gif"] → true
//   - "doc.pdf" with the same list         → false
//   - "" with ["png"]                      → false
pub fn ends_with_any(s: &str, suffixes: &[&str]) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_identifier() {
        assert!(is_valid_identifier("hello"));
        assert!(is_valid_identifier("_private"));
        assert!(is_valid_identifier("foo_bar2"));
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("2bad"));
        assert!(!is_valid_identifier("has space"));
        assert!(!is_valid_identifier("has-dash"));
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello world"), "Hello world");
        assert_eq!(capitalize("HELLO"), "Hello");
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("a"), "A");
    }

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("hello world"), 2);
        assert_eq!(word_count("  spaces   everywhere  "), 2);
        assert_eq!(word_count("one"), 1);
        assert_eq!(word_count(""), 0);
    }

    #[test]
    fn test_ends_with_any() {
        assert!(ends_with_any("image.png", &["png", "jpg", "gif"]));
        assert!(ends_with_any("photo.jpg", &["png", "jpg", "gif"]));
        assert!(!ends_with_any("doc.pdf", &["png", "jpg", "gif"]));
        assert!(!ends_with_any("", &["png"]));
    }
}
