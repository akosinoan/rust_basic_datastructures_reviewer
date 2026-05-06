use std::collections::HashMap;

pub fn increment(map: &mut HashMap<String, i32>, key: &str) {
    map.entry(key.to_string()).and_modify(|v| *v += 1).or_insert(1);
}

pub fn sum_by_parity(nums: &[i32]) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for &n in nums {
        let key = if n % 2 == 0 { "even" } else { "odd" };
        *map.entry(key.to_string()).or_insert(0) += n;
    }
    map
}

pub fn highest_score(records: &[(&str, i32)]) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for &(student, score) in records {
        map.entry(student.to_string())
            .and_modify(|old| {
                if score > *old {
                    *old = score;
                }
            })
            .or_insert(score);
    }
    map
}

pub fn first_repeated_word(sentence: &str) -> Option<String> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for word in sentence.split_whitespace() {
        let count = counts.entry(word.to_string()).or_insert(0);
        *count += 1;
        if *count == 2 {
            return Some(word.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let mut m: HashMap<String, i32> = HashMap::new();
        increment(&mut m, "a");
        assert_eq!(m["a"], 1);
        increment(&mut m, "a");
        assert_eq!(m["a"], 2);
        increment(&mut m, "b");
        assert_eq!(m["b"], 1);
    }

    #[test]
    fn test_sum_by_parity() {
        let m = sum_by_parity(&[1, 2, 3, 4, 5]);
        assert_eq!(m["even"], 6);
        assert_eq!(m["odd"], 9);
    }

    #[test]
    fn test_highest_score() {
        let records = [("alice", 90), ("bob", 85), ("alice", 95), ("bob", 80)];
        let m = highest_score(&records);
        assert_eq!(m["alice"], 95);
        assert_eq!(m["bob"], 85);
    }

    #[test]
    fn test_first_repeated_word() {
        assert_eq!(
            first_repeated_word("the cat sat on the mat"),
            Some("the".to_string())
        );
        assert_eq!(first_repeated_word("one two three"), None);
        assert_eq!(
            first_repeated_word("hello hello world"),
            Some("hello".to_string())
        );
    }
}
