// STRINGS - Exercise 4: Palindrome Problems
//
// A palindrome reads the same forwards and backwards.
// Strategy: collect to Vec<char>, then use two-pointer check.
//
// Gotcha: "A man a plan a canal Panama" IS a palindrome if we
// ignore case and non-alphanumeric characters. LeetCode does this.

// --- Exercise 4a ---
// Strict palindrome check (case-sensitive, every character counts).
//
// Inputs:  s — borrowed &str.
// Returns: true iff s.chars() reads the same forwards and backwards.
//
// Edge cases the tests check:
//   - "racecar" → true
//   - "madam"   → true
//   - ""        → true (empty is a palindrome)
//   - "a"       → true (single char is a palindrome)
//   - "hello"   → false
//   - "Racecar" → false (case-sensitive: 'R' ≠ 'r')
pub fn is_palindrome_exact(s: &str) -> bool {
    todo!()
}

// --- Exercise 4b ---
// LeetCode #125: palindrome ignoring case AND non-alphanumeric characters.
//
// Inputs:  s — borrowed &str (may contain spaces, punctuation, mixed case).
// Returns: true iff after filtering to alphanumeric chars and lowercasing,
//          the result is a palindrome.
//
// Edge cases the tests check:
//   - "A man, a plan, a canal: Panama" → true
//   - "race a car"                     → false
//   - ""                               → true
//   - " " (a single space)             → true (filters to empty → palindrome)
pub fn is_palindrome_lc(s: &str) -> bool {
    todo!()
}

// --- Exercise 4c ---
// LeetCode #680: palindrome after AT MOST one character deletion.
//
// Inputs:  s — borrowed &str.
// Returns: true iff s is already a palindrome OR becomes one after removing
//          exactly one character at any position.
//
// A small helper `check` is provided. Use it with two index ranges (skip the left
// or skip the right) the moment your two pointers find a mismatch.
//
// Edge cases the tests check:
//   - "aba"      → true (already palindrome)
//   - "abca"     → true (remove 'c' → "aba")
//   - "abc"      → false
//   - "raceacar" → true (remove the first 'a' → "racecar")
pub fn is_palindrome_one_deletion(s: &str) -> bool {
    fn check(chars: &[char], mut l: usize, mut r: usize) -> bool {
        while l < r {
            if chars[l] != chars[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }

    todo!()
}

// --- Exercise 4d ---
// Return every contiguous substring of `s` that is itself a palindrome
// (single characters included, length ≥ 1).
//
// Inputs:  s — borrowed &str.
// Returns: Vec<String> of palindrome substrings. Order is not strictly checked
//          beyond the test assertions below; brute force is acceptable.
//
// Edge cases the tests check:
//   - "aab" → result includes "a", "b", and "aa" but NOT "ab"
//     (the test uses .contains() so other palindromes like the second "a"
//     may or may not be present; just include the obvious ones)
pub fn all_palindrome_substrings(s: &str) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_exact() {
        assert!(is_palindrome_exact("racecar"));
        assert!(is_palindrome_exact("madam"));
        assert!(is_palindrome_exact(""));
        assert!(is_palindrome_exact("a"));
        assert!(!is_palindrome_exact("hello"));
        assert!(!is_palindrome_exact("Racecar")); // case sensitive
    }

    #[test]
    fn test_is_palindrome_lc() {
        assert!(is_palindrome_lc("A man, a plan, a canal: Panama"));
        assert!(!is_palindrome_lc("race a car"));
        assert!(is_palindrome_lc(""));
        assert!(is_palindrome_lc(" "));
    }

    #[test]
    fn test_is_palindrome_one_deletion() {
        assert!(is_palindrome_one_deletion("aba"));
        assert!(is_palindrome_one_deletion("abca")); // remove 'c'
        assert!(!is_palindrome_one_deletion("abc"));
        assert!(is_palindrome_one_deletion("raceacar")); // remove first 'a'
    }

    #[test]
    fn test_all_palindrome_substrings() {
        let result = all_palindrome_substrings("aab");
        // single chars: "a","a","b" + substrings: "aa"
        assert!(result.contains(&"a".to_string()));
        assert!(result.contains(&"b".to_string()));
        assert!(result.contains(&"aa".to_string()));
        assert!(!result.contains(&"ab".to_string()));
    }
}
