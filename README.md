# Rust DSA Reviewer

A self-directed, test-driven study system for mastering data structures and algorithms in Rust — modeled after [Rustlings](https://github.com/rust-lang/rustlings).

Each exercise is a function stub with a `todo!()` macro and inline teaching notes. You implement the function, run `cargo test`, and iterate until all tests pass.

---

## Why This Exists

Most DSA resources teach concepts in isolation. This project connects fundamentals directly to the patterns that appear in technical interviews — with 100 exercises that build on each other, culminating in real LeetCode problems.

The exercises are written in idiomatic Rust: ownership, iterators, the entry API, and pattern matching are first-class tools, not afterthoughts.

---

## Getting Started

```bash
git clone <this-repo>
cd rust_basic_datastructures_reviewer

# See all 100 tests failing (that's the starting point)
cargo test

# Focus on one module at a time
cargo test vectors
cargo test hashmaps
cargo test strings
cargo test stacks
cargo test queues
cargo test leetcode

# Focus on one file within a module
cargo test vectors::ex01_basics
cargo test hashmaps::ex03_entry_api
cargo test stacks::ex02_valid_parens

# Run a single test function
cargo test vectors::ex01_basics::tests::test_build_vec
cargo test test_two_sum
```

Open the exercise file, read the header comment, implement the `todo!()`, and run the tests. Repeat.

If you're stuck, the `src/solutions/` folder contains a complete reference implementation for every exercise — mirroring the same structure. Check it only after you've made a real attempt.

```bash
# Run only solution tests (all should pass — use as a reference)
cargo test solutions

# Run only exercise tests (your work — starts at 100 failing)
cargo test exercises
```

---

## Curriculum

### Module 1 — Vectors & Slices (`src/exercises/01_vectors/`)

The foundation. Every other module builds on slice manipulation.

| File | Topic | Key Concepts |
|---|---|---|
| `ex01_basics.rs` | Vec operations | `push`, `pop`, `get`, `clear` |
| `ex02_iteration.rs` | Iterator chains | `map`, `filter`, `enumerate`, `fold` |
| `ex03_sorting.rs` | Sorting & searching | `sort_by`, `binary_search`, `windows`, `dedup` |
| `ex04_two_pointers.rs` | Two-pointer pattern | in-place reversal, pair finding, duplicate removal |
| `ex05_prefix_sum.rs` | Prefix sums | range queries, subarray sum, product except self |

### Module 2 — HashMaps (`src/exercises/02_hashmaps/`)

O(1) lookups unlock the jump from brute force to optimal.

| File | Topic | Key Concepts |
|---|---|---|
| `ex01_basics.rs` | HashMap operations | `insert`, `get`, `remove`, `keys`, merge |
| `ex02_frequency.rs` | Frequency counting | character counts, anagram detection, grouping |
| `ex03_entry_api.rs` | Entry API | `or_insert`, `and_modify`, conditional insert |
| `ex04_two_sum.rs` | Two Sum family | complement lookup, all pairs, Three Sum |

### Module 3 — Strings (`src/exercises/03_strings/`)

Rust's strict UTF-8 model forces correct string handling from day one.

| File | Topic | Key Concepts |
|---|---|---|
| `ex01_basics.rs` | `String` vs `&str` | ownership, borrowing, `contains`, `starts_with` |
| `ex02_chars.rs` | Char iteration | `chars()`, Caesar cipher, Unicode safety |
| `ex03_manipulation.rs` | Building strings | `split`, `join`, snake_to_camel, run-length encoding |
| `ex04_palindrome.rs` | Palindrome problems | exact, LC#125, LC#680 (one deletion) |

### Module 4 — Stacks (`src/exercises/04_stacks/`)

`Vec<T>` as a stack. Pattern recognition for nesting and monotonicity.

| File | Topic | Key Concepts |
|---|---|---|
| `ex01_basics.rs` | Stack fundamentals | RPN calculator, MinStack implementation |
| `ex02_valid_parens.rs` | Bracket matching | LC#20, LC#921, LC#678 |
| `ex03_monotonic.rs` | Monotonic stack | next greater element, daily temperatures, histogram |

### Module 5 — Queues (`src/exercises/05_queues/`)

`VecDeque<T>` for FIFO, circular buffers, and O(1) window queries.

| File | Topic | Key Concepts |
|---|---|---|
| `ex01_basics.rs` | VecDeque basics | circular buffer, rotation simulation |
| `ex02_sliding_window.rs` | Sliding window | fixed/variable window, LC#3, LC#239 |

### Module 6 — LeetCode Problems (`src/exercises/06_leetcode/`)

Applying everything above to real interview problems — each with multiple approaches to compare trade-offs.

| File | Problem | Difficulty | Core Pattern |
|---|---|---|---|
| `lc001_two_sum.rs` | Two Sum | Easy | HashMap complement lookup |
| `lc020_valid_parentheses.rs` | Valid Parentheses | Easy | Stack bracket matching |
| `lc026_remove_duplicates.rs` | Remove Duplicates from Sorted Array | Easy | Two pointers |
| `lc121_best_time_stocks.rs` | Best Time to Buy and Sell Stock | Easy | One-pass min tracking |
| `lc217_contains_duplicate.rs` | Contains Duplicate | Easy | Brute / Sort / HashSet comparison |

---

## Project Structure

```
src/
├── main.rs                        # Usage instructions
└── exercises/
    ├── 01_vectors/
    │   ├── ex01_basics.rs
    │   ├── ex02_iteration.rs
    │   ├── ex03_sorting.rs
    │   ├── ex04_two_pointers.rs
    │   └── ex05_prefix_sum.rs
    ├── 02_hashmaps/
    │   ├── ex01_basics.rs
    │   ├── ex02_frequency.rs
    │   ├── ex03_entry_api.rs
    │   └── ex04_two_sum.rs
    ├── 03_strings/
    │   ├── ex01_basics.rs
    │   ├── ex02_chars.rs
    │   ├── ex03_manipulation.rs
    │   └── ex04_palindrome.rs
    ├── 04_stacks/
    │   ├── ex01_basics.rs
    │   ├── ex02_valid_parens.rs
    │   └── ex03_monotonic.rs
    ├── 05_queues/
    │   ├── ex01_basics.rs
    │   └── ex02_sliding_window.rs
    └── 06_leetcode/
        ├── lc001_two_sum.rs
        ├── lc020_valid_parentheses.rs
        ├── lc026_remove_duplicates.rs
        ├── lc121_best_time_stocks.rs
        └── lc217_contains_duplicate.rs
```

---

## Exercise Format

Every file follows the same structure:

```rust
// TOPIC — Exercise N: Title
//
// Concept explanation with the key Rust APIs listed.
// Explains WHY this pattern matters for interviews.

// --- Exercise Na ---
// What you need to implement.
pub fn solve(input: &[i32]) -> i32 {
    todo!("Hint: the specific approach to use")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        assert_eq!(solve(&[1, 2, 3]), 6);
    }
}
```

The `todo!()` hint tells you the algorithm to reach for — not the answer. The tests tell you when you're done.

---

## Patterns Covered

| Pattern | Where Introduced | LeetCode Problems |
|---|---|---|
| Two Pointers | `01_vectors/ex04` | LC#26, LC#167, LC#15 |
| Prefix Sum | `01_vectors/ex05` | LC#560, LC#238 |
| HashMap Complement | `02_hashmaps/ex04` | LC#1, LC#15 |
| Frequency Count | `02_hashmaps/ex02` | LC#242, LC#49 |
| Sliding Window (fixed) | `05_queues/ex02` | LC#239, LC#643 |
| Sliding Window (variable) | `05_queues/ex02` | LC#3, LC#1004 |
| Stack (bracket matching) | `04_stacks/ex02` | LC#20, LC#921, LC#678 |
| Monotonic Stack | `04_stacks/ex03` | LC#739, LC#84 |

---

## Recommended Order

Work through one module at a time before moving to the next:

```
vectors → hashmaps → strings → stacks → queues → leetcode
```

Each module's later exercises assume you've internalized the earlier ones. The LeetCode module is the checkpoint — if those feel natural, the foundations are solid.

---

## Stack

- **Language:** Rust (edition 2024)
- **Testing:** `cargo test` (built-in, no external dependencies)
- **Dependencies:** none
