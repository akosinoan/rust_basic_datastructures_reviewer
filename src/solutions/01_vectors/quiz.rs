// QUIZ — Section 1: Running Statistics
//
// Build a RunningStats struct that collects integers and answers statistical
// queries. You will use Vec operations, iterator chains (map, filter, sum),
// sorting, and a frequency map — everything from this section.
//
// Fields are provided. Implement every method. All return Option because an
// empty dataset has no meaningful answer.

pub struct RunningStats {
    data: Vec<i32>,
}

impl RunningStats {
    // Create an empty RunningStats.
    pub fn new() -> Self {
        RunningStats { data: Vec::new() }
    }

    // Append a value to the dataset.
    pub fn push(&mut self, val: i32) {
        self.data.push(val);
    }

    // Smallest value, or None if empty.
    pub fn min(&self) -> Option<i32> {
        self.data.iter().min().copied()
    }

    // Largest value, or None if empty.
    pub fn max(&self) -> Option<i32> {
        self.data.iter().max().copied()
    }

    // Arithmetic mean as f64, or None if empty.
    // HINT: sum as i64 first to avoid overflow, then divide by len as f64
    pub fn mean(&self) -> Option<f64> {
        if self.data.is_empty() {
            return None;
        }
        let sum: i64 = self.data.iter().map(|&x| x as i64).sum();
        Some(sum as f64 / self.data.len() as f64)
    }

    // Middle value of the sorted dataset.
    // Even length  → average of the two middle elements.
    // Odd length   → the single middle element.
    // HINT: clone and sort self.data, then index into the middle
    pub fn median(&self) -> Option<f64> {
        if self.data.is_empty() {
            return None;
        }
        let mut sorted = self.data.clone();
        sorted.sort();
        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            Some((sorted[mid - 1] as f64 + sorted[mid] as f64) / 2.0)
        } else {
            Some(sorted[mid] as f64)
        }
    }

    // Most frequently occurring value, or None if empty.
    // Ties are fine — return any of the tied values.
    // HINT: build a HashMap<i32,usize> frequency map, then max_by_key
    pub fn mode(&self) -> Option<i32> {
        if self.data.is_empty() {
            return None;
        }
        let mut freq = std::collections::HashMap::new();
        for &x in &self.data {
            *freq.entry(x).or_insert(0usize) += 1;
        }
        freq.into_iter().max_by_key(|(_, count)| *count).map(|(val, _)| val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let s = RunningStats::new();
        assert_eq!(s.min(), None);
        assert_eq!(s.max(), None);
        assert_eq!(s.mean(), None);
        assert_eq!(s.median(), None);
        assert_eq!(s.mode(), None);
    }

    #[test]
    fn test_single() {
        let mut s = RunningStats::new();
        s.push(42);
        assert_eq!(s.min(), Some(42));
        assert_eq!(s.max(), Some(42));
        assert_eq!(s.mean(), Some(42.0));
        assert_eq!(s.median(), Some(42.0));
        assert_eq!(s.mode(), Some(42));
    }

    #[test]
    fn test_min_max() {
        let mut s = RunningStats::new();
        for v in [3, 1, 4, 1, 5, 9, 2, 6] {
            s.push(v);
        }
        assert_eq!(s.min(), Some(1));
        assert_eq!(s.max(), Some(9));
    }

    #[test]
    fn test_mean() {
        let mut s = RunningStats::new();
        for v in [1, 2, 3, 4, 5] {
            s.push(v);
        }
        assert_eq!(s.mean(), Some(3.0));
    }

    #[test]
    fn test_mean_non_integer() {
        let mut s = RunningStats::new();
        s.push(1);
        s.push(2);
        assert_eq!(s.mean(), Some(1.5));
    }

    #[test]
    fn test_median_odd() {
        let mut s = RunningStats::new();
        for v in [5, 1, 3] {
            s.push(v);
        }
        assert_eq!(s.median(), Some(3.0));
    }

    #[test]
    fn test_median_even() {
        let mut s = RunningStats::new();
        for v in [1, 2, 3, 4] {
            s.push(v);
        }
        assert_eq!(s.median(), Some(2.5));
    }

    #[test]
    fn test_mode() {
        let mut s = RunningStats::new();
        for v in [1, 2, 2, 3, 3, 3] {
            s.push(v);
        }
        assert_eq!(s.mode(), Some(3));
    }

    #[test]
    fn test_negatives() {
        let mut s = RunningStats::new();
        for v in [-5, -1, -3] {
            s.push(v);
        }
        assert_eq!(s.min(), Some(-5));
        assert_eq!(s.max(), Some(-1));
        assert_eq!(s.median(), Some(-3.0));
    }
}
