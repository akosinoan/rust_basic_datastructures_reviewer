# Rust DSA Reviewer

> A self-built, Rustlings-style interactive learning system for data structures and algorithms — **28 exercise files, 117 function stubs, 148 tests**, all driven by a custom terminal UI that re-runs the suite on every save.

Built in Rust 2024 with `notify` and `crossterm`. No external test framework, no LeetCode plugins — just a small, well-factored engine that watches the filesystem, parses `cargo test` output, and renders a live dashboard with per-function pass/fail status and contextual hints.

---

## At a Glance

| | |
|---|---|
| **Language** | Rust (edition 2024) |
| **Curriculum** | 6 modules · 28 files · 117 `todo!()` stubs to implement |
| **Test surface** | 148 unit tests covering every function, edge cases, and Unicode-aware string handling |
| **Runner engine** | ~1,600 LOC across 9 modules — file watcher, output parser, TUI renderer, hint engine |
| **Solutions** | Mirror layout, hidden behind a Cargo feature flag — opt-in, never auto-revealed |
| **Dependencies** | Two: `notify` (file events), `crossterm` (terminal UI) |

---

## What This Project Demonstrates

This isn't just "DSA exercises I worked through" — it's a complete tooling system built from scratch. The repository shows:

- **Systems-level Rust** — channels, threads, `Result`-based error flow, lifetimes, the entry API, iterator adapters, and ownership-aware borrowing throughout the runner.
- **Architecture under constraint** — a clean separation between curriculum data (`catalog`, `hints`), input (`runner`, `parser`), state (`results`), and presentation (`render`, `theme`), all coordinated by a single event loop in `watcher.rs`.
- **Robust output parsing** — the runner shells out to `cargo test`, classifies stdout/stderr into either a compile-error path or a structured test result, then scrapes `panicked at ...` lines to surface `todo!()` hints next to each failing function.
- **Responsive TUI design** — adapts to terminal width: switches between a stacked layout and a side-by-side progress + current-exercise panel above 114 columns.
- **Self-validating curriculum** — `cargo test` includes a meta-test that ensures every catalogued exercise has at least one hint and that no orphan hints exist. The data model can't drift from the curriculum.
- **Pedagogical depth** — every exercise file teaches a concept with inline notes, lists the relevant Rust APIs, calls out edge cases, and ends with a capstone `quiz.rs` that combines the section's primitives into a single struct or function.

---

## Live Runner

`cargo run` launches a TUI that re-renders on every file save (300ms debounce). No reload, no manual test commands.

```
┌──────────────────────────────────────────────────────────────┐
│          Rust DSA Reviewer · Phase 1 Foundations             │
└──────────────────────────────────────────────────────────────┘

  Progress                              Current ──── vectors::ex02_iteration ───────
  ─────────────────────────────────────
  01_vectors     ████░░░░░░░░░░░░░░░░  1 / 6     src/exercises/01_vectors/ex02_iteration.rs
  02_hashmaps    ░░░░░░░░░░░░░░░░░░░░  0 / 5
  03_strings     ░░░░░░░░░░░░░░░░░░░░  0 / 5     1 / 5 passing
  04_stacks      ░░░░░░░░░░░░░░░░░░░░  0 / 4
  05_queues      ░░░░░░░░░░░░░░░░░░░░  0 / 3     ✓  sum()
  06_leetcode    ░░░░░░░░░░░░░░░░░░░░  0 / 5     ✗  double_all()         Map each element to its double
                                                  ✗  keep_evens()         Filter where x % 2 == 0
  Total:  1 / 28 files complete                   ✗  first_negative_index() Position of first x < 0
                                                  ✗  concat_all()         Join all strings into one

  save to re-run · type 'hint' for help · Ctrl+C to quit
```

Section bars color-shift by status: **grey** untouched · **yellow** in progress · **green** complete. A `✗ Compilation Error` panel renders inline when the code doesn't build, so you never have to switch terminals to read `cargo`.

### Type `hint` for guided help

Pressing `hint` <kbd>Enter</kbd> at the runner prompt prints a one-line tactical hint for the first failing function in the current file — pulled from a static `&[FileHints]` keyed by `module::file`, validated against the catalog at compile time.

---

## Architecture

The reviewer engine lives entirely in `src/reviewer/` and is organised as a one-way data pipeline:

```
              ┌──────────┐
              │  notify  │  filesystem events on src/exercises/
              └────┬─────┘
                   │
              ┌────▼─────┐
              │ watcher  │  debounces, dispatches Watch/Stdin events
              └────┬─────┘
                   │
       ┌───────────┴──────────────┐
       │                          │
  ┌────▼─────┐               ┌────▼─────┐
  │  runner  │ cargo test    │  stdin   │ user types 'hint'
  └────┬─────┘ output        └────┬─────┘
       │                          │
  ┌────▼─────┐                    │
  │  parser  │ extracts test      │
  │          │ paths, status,     │
  │          │ panic hints        │
  └────┬─────┘                    │
       │                          │
  ┌────▼──────────────────────────▼────┐
  │              results              │  HashMap<module, HashMap<file, HashMap<test, TestResult>>>
  └────────────────┬──────────────────┘
                   │
              ┌────▼─────┐
              │  render  │  crossterm-styled TUI, responsive columns
              └──────────┘
```

| Module | Role | Lines |
|---|---|---|
| `watcher.rs` | Event loop · file-watch + stdin channels · debouncing | 117 |
| `runner.rs` | Invokes `cargo test`, separates compile-error vs. test output | 49 |
| `parser.rs` | Pure-function parser of cargo output into structured results | 88 |
| `results.rs` | Result types · `ExerciseStatus` state machine · "what's next" lookup | 56 |
| `catalog.rs` | Curriculum data — `&'static [Section]` of modules & files | 73 |
| `hints.rs` | Hint database + meta-test that validates against the catalog | 735 |
| `render.rs` | `crossterm` rendering · responsive two-column layout | 444 |
| `theme.rs` | Colors, dimensions, ASCII box characters | 29 |

Each module has a single responsibility and a narrow public surface. The parser is a pure function over a `&str` — easy to test, easy to reason about. The renderer holds no state except an `io::Stdout` handle.

---

## Getting Started

```bash
git clone <this-repo>
cd rust_basic_datastructures_reviewer
cargo run
```

That's it. The runner starts, runs all tests, and shows current progress. Open any file under `src/exercises/`, replace the `todo!()` body, save — the screen refreshes within ~300 ms.

### Manual test commands

```bash
cargo test                          # full suite
cargo test vectors                  # by module
cargo test vectors::ex01_basics     # by file
cargo test test_two_sum             # by test name
```

### Reference solutions (opt-in)

```bash
cargo test --features solutions solutions   # run solution tests only
cargo test --features solutions             # run both
```

Solutions live in `src/solutions/` mirroring the exercises. They're gated by the `solutions` Cargo feature so you can't accidentally read them while working.

---

## Curriculum

Six modules, each ending in a capstone `quiz.rs` that combines the section's primitives into a single design problem.

### Module 1 — Vectors & Slices · `01_vectors/`

The foundation. Every other module builds on slice manipulation.

| File | Topic | Key Concepts |
|---|---|---|
| `ex01_basics.rs` | `Vec<T>` operations | `push`, `pop`, `get`, `clear`, `Option<&T>` handling |
| `ex02_iteration.rs` | Iterator chains | `map`, `filter`, `position`, `fold`, turbofish |
| `ex03_sorting.rs` | Sorting & searching | `sort_by`, `sort_by_key`, `binary_search`, `windows`, `dedup` |
| `ex04_two_pointers.rs` | Two-pointer pattern | in-place reversal, pair finding, duplicate removal |
| `ex05_prefix_sum.rs` | Prefix sums | range queries, subarray sum, product except self |
| `quiz.rs` | `RunningStats` struct | min · max · mean · median · mode in one type |

### Module 2 — HashMaps · `02_hashmaps/`

O(1) lookups unlock the jump from brute force to optimal.

| File | Topic | Key Concepts |
|---|---|---|
| `ex01_basics.rs` | `HashMap` operations | `insert`, `get`, `remove`, `keys`, merge |
| `ex02_frequency.rs` | Frequency counting | character counts, anagram detection, grouping |
| `ex03_entry_api.rs` | Entry API | `or_insert`, `and_modify`, conditional insert |
| `ex04_two_sum.rs` | Two-Sum family | complement lookup, all pairs, Three-Sum (sort + two-pointer) |
| `quiz.rs` | `WordCounter` struct | case-insensitive counting, intersection, top-k queries |

### Module 3 — Strings · `03_strings/`

Rust's strict UTF-8 model forces correct string handling from day one.

| File | Topic | Key Concepts |
|---|---|---|
| `ex01_basics.rs` | `String` vs `&str` | ownership, borrowing, `contains`, identifier validation |
| `ex02_chars.rs` | Char iteration | `chars()`, Caesar cipher, Unicode-safe reverse |
| `ex03_manipulation.rs` | Building strings | `split`, `join`, snake_to_camel, run-length encoding |
| `ex04_palindrome.rs` | Palindrome problems | exact, LC#125, LC#680 (one deletion), all substrings |
| `quiz.rs` | `TextAnalyzer` struct | pangram detection, longest-word, char-frequency in one API |

### Module 4 — Stacks · `04_stacks/`

`Vec<T>` as a stack. Pattern recognition for nesting and monotonicity.

| File | Topic | Key Concepts |
|---|---|---|
| `ex01_basics.rs` | Stack fundamentals | RPN calculator, `MinStack` with O(1) min |
| `ex02_valid_parens.rs` | Bracket matching | LC#20, LC#921, LC#678 (with wildcards) |
| `ex03_monotonic.rs` | Monotonic stack | next greater element, daily temperatures, histogram |
| `quiz.rs` | Expression evaluator | RPN + bracket-balancing + next-greater in one file |

### Module 5 — Queues · `05_queues/`

`VecDeque<T>` for FIFO, circular buffers, and O(1) window queries.

| File | Topic | Key Concepts |
|---|---|---|
| `ex01_basics.rs` | `VecDeque` basics | circular buffer, rotation simulation |
| `ex02_sliding_window.rs` | Sliding window | fixed/variable window, LC#3, LC#239 |
| `quiz.rs` | `StreamProcessor` | bounded-window stats over a streaming input |

### Module 6 — LeetCode Problems · `06_leetcode/`

Applying the patterns above to real interview problems — each with multiple approaches to compare trade-offs.

| File | Problem | Difficulty | Core Pattern |
|---|---|---|---|
| `lc001_two_sum.rs` | Two Sum | Easy | HashMap complement lookup |
| `lc020_valid_parentheses.rs` | Valid Parentheses | Easy | Stack bracket matching |
| `lc026_remove_duplicates.rs` | Remove Duplicates from Sorted Array | Easy | Two pointers in-place |
| `lc121_best_time_stocks.rs` | Best Time to Buy and Sell Stock | Easy | One-pass min tracking + greedy variant |
| `lc217_contains_duplicate.rs` | Contains Duplicate | Easy | Brute / Sort / HashSet comparison |

---

## Patterns Covered

| Pattern | Introduced In | Reinforced By |
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

## Exercise Format

Every file follows the same shape — concept up top, well-scoped function stubs, edge cases listed explicitly, tests at the bottom:

```rust
// VECTORS — Exercise 4: Two Pointers
//
// Maintain two indices and move them toward each other or in lockstep.
// When to reach for it: in-place reversal, pair finding, deduplication
// on sorted data — anything that would otherwise be O(n²).

// --- Exercise 4b ---
// Find two indices i < j whose sorted values sum to target. O(n).
//
// Edge cases the tests check:
//   - [1,2,3,4] target 5 → (0, 3)
//   - [1,2,3]   target 7 → no pair (function preconditioned to find one)
pub fn two_sum_sorted(nums: &[i32], target: i32) -> (usize, usize) {
    todo!("two pointers; sum<target → left+=1, sum>target → right-=1")
}

#[cfg(test)]
mod tests { /* ... */ }
```

The `todo!()` message names the *algorithm to reach for*, not the answer. The runner shows this hint next to each failing function. The tests tell you when you're done.

---

## Project Structure

```
src/
├── main.rs                       # entry: launches reviewer::watcher::run()
├── reviewer/                     # the runner engine (~1,600 LOC)
│   ├── catalog.rs                #   curriculum data (sections, files)
│   ├── hints.rs                  #   per-function hint database
│   ├── parser.rs                 #   cargo test output → structured results
│   ├── results.rs                #   result types + ExerciseStatus
│   ├── runner.rs                 #   subprocess + compile-error extraction
│   ├── render.rs                 #   crossterm TUI (responsive 2-column)
│   ├── theme.rs                  #   colors, dimensions, box chars
│   └── watcher.rs                #   event loop, debouncing, stdin
├── exercises/                    # 28 files, 117 todo!() stubs
│   ├── 01_vectors/  …  06_leetcode/
└── solutions/                    # mirrored reference impls (opt-in feature)
    └── 01_vectors/  …  06_leetcode/
```

---

## Stack

- **Language:** Rust, edition 2024
- **Runner:** `cargo run` — interactive, file-watching TUI
- **Tests:** `cargo test` — 148 exercise tests + 3 meta-tests for catalog/hints consistency
- **Dependencies:** [`notify` 6](https://crates.io/crates/notify) for filesystem events, [`crossterm` 0.28](https://crates.io/crates/crossterm) for terminal control. Nothing else.

---

## Inspirations

[Rustlings](https://github.com/rust-lang/rustlings) for the watch-and-iterate workflow, [LeetCode](https://leetcode.com) for the interview-grade problem set. The engine, curriculum, hint system, and TUI are all original.
