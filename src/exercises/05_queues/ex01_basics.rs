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
// Simulate a simple task queue: enqueue N tasks (strings), then
// dequeue and return them all in FIFO order.
pub fn fifo_order(tasks: &[&str]) -> Vec<String> {
    todo!(
        "Push all tasks into a VecDeque, then pop_front into a result vec."
    )
}

// --- Exercise 1b ---
// Implement a circular buffer of fixed capacity using VecDeque.
// When the buffer is full and you enqueue, the OLDEST element is dropped.
pub struct CircularBuffer {
    data: VecDeque<i32>,
    capacity: usize,
}

impl CircularBuffer {
    pub fn new(capacity: usize) -> Self {
        todo!("Initialize with an empty VecDeque and store capacity")
    }

    pub fn enqueue(&mut self, val: i32) {
        todo!(
            "If data.len() == capacity, pop_front first. Then push_back val."
        )
    }

    pub fn to_vec(&self) -> Vec<i32> {
        self.data.iter().copied().collect()
    }
}

// --- Exercise 1c ---
// Given a sequence of moves "LR", use a deque to simulate:
// 'L' → move head element to tail
// 'R' → move tail element to head
// Return final state.
pub fn rotate(items: Vec<i32>, moves: &str) -> Vec<i32> {
    todo!(
        "Put items into a VecDeque. For each move char:\
         'L' → pop_front, push_back. 'R' → pop_back, push_front.\
         Collect at end."
    )
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
