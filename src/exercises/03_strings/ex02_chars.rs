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
// Count the number of vowels (a, e, i, o, u, case insensitive).
pub fn count_vowels(s: &str) -> usize {
    todo!("Iterate chars, check if it's in \"aeiouAEIOU\"")
}

// --- Exercise 2b ---
// Reverse a string (character by character, not byte by byte).
pub fn reverse_string(s: &str) -> String {
    todo!("Collect chars into a Vec, reverse it, collect back into String")
}

// --- Exercise 2c ---
// Return only the alphabetic characters (remove digits, spaces, punctuation).
pub fn letters_only(s: &str) -> String {
    todo!("Filter chars by c.is_alphabetic()")
}

// --- Exercise 2d ---
// Caesar cipher: shift every letter by `n` positions in the alphabet.
// Wrap around: 'z' + 1 = 'a'. Non-letter chars are unchanged.
// Handle both uppercase and lowercase.
// HINT: for lowercase: ((c as u8 - b'a' + n as u8) % 26 + b'a') as char
pub fn caesar_cipher(s: &str, n: u8) -> String {
    todo!(
        "For each char: if lowercase, shift within a-z; \
         if uppercase, shift within A-Z; else keep as-is."
    )
}

// --- Exercise 2e ---
// Return true if two strings are equal ignoring case AND ignoring all whitespace.
pub fn loose_equal(a: &str, b: &str) -> bool {
    todo!(
        "Filter whitespace from both strings, lowercase both, then compare.\
         HINT: filter(|c| !c.is_whitespace()).collect::<String>().to_lowercase()"
    )
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
