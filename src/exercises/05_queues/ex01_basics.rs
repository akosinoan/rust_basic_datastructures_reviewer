// QUEUES - Exercise 1: VecDeque
//
// VecDeque<T> is a double-ended queue. Use it when you need
// to push/pop from BOTH ends efficiently.
//
//   use std::collections::VecDeque;
//   let mut q: VecDeque<i32> = VecDeque::new();
//   q.push_back(x)      — enqueue (add to back)
//   q.pop_front()       — dequeue (remove from front) → Option<T>
//   q.push_front(x)     — add to front
//   q.pop_back()        — remove from back
//   q.front() / .back() — peek without removing
//   q.len() / .is_empty()
//
// Queue property: FIFO — First In, First Out.
// Use for BFS, scheduling, sliding window problems.

use std::collections::VecDeque;

// --- Exercise 1a ---
// Simulate a FIFO task queue: enqueue every input task, then dequeue all
// of them and return them in the order they came out.
//
// Inputs:  tasks — borrowed slice of &str.
// Returns: Vec<String> with each task converted to an owned String,
//          appearing in the SAME order as in `tasks` (FIFO).
//
// Edge cases the tests check:
//   - ["a","b","c","d"] → ["a","b","c","d"]
pub fn fifo_order(tasks: &[&str]) -> Vec<String> {
    todo!()
}

// --- Exercise 1b ---
// Fixed-capacity circular buffer. When `enqueue` is called and the buffer is
// already full, the OLDEST element is evicted (dropped) before the new one is added.
//
// `to_vec` is provided to make assertions easy — you do not implement it.
pub struct CircularBuffer {
    data: VecDeque<i32>,
    capacity: usize,
}

impl CircularBuffer {
    // Construct an empty buffer with maximum capacity `capacity`.
    //
    // Inputs:  capacity — usize, the most elements the buffer ever holds.
    // Returns: CircularBuffer with empty internal VecDeque and stored capacity.
    pub fn new(capacity: usize) -> Self {
        todo!()
    }

    // Insert `val` at the back.
    //
    // If the buffer is already full (len == capacity), drop the oldest element
    // (front) BEFORE inserting `val` at the back.
    //
    // Edge cases the tests check:
    //   - capacity 3, push 1,2,3 → [1,2,3]
    //   - then push 4            → drops 1, becomes [2,3,4]
    //   - then push 5,6          → ends up [4,5,6]
    pub fn enqueue(&mut self, val: i32) {
        todo!()
    }

    // (Provided.) Snapshot the current contents into a Vec<i32>.
    pub fn to_vec(&self) -> Vec<i32> {
        self.data.iter().copied().collect()
    }
}

// --- Exercise 1c ---
// Apply a sequence of "rotation" moves to a list. 'L' moves the head element
// to the tail; 'R' moves the tail element to the head. Return the final order.
//
// Inputs:  items — owned Vec<i32> seeded into a deque;
//          moves — borrowed &str of 'L'/'R' characters.
// Returns: Vec<i32> after all moves are applied, in left-to-right order.
//
// Examples:
//   rotate(vec![1,2,3,4], "L")  → [2,3,4,1]   (head 1 goes to tail)
//   rotate(vec![1,2,3,4], "R")  → [4,1,2,3]   (tail 4 goes to head)
//   rotate(vec![1,2,3], "LL")   → [3,1,2]
pub fn rotate(items: Vec<i32>, moves: &str) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fifo_order() {
        let result = fifo_order(&["a", "b", "c", "d"]);
        assert_eq!(result, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn test_circular_buffer() {
        let mut buf = CircularBuffer::new(3);
        buf.enqueue(1);
        buf.enqueue(2);
        buf.enqueue(3);
        assert_eq!(buf.to_vec(), vec![1, 2, 3]);
        buf.enqueue(4); // drops 1
        assert_eq!(buf.to_vec(), vec![2, 3, 4]);
        buf.enqueue(5); // drops 2
        buf.enqueue(6); // drops 3
        assert_eq!(buf.to_vec(), vec![4, 5, 6]);
    }

    #[test]
    fn test_rotate() {
        // [1,2,3,4], L → 2 goes front, tail becomes 1 → [2,3,4,1]
        assert_eq!(rotate(vec![1, 2, 3, 4], "L"), vec![2, 3, 4, 1]);
        // [1,2,3,4], R → 4 goes front → [4,1,2,3]
        assert_eq!(rotate(vec![1, 2, 3, 4], "R"), vec![4, 1, 2, 3]);
        // [1,2,3], LL → [3,1,2]
        assert_eq!(rotate(vec![1, 2, 3], "LL"), vec![3, 1, 2]);
    }
}
