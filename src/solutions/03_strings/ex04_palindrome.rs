pub fn is_palindrome_exact(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let (mut l, mut r) = (0, chars.len().saturating_sub(1));
    while l < r {
        if chars[l] != chars[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

pub fn is_palindrome_lc(s: &str) -> bool {
    let chars: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let (mut l, mut r) = (0, chars.len().saturating_sub(1));
    while l < r {
        if chars[l] != chars[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}

pub fn is_palindrome_one_deletion(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

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

    let (mut l, mut r) = (0, chars.len().saturating_sub(1));
    while l < r {
        if chars[l] != chars[r] {
            return check(&chars, l + 1, r) || check(&chars, l, r - 1);
        }
        l += 1;
        r -= 1;
    }
    true
}

pub fn all_palindrome_substrings(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut result = Vec::new();
    for i in 0..n {
        for j in i..n {
            let sub: String = chars[i..=j].iter().collect();
            if is_palindrome_exact(&sub) {
                result.push(sub);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome_exact() {
        assert!(is_palindrome_exact("racecar"));
        assert!(is_palindrome_exact("madam"));
        assert!(is_palindrome_exact(""));
        assert!(!is_palindrome_exact("hello"));
    }

    #[test]
    fn test_is_palindrome_lc() {
        assert!(is_palindrome_lc("A man, a plan, a canal: Panama"));
        assert!(!is_palindrome_lc("race a car"));
        assert!(is_palindrome_lc(""));
    }

    #[test]
    fn test_is_palindrome_one_deletion() {
        assert!(is_palindrome_one_deletion("aba"));
        assert!(is_palindrome_one_deletion("abca"));
        assert!(!is_palindrome_one_deletion("abc"));
    }

    #[test]
    fn test_all_palindrome_substrings() {
        let result = all_palindrome_substrings("aab");
        assert!(result.contains(&"a".to_string()));
        assert!(result.contains(&"b".to_string()));
        assert!(result.contains(&"aa".to_string()));
        assert!(!result.contains(&"ab".to_string()));
    }
}
