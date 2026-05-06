pub fn split_csv(line: &str) -> Vec<String> {
    line.split(',').map(str::to_string).collect()
}

pub fn reverse_words(s: &str) -> String {
    s.split_whitespace()
        .rev()
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn snake_to_camel(s: &str) -> String {
    let mut parts = s.split('_');
    let first = parts.next().unwrap_or("").to_string();
    let rest: String = parts
        .map(|p| {
            let mut c = p.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().to_string() + c.as_str(),
            }
        })
        .collect();
    first + &rest
}

pub fn longest_common_prefix(strs: &[&str]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut prefix = strs[0].to_string();
    for s in &strs[1..] {
        while !s.starts_with(prefix.as_str()) {
            prefix.pop();
            if prefix.is_empty() {
                return String::new();
            }
        }
    }
    prefix
}

pub fn run_length_encode(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let mut result = String::new();
    let mut chars = s.chars();
    let mut current = chars.next().unwrap();
    let mut count = 1usize;
    for c in chars {
        if c == current {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push(current);
            current = c;
            count = 1;
        }
    }
    result.push_str(&count.to_string());
    result.push(current);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_csv() {
        assert_eq!(split_csv("alice,30,engineer"), vec!["alice", "30", "engineer"]);
    }

    #[test]
    fn test_reverse_words() {
        assert_eq!(reverse_words("hello world foo"), "foo world hello");
        assert_eq!(reverse_words("  spaces  ahead  "), "ahead spaces");
        assert_eq!(reverse_words("single"), "single");
    }

    #[test]
    fn test_snake_to_camel() {
        assert_eq!(snake_to_camel("hello_world"), "helloWorld");
        assert_eq!(snake_to_camel("foo_bar_baz"), "fooBarBaz");
        assert_eq!(snake_to_camel("simple"), "simple");
    }

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(longest_common_prefix(&["flower", "flow", "flight"]), "fl");
        assert_eq!(longest_common_prefix(&["dog", "racecar", "car"]), "");
        assert_eq!(longest_common_prefix(&["interview", "interact", "interface"]), "inter");
    }

    #[test]
    fn test_run_length_encode() {
        assert_eq!(run_length_encode("aaabbbcc"), "3a3b2c");
        assert_eq!(run_length_encode("abcd"), "1a1b1c1d");
        assert_eq!(run_length_encode(""), "");
        assert_eq!(run_length_encode("aaaa"), "4a");
    }
}
