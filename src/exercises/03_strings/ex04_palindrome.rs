// STRINGS - Exercise 4: Palindrome Problems
//
// A palindrome reads the same forwards and backwards.
// Strategy: collect to Vec<char>, then use two-pointer check.
//
// Gotcha: "A man a plan a canal Panama" IS a palindrome if we
// ignore case and non-alphanumeric characters. LeetCode does this.

// --- Exercise 4a ---
// Return true if the string is a palindrome (exact match, case-sensitive).
pub fn is_palindrome_exact(s: &str) -> bool {
    todo!(
        "Collect to Vec<char>. Use two pointers (left, right).\
         While left < right: if chars differ, return false."
    )
}

// --- Exercise 4b ---
// Return true if the string is a palindrome ignoring case and non-alphanumeric chars.
// This is LeetCode #125.
pub fn is_palindrome_lc(s: &str) -> bool {
    todo!(
        "Filter to only alphanumeric chars using c.is_alphanumeric(),\
         lowercase, collect to Vec<char>, then two-pointer check."
    )
}

// --- Exercise 4c ---
// Return true if the string is a palindrome after removing AT MOST one character.
// This is LeetCode #680.
// HINT: two pointers. If chars match, move both. If they don't,
//       try removing the left char OR the right char and check if either substring is a palindrome.
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

    todo!(
        "Collect to Vec<char>. Two pointers l=0, r=len-1.\
         When chars[l] != chars[r], return check(l+1,r) || check(l,r-1)."
    )
}

// --- Exercise 4d ---
// Find ALL substrings of s that are palindromes. Return them in order of start position.
// (If a substring of length >= 2, include it; include single chars too.)
// This is the brute force approach — O(n³) is fine for now.
pub fn all_palindrome_substrings(s: &str) -> Vec<String> {
    todo!(
        "For each start i, for each end j > i,\
         check if s[i..j] is a palindrome using is_palindrome_exact.\
         Collect the matching substrings.\
         HINT: collect chars to Vec<char> first, then build substrings from slices."
    )
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
