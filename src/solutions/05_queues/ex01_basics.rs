use std::collections::VecDeque;

pub fn fifo_order(tasks: &[&str]) -> Vec<String> {
    let q: VecDeque<String> = tasks.iter().map(|s| s.to_string()).collect();
    q.into_iter().collect()
}

pub struct CircularBuffer {
    data: VecDeque<i32>,
    capacity: usize,
}

impl CircularBuffer {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            data: VecDeque::new(),
            capacity,
        }
    }

    pub fn enqueue(&mut self, val: i32) {
        if self.data.len() == self.capacity {
            self.data.pop_front();
        }
        self.data.push_back(val);
    }

    pub fn to_vec(&self) -> Vec<i32> {
        self.data.iter().copied().collect()
    }
}

pub fn rotate(items: Vec<i32>, moves: &str) -> Vec<i32> {
    let mut q: VecDeque<i32> = items.into_iter().collect();
    for m in moves.chars() {
        match m {
            'L' => {
                if let Some(front) = q.pop_front() {
                    q.push_back(front);
                }
            }
            'R' => {
                if let Some(back) = q.pop_back() {
                    q.push_front(back);
                }
            }
            _ => {}
        }
    }
    q.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fifo_order() {
        assert_eq!(fifo_order(&["a", "b", "c", "d"]), vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn test_circular_buffer() {
        let mut buf = CircularBuffer::new(3);
        buf.enqueue(1);
        buf.enqueue(2);
        buf.enqueue(3);
        assert_eq!(buf.to_vec(), vec![1, 2, 3]);
        buf.enqueue(4);
        assert_eq!(buf.to_vec(), vec![2, 3, 4]);
        buf.enqueue(5);
        buf.enqueue(6);
        assert_eq!(buf.to_vec(), vec![4, 5, 6]);
    }

    #[test]
    fn test_rotate() {
        assert_eq!(rotate(vec![1, 2, 3, 4], "L"), vec![2, 3, 4, 1]);
        assert_eq!(rotate(vec![1, 2, 3, 4], "R"), vec![4, 1, 2, 3]);
        assert_eq!(rotate(vec![1, 2, 3], "LL"), vec![3, 1, 2]);
    }
}
