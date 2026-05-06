pub fn count_vowels(s: &str) -> usize {
    s.chars().filter(|c| "aeiouAEIOU".contains(*c)).count()
}

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn letters_only(s: &str) -> String {
    s.chars().filter(|c| c.is_alphabetic()).collect()
}

pub fn caesar_cipher(s: &str, n: u8) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                (((c as u8 - b'a' + n) % 26) + b'a') as char
            } else if c.is_ascii_uppercase() {
                (((c as u8 - b'A' + n) % 26) + b'A') as char
            } else {
                c
            }
        })
        .collect()
}

pub fn loose_equal(a: &str, b: &str) -> bool {
    let normalize = |s: &str| -> String {
        s.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .to_lowercase()
    };
    normalize(a) == normalize(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_vowels() {
        assert_eq!(count_vowels("hello world"), 3);
        assert_eq!(count_vowels("AEIOU"), 5);
        assert_eq!(count_vowels("rhythm"), 0);
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("abcd"), "dcba");
        assert_eq!(reverse_string(""), "");
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
    }

    #[test]
    fn test_loose_equal() {
        assert!(loose_equal("Hello World", "helloworld"));
        assert!(loose_equal("  ABC  ", "abc"));
        assert!(!loose_equal("hello", "world"));
    }
}
