// VECTORS - Exercise 2: Iteration
//
// Rust iterators are lazy and composable. Key methods:
//   .iter()             — iterate by reference (&T)
//   .into_iter()        — iterate by value (consumes the vec)
//   .iter_mut()         — iterate by mutable reference (&mut T)
//   .enumerate()        — yields (index, &value) pairs
//   .map(|x| ...)       — transform each element
//   .filter(|x| ...)    — keep elements matching predicate
//   .sum::<i32>()       — sum all elements
//   .collect::<Vec<_>>()— turn iterator back into a collection
//   .any(|x| ...)       — true if any element matches
//   .all(|x| ...)       — true if all elements match
//   .position(|x| ...) — index of first match (Option<usize>)

// --- Exercise 2a ---
// Return the sum of all elements.
// HINT: .iter().sum::<i32>()
pub fn sum(v: &[i32]) -> i32 {
    todo!("Sum all elements using an iterator")
}

// --- Exercise 2b ---
// Return a new vec with every element doubled.
// HINT: .iter().map(|&x| x * 2).collect()
pub fn double_all(v: &[i32]) -> Vec<i32> {
    todo!("Map each element to its double")
}

// --- Exercise 2c ---
// Return only the even numbers from the slice.
pub fn keep_evens(v: &[i32]) -> Vec<i32> {
    todo!("Filter elements where x % 2 == 0")
}

// --- Exercise 2d ---
// Return the index of the first negative number, or None.
// HINT: .iter().position(|&x| ...)
pub fn first_negative_index(v: &[i32]) -> Option<usize> {
    todo!("Find position of first element < 0")
}

// --- Exercise 2e ---
// Given a vec of strings, concatenate them all into one String.
// HINT: .iter().map(|s| s.as_str()).collect::<Vec<_>>().join("")
//       OR fold/for loop
pub fn concat_all(v: &[String]) -> String {
    todo!("Concatenate all strings into one")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum(&[]), 0);
        assert_eq!(sum(&[-1, 1]), 0);
    }

    #[test]
    fn test_double_all() {
        assert_eq!(double_all(&[1, 2, 3]), vec![2, 4, 6]);
        assert_eq!(double_all(&[]), vec![]);
    }

    #[test]
    fn test_keep_evens() {
        assert_eq!(keep_evens(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
        assert_eq!(keep_evens(&[1, 3, 5]), vec![]);
    }

    #[test]
    fn test_first_negative_index() {
        assert_eq!(first_negative_index(&[1, 2, -3, 4]), Some(2));
        assert_eq!(first_negative_index(&[1, 2, 3]), None);
        assert_eq!(first_negative_index(&[-1]), Some(0));
    }

    #[test]
    fn test_concat_all() {
        let v = vec!["foo".to_string(), "bar".to_string(), "baz".to_string()];
        assert_eq!(concat_all(&v), "foobarbaz");
        assert_eq!(concat_all(&[]), "");
    }
}
