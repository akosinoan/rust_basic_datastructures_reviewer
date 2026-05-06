// STRINGS - Exercise 3: Manipulation
//
// Key methods for splitting and building strings:
//   s.split("delim")          — iterator of &str parts
//   s.split_whitespace()      — split on any whitespace
//   s.splitn(n, "delim")      — at most n parts
//   parts.join("delim")       — join a slice of strings
//   s.replace("old", "new")   — replace all occurrences
//   s.replacen("old", "new", n) — replace first n
//   s.trim() / .trim_start() / .trim_end()
//   format!("{}{}", a, b)     — build a new String
//   s.push_str("more")        — append to a String
//   s.push('c')               — append a single char

// --- Exercise 3a ---
// Given a CSV line like "alice,30,engineer", split by comma and return the parts.
pub fn split_csv(line: &str) -> Vec<String> {
    todo!("Use .split(',').map(str::to_string).collect()")
}

// --- Exercise 3b ---
// Reverse the order of words in a sentence.
// "hello world foo" → "foo world hello"
pub fn reverse_words(s: &str) -> String {
    todo!(
        "Split on whitespace, collect into Vec, reverse, join with space.\
         Trim the input first."
    )
}

// --- Exercise 3c ---
// Convert a snake_case string to camelCase.
// "hello_world_foo" → "helloWorldFoo"
// HINT: split by '_', capitalize all parts except the first, then join
pub fn snake_to_camel(s: &str) -> String {
    todo!(
        "Split on '_', take first part as-is, capitalize the rest,\
         then concatenate all parts."
    )
}

// --- Exercise 3d ---
// Return the longest common prefix of a list of strings.
// If none, return "".
// HINT: start with the first string, shrink it until all strings start with it.
pub fn longest_common_prefix(strs: &[&str]) -> String {
    todo!(
        "Take strs[0] as prefix. For each other string, shrink prefix\
         while !string.starts_with(prefix)."
    )
}

// --- Exercise 3e ---
// Run-length encoding: compress "aaabbbccddddee" → "3a3b2c4d2e".
// For a single char (count=1), still include the count: "a" → "1a".
pub fn run_length_encode(s: &str) -> String {
    todo!(
        "Walk through chars with a counter. When the char changes,\
         push the count and char to the result, then reset."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_csv() {
        assert_eq!(split_csv("alice,30,engineer"), vec!["alice", "30", "engineer"]);
        assert_eq!(split_csv("a"), vec!["a"]);
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
        assert_eq!(longest_common_prefix(&["abc"]), "abc");
    }

    #[test]
    fn test_run_length_encode() {
        assert_eq!(run_length_encode("aaabbbcc"), "3a3b2c");
        assert_eq!(run_length_encode("abcd"), "1a1b1c1d");
        assert_eq!(run_length_encode(""), "");
        assert_eq!(run_length_encode("aaaa"), "4a");
    }
}
