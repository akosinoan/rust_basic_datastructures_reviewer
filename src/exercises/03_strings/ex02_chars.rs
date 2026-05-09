// STRINGS - Exercise 2: Char Iteration
//
// In Rust, strings are UTF-8. A single `char` can be 1-4 bytes,
// so indexing by byte position is unsafe (and banned by default).
//
// Always iterate with:
//   s.chars()                        — iterator of char
//   s.chars().collect::<Vec<char>>() — random access via index
//   s.char_indices()                 — yields (byte_offset, char)
//   s.bytes()                        — raw u8 bytes (ASCII-only tricks)
//
// Useful char methods:
//   c.is_alphabetic()
//   c.is_ascii_digit() / c.is_numeric()
//   c.is_uppercase() / c.is_lowercase()
//   c.is_whitespace()
//   c.to_ascii_uppercase()
//   c as u8 / c as u32              — char to number (ASCII)

// --- Exercise 2a ---
// Count vowels (a, e, i, o, u, case-insensitive) in `s`.
//
// Inputs:  s — borrowed &str.
// Returns: usize count of vowel characters.
//
// Edge cases the tests check:
//   - "hello world" → 3
//   - "AEIOU"       → 5  (uppercase counts)
//   - "rhythm"      → 0
//   - ""            → 0
pub fn count_vowels(s: &str) -> usize {
    todo!()
}

// --- Exercise 2b ---
// Reverse `s` character-by-character. Must produce a valid UTF-8 String even when
// the input contains multi-byte characters (do not reverse bytes directly).
//
// Inputs:  s — borrowed &str.
// Returns: String.
//
// Edge cases the tests check:
//   - "hello" → "olleh"
//   - "abcd"  → "dcba"
//   - ""      → ""
//   - "a"     → "a"
pub fn reverse_string(s: &str) -> String {
    todo!()
}

// --- Exercise 2c ---
// Keep only the ALPHABETIC characters of `s`, dropping digits, whitespace,
// punctuation, and any other non-letter.
//
// Inputs:  s — borrowed &str.
// Returns: a new String of letters in original order.
//
// Edge cases the tests check:
//   - "h3ll0 w0rld!" → "hllwrld"
//   - "abc"          → "abc"
//   - "123"          → ""
pub fn letters_only(s: &str) -> String {
    todo!()
}

// --- Exercise 2d ---
// Caesar cipher: shift every alphabetic character by `n` positions (mod 26),
// preserving case. Non-alphabetic characters pass through unchanged.
//
// Inputs:  s — borrowed &str; n — small unsigned shift (n is u8 in 0..255 but
//          the meaningful shifts are mod 26).
// Returns: a new String of the same length (in chars).
//
// Examples:
//   caesar_cipher("abc", 3)            → "def"
//   caesar_cipher("xyz", 3)            → "abc"  (wraparound)
//   caesar_cipher("Hello, World!", 13) → "Uryyb, Jbeyq!"  (ROT13, punctuation untouched)
//   caesar_cipher("abc", 0)            → "abc"
pub fn caesar_cipher(s: &str, n: u8) -> String {
    todo!()
}

// --- Exercise 2e ---
// Compare two strings ignoring case AND ignoring all whitespace.
//
// Inputs:  a, b — borrowed &str values.
// Returns: bool — true iff after lowercasing both AND removing every whitespace
//          char from both, the resulting strings are byte-equal.
//
// Edge cases the tests check:
//   - "Hello World" vs "helloworld" → true
//   - "  ABC  "     vs "abc"        → true
//   - "hello"       vs "world"      → false
pub fn loose_equal(a: &str, b: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_vowels() {
        assert_eq!(count_vowels("hello world"), 3);
        assert_eq!(count_vowels("AEIOU"), 5);
        assert_eq!(count_vowels("rhythm"), 0);
        assert_eq!(count_vowels(""), 0);
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("abcd"), "dcba");
        assert_eq!(reverse_string(""), "");
        assert_eq!(reverse_string("a"), "a");
    }

    #[test]
    fn test_letters_only() {
        assert_eq!(letters_only("h3ll0 w0rld!"), "hllwrld");
        assert_eq!(letters_only("abc"), "abc");
        assert_eq!(letters_only("123"), "");
    }

    #[test]
    fn test_caesar_cipher() {
        assert_eq!(caesar_cipher("abc", 3), "def");
        assert_eq!(caesar_cipher("xyz", 3), "abc");
        assert_eq!(caesar_cipher("Hello, World!", 13), "Uryyb, Jbeyq!");
        assert_eq!(caesar_cipher("abc", 0), "abc");
    }

    #[test]
    fn test_loose_equal() {
        assert!(loose_equal("Hello World", "helloworld"));
        assert!(loose_equal("  ABC  ", "abc"));
        assert!(!loose_equal("hello", "world"));
    }
}
