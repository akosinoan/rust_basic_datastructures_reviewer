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
// Return true if the string is a valid non-empty identifier:
// starts with a letter or underscore, rest are alphanumeric or underscore.
pub fn is_valid_identifier(s: &str) -> bool {
    todo!(
        "Check s.is_empty(), then check the first char with .chars().next(),\
         then check all remaining chars with .chars().all(|c| c.is_alphanumeric() || c == '_')"
    )
}

// --- Exercise 1b ---
// Return the string with the first character capitalized and the rest lowercased.
// Handle empty string.
pub fn capitalize(s: &str) -> String {
    todo!(
        "Use .chars() to get the first char, uppercase it, then collect the rest lowercased.\
         Concatenate with format!() or string push."
    )
}

// --- Exercise 1c ---
// Count how many words are in the string (split by whitespace).
// Consecutive spaces count as one delimiter. Trim edges.
pub fn word_count(s: &str) -> usize {
    todo!("Use .split_whitespace().count()")
}

// --- Exercise 1d ---
// Return true if `s` ends with any of the given suffixes.
pub fn ends_with_any(s: &str, suffixes: &[&str]) -> bool {
    todo!("Use .iter().any(|suf| s.ends_with(suf))")
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
