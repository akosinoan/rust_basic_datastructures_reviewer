# Rust DSA Reviewer

A self-directed, test-driven study system for mastering data structures and algorithms in Rust — modeled after [Rustlings](https://github.com/rust-lang/rustlings).

Each exercise is a function stub with a `todo!()` macro and inline teaching notes. Implement the function, save the file, and the runner updates automatically.

---

## Why This Exists

Most DSA resources teach concepts in isolation. This project connects fundamentals directly to the patterns that appear in technical interviews — with 100 exercises that build on each other, culminating in real LeetCode problems.

The exercises are written in idiomatic Rust: ownership, iterators, the entry API, and pattern matching are first-class tools, not afterthoughts.

---

## Getting Started

```bash
git clone <this-repo>
cd rust_basic_datastructures_reviewer
cargo run
```

That's it. The interactive runner starts, runs all tests, and shows your current progress. Open an exercise file, implement the `todo!()`, save — the screen clears and updates within a second.

---

## Interactive Runner

`cargo run` launches a live dashboard that:

- Shows **section-by-section progress** from `01_vectors` through `06_leetcode`
- Displays the **current exercise** — the first file in sequence that still has failures
- Lists every function in that file with `✓` (passing) or `✗` (failing) and the `todo!()` hint
- **Auto-reruns** on every file save with a 300ms debounce
- Clears the screen cleanly between runs

```
┌──────────────────────────────────────────────────────────────┐
│          Rust DSA Reviewer · Phase 1 Foundations            │
└──────────────────────────────────────────────────────────────┘

  Progress
  ──────────────────────────────────────────────────────────────
  01_vectors     ████░░░░░░░░░░░░░░░░  1 / 5
  02_hashmaps    ░░░░░░░░░░░░░░░░░░░░  0 / 4
  03_strings     ░░░░░░░░░░░░░░░░░░░░  0 / 4
  04_stacks      ░░░░░░░░░░░░░░░░░░░░  0 / 3
  05_queues      ░░░░░░░░░░░░░░░░░░░░  0 / 2
  06_leetcode    ░░░░░░░░░░░░░░░░░░░░  0 / 5

  Total: 1 / 23 files complete

  Current ──── vectors::ex02_iteration ───────────────────────

  src/exercises/01_vectors/ex02_iteration.rs

  1 / 5 passing

  ✓  sum()
  ✗  double_all()        Map each element to its double
  ✗  keep_evens()        Filter elements where x % 2 == 0
  ✗  first_negative_index()  Find position of first element < 0
  ✗  concat_all()        Concatenate all strings into one
```

Section bars color-code automatically: grey = untouched · yellow = in progress · green = complete.

---

## Manual Test Commands

If you prefer running tests directly:

```bash
# Full suite (100 tests, all failing at start)
cargo test

# By section
cargo test vectors
cargo test hashmaps
cargo test strings
cargo test stacks
cargo test queues
cargo test leetcode

# By file within a section
cargo test vectors::ex01_basics
cargo test hashmaps::ex03_entry_api

# By individual test
cargo test vectors::ex01_basics::tests::test_build_vec
cargo test test_two_sum
```

---

## Solutions

Every exercise has a reference implementation in `src/solutions/`, mirroring the same directory structure. It is hidden from the default test run — check it only after a genuine attempt.

```bash
# Run solution tests (opt-in, all should pass)
cargo test --features solutions solutions

# Run both exercises and solutions
cargo test --features solutions
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
├── main.rs                        # Interactive runner (cargo run)
├── exercises/                     # Your work — all todo!() stubs
│   ├── 01_vectors/
│   │   ├── ex01_basics.rs
│   │   ├── ex02_iteration.rs
│   │   ├── ex03_sorting.rs
│   │   ├── ex04_two_pointers.rs
│   │   └── ex05_prefix_sum.rs
│   ├── 02_hashmaps/
│   │   ├── ex01_basics.rs
│   │   ├── ex02_frequency.rs
│   │   ├── ex03_entry_api.rs
│   │   └── ex04_two_sum.rs
│   ├── 03_strings/
│   │   ├── ex01_basics.rs
│   │   ├── ex02_chars.rs
│   │   ├── ex03_manipulation.rs
│   │   └── ex04_palindrome.rs
│   ├── 04_stacks/
│   │   ├── ex01_basics.rs
│   │   ├── ex02_valid_parens.rs
│   │   └── ex03_monotonic.rs
│   ├── 05_queues/
│   │   ├── ex01_basics.rs
│   │   └── ex02_sliding_window.rs
│   └── 06_leetcode/
│       ├── lc001_two_sum.rs
│       ├── lc020_valid_parentheses.rs
│       ├── lc026_remove_duplicates.rs
│       ├── lc121_best_time_stocks.rs
│       └── lc217_contains_duplicate.rs
└── solutions/                     # Reference implementations (opt-in)
    └── (mirrors exercises/ structure)
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

The `todo!()` hint tells you the algorithm to reach for — not the answer. The runner shows this hint next to each failing function. The tests tell you when you're done.

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

## Stack

- **Language:** Rust (edition 2024)
- **Runner:** `cargo run` — interactive, file-watching dashboard
- **Testing:** `cargo test` — standard test suite
- **Dependencies:** `notify` (file watching), `crossterm` (terminal UI)
