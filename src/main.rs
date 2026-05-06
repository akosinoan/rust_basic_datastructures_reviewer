mod exercises;

#[cfg(feature = "solutions")]
mod solutions;

fn main() {
    println!("=== Phase 1: LeetCode Foundations ===");
    println!();
    println!("Run exercises with: cargo test");
    println!("Run a single module: cargo test vectors");
    println!("Run a single exercise: cargo test ex01");
    println!();
    println!("Modules:");
    println!("  vectors   - Vec<T>, slices, two pointers, prefix sums");
    println!("  hashmaps  - HashMap<K,V>, frequency counting, entry API");
    println!("  strings   - String, &str, chars(), manipulation");
    println!("  stacks    - Vec<T> as stack, stack patterns");
    println!("  queues    - VecDeque<T>, sliding window");
    println!("  leetcode  - Two Sum, Valid Parentheses, Best Time, Contains Duplicate");
}
