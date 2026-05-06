use std::collections::HashMap;

pub fn char_frequency(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}

pub fn most_frequent_char(s: &str) -> char {
    char_frequency(s)
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(c, _)| c)
        .expect("non-empty string")
}

pub fn is_anagram(a: &str, b: &str) -> bool {
    char_frequency(a) == char_frequency(b)
}

pub fn group_by_first_char(words: &[&str]) -> HashMap<char, Vec<String>> {
    let mut map: HashMap<char, Vec<String>> = HashMap::new();
    for &word in words {
        if let Some(first) = word.chars().next() {
            map.entry(first).or_default().push(word.to_string());
        }
    }
    map
}

pub fn find_duplicates(nums: &[i32]) -> Vec<i32> {
    let mut counts = HashMap::new();
    for &n in nums {
        *counts.entry(n).or_insert(0) += 1;
    }
    let mut result: Vec<i32> = counts.into_iter().filter(|&(_, c)| c > 1).map(|(n, _)| n).collect();
    result.sort();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_frequency() {
        let f = char_frequency("aabbcc");
        assert_eq!(f[&'a'], 2);
        assert_eq!(f[&'b'], 2);
        assert_eq!(f[&'c'], 2);
    }

    #[test]
    fn test_most_frequent_char() {
        assert_eq!(most_frequent_char("aabbbcc"), 'b');
        assert_eq!(most_frequent_char("a"), 'a');
    }

    #[test]
    fn test_is_anagram() {
        assert!(is_anagram("listen", "silent"));
        assert!(is_anagram("anagram", "nagaram"));
        assert!(!is_anagram("rat", "car"));
        assert!(!is_anagram("ab", "a"));
    }

    #[test]
    fn test_group_by_first_char() {
        let words = ["apple", "avocado", "banana", "blueberry", "cherry"];
        let groups = group_by_first_char(&words);
        let mut a_group = groups[&'a'].clone();
        a_group.sort();
        assert_eq!(a_group, vec!["apple", "avocado"]);
        assert_eq!(groups[&'c'].len(), 1);
    }

    #[test]
    fn test_find_duplicates() {
        assert_eq!(find_duplicates(&[1, 2, 3, 2, 4, 3, 5]), vec![2, 3]);
        assert_eq!(find_duplicates(&[1, 2, 3]), vec![]);
    }
}
