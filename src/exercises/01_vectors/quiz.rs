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
    // Construct a fresh RunningStats with an empty dataset.
    //
    // Returns: RunningStats whose internal Vec is empty (len == 0).
    pub fn new() -> Self {
        todo!()
    }

    // Append `val` to the dataset.
    //
    // Inputs:  val — any i32 (positive, zero, or negative).
    // Returns: nothing. After the call, the new value is the last element.
    pub fn push(&mut self, val: i32) {
        todo!()
    }

    // Smallest value in the dataset.
    //
    // Returns: Some(min) if the dataset has at least one element, else None.
    //
    // Edge cases the tests check:
    //   - empty             → None
    //   - single value      → Some(that value)
    //   - many values       → the minimum (test hits both positive and negative inputs)
    pub fn min(&self) -> Option<i32> {
        todo!()
    }

    // Largest value in the dataset.
    //
    // Returns: Some(max) if the dataset has at least one element, else None.
    pub fn max(&self) -> Option<i32> {
        todo!()
    }

    // Arithmetic mean as f64.
    //
    // Returns: Some(sum / len) if len > 0, else None.
    //          Watch out for integer division; cast carefully so 1.5 stays 1.5
    //          and so a sum of large i32 values does not overflow i32.
    //
    // Edge cases the tests check:
    //   - empty             → None
    //   - whole-number mean → e.g. [1,2,3,4,5] → Some(3.0)
    //   - non-integer mean  → e.g. [1,2]       → Some(1.5)
    pub fn mean(&self) -> Option<f64> {
        todo!()
    }

    // Median of the dataset, as f64.
    //
    // Returns: Some(median) when len > 0, else None.
    //          Odd length  → the single middle element after sorting.
    //          Even length → average of the two middle elements after sorting.
    //          Do NOT mutate self.data — operate on a clone.
    //
    // Edge cases the tests check:
    //   - odd length     → e.g. [5,1,3] → Some(3.0)
    //   - even length    → e.g. [1,2,3,4] → Some(2.5)
    //   - negatives      → still works, e.g. [-5,-1,-3] sorted is [-5,-3,-1] → Some(-3.0)
    pub fn median(&self) -> Option<f64> {
        todo!()
    }

    // Most frequently occurring value (the "mode").
    //
    // Returns: Some(value) if len > 0, else None.
    //          If multiple values are tied for the highest count, returning
    //          any of them is acceptable.
    //
    // Edge cases the tests check:
    //   - [1,2,2,3,3,3] → Some(3)
    //   - single value  → that value
    pub fn mode(&self) -> Option<i32> {
        todo!()
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
