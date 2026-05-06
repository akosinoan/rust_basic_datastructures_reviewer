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
        todo!("Return RunningStats with an empty Vec")
    }

    // Append a value to the dataset.
    pub fn push(&mut self, val: i32) {
        todo!("Push val onto self.data")
    }

    // Smallest value, or None if empty.
    pub fn min(&self) -> Option<i32> {
        todo!("self.data.iter().min() — copy the i32 out")
    }

    // Largest value, or None if empty.
    pub fn max(&self) -> Option<i32> {
        todo!("self.data.iter().max() — copy the i32 out")
    }

    // Arithmetic mean as f64, or None if empty.
    // HINT: sum as i64 first to avoid overflow, then divide by len as f64
    pub fn mean(&self) -> Option<f64> {
        todo!("sum / len — watch out for integer division")
    }

    // Middle value of the sorted dataset.
    // Even length  → average of the two middle elements.
    // Odd length   → the single middle element.
    // HINT: clone and sort self.data, then index into the middle
    pub fn median(&self) -> Option<f64> {
        todo!("sort a clone, return mid element (or avg of two mids)")
    }

    // Most frequently occurring value, or None if empty.
    // Ties are fine — return any of the tied values.
    // HINT: build a HashMap<i32,usize> frequency map, then max_by_key
    pub fn mode(&self) -> Option<i32> {
        todo!("frequency map → find key with highest count")
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
