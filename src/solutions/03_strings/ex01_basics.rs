pub fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    (first.is_alphabetic() || first == '_') && chars.all(|c| c.is_alphanumeric() || c == '_')
}

pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + &chars.as_str().to_lowercase(),
    }
}

pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

pub fn ends_with_any(s: &str, suffixes: &[&str]) -> bool {
    suffixes.iter().any(|suf| s.ends_with(suf))
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
    }
}
