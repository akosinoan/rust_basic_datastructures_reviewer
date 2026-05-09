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
// Split a single CSV-style line on commas.
//
// Inputs:  line — borrowed &str like "alice,30,engineer".
// Returns: Vec<String> of the comma-separated fields, in order, owned.
//
// Edge cases the tests check:
//   - "alice,30,engineer" → ["alice","30","engineer"]
//   - "a"                 → ["a"]   (no commas → single field)
pub fn split_csv(line: &str) -> Vec<String> {
    todo!()
}

// --- Exercise 3b ---
// Reverse the WORD order of `s`. Words are separated by runs of whitespace.
//
// Inputs:  s — borrowed &str.
// Returns: a String with the words in reverse order, separated by single spaces.
//          Leading/trailing whitespace and runs of whitespace must NOT produce
//          empty words; "  spaces  ahead  " has two words.
//
// Edge cases the tests check:
//   - "hello world foo"     → "foo world hello"
//   - "  spaces  ahead  "   → "ahead spaces"
//   - "single"              → "single"
pub fn reverse_words(s: &str) -> String {
    todo!()
}

// --- Exercise 3c ---
// Convert snake_case to camelCase.
//
// Inputs:  s — borrowed &str of lowercase letters and underscores. The first
//          run is lowercased; every subsequent run capitalizes its first char.
// Returns: String. No underscores remain.
//
// Edge cases the tests check:
//   - "hello_world"      → "helloWorld"
//   - "foo_bar_baz"      → "fooBarBaz"
//   - "simple" (no '_')  → "simple"
pub fn snake_to_camel(s: &str) -> String {
    todo!()
}

// --- Exercise 3d ---
// Longest common prefix of every string in `strs`.
//
// Inputs:  strs — borrowed slice of &str (input is non-empty for the asserts).
// Returns: a String containing the longest prefix that every input string
//          starts with. Empty if there is no common starting character.
//
// Edge cases the tests check:
//   - ["flower","flow","flight"]                     → "fl"
//   - ["dog","racecar","car"]                        → ""    (no common start)
//   - ["interview","interact","interface"]           → "inter"
//   - ["abc"]                                        → "abc" (single string is the prefix)
pub fn longest_common_prefix(strs: &[&str]) -> String {
    todo!()
}

// --- Exercise 3e ---
// Run-length encode `s`: each maximal run of one character becomes <count><char>.
// Counts of 1 are NOT omitted ("a" → "1a"), so the encoding is fully reversible.
//
// Inputs:  s — borrowed &str.
// Returns: String such that "aaabbcc" → "3a2b2c", "abcd" → "1a1b1c1d", "" → "".
//
// Edge cases the tests check:
//   - "aaabbcc" → "3a2b2c"
//   - "abcd"    → "1a1b1c1d"
//   - ""        → ""
//   - "aaaa"    → "4a"
pub fn run_length_encode(s: &str) -> String {
    todo!()
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
        assert_eq!(run_length_encode("aaabbcc"), "3a2b2c");
        assert_eq!(run_length_encode("abcd"), "1a1b1c1d");
        assert_eq!(run_length_encode(""), "");
        assert_eq!(run_length_encode("aaaa"), "4a");
    }
}
