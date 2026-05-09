pub struct ExerciseHint {
    pub fn_name: &'static str,
    pub hint: &'static str,
}

pub struct FileHints {
    pub module: &'static str,
    pub file: &'static str,
    pub hints: &'static [ExerciseHint],
}

pub fn hints_for(module: &str, file: &str) -> Option<&'static FileHints> {
    HINTS.iter().find(|h| h.module == module && h.file == file)
}

pub const HINTS: &[FileHints] = &[
    // ======================================================================
    // 01_vectors
    // ======================================================================
    FileHints {
        module: "vectors",
        file: "ex01_basics",
        hints: &[
            ExerciseHint {
                fn_name: "build_vec",
                hint: "Vec::new() or vec![], then .push(x) for each value 1..=5",
            },
            ExerciseHint {
                fn_name: "last_element",
                hint: ".last() returns Option<&T>; use match or unwrap_or with -1 sentinel",
            },
            ExerciseHint {
                fn_name: "safe_get",
                hint: "v.get(i) returns Option<&T>; chain .copied() to lift to Option<i32>",
            },
            ExerciseHint {
                fn_name: "drain_and_count",
                hint: "record .len() before .clear() (clearing zeroes the length)",
            },
        ],
    },
    FileHints {
        module: "vectors",
        file: "ex02_iteration",
        hints: &[
            ExerciseHint {
                fn_name: "sum",
                hint: "v.iter().sum::<i32>() — let inference fail and turbofish the sum",
            },
            ExerciseHint {
                fn_name: "double_all",
                hint: "v.iter().map(|&x| x * 2).collect()",
            },
            ExerciseHint {
                fn_name: "keep_evens",
                hint: "v.iter().filter(|&&x| x % 2 == 0).copied().collect()",
            },
            ExerciseHint {
                fn_name: "first_negative_index",
                hint: "v.iter().position(|&x| x < 0) — already returns Option<usize>",
            },
            ExerciseHint {
                fn_name: "concat_all",
                hint: "v.iter().map(String::as_str).collect::<Vec<_>>().join(\"\")",
            },
        ],
    },
    FileHints {
        module: "vectors",
        file: "ex03_sorting",
        hints: &[
            ExerciseHint {
                fn_name: "sort_asc",
                hint: "v.sort() (in place — i32 is Ord), then return v",
            },
            ExerciseHint {
                fn_name: "sort_desc",
                hint: "v.sort_by(|a, b| b.cmp(a)) — flip the comparison",
            },
            ExerciseHint {
                fn_name: "sort_by_length",
                hint: "v.sort_by_key(|s| s.len())",
            },
            ExerciseHint {
                fn_name: "binary_search",
                hint: "v.binary_search(&target).is_ok() — Ok(idx) on hit, Err(_) on miss",
            },
            ExerciseHint {
                fn_name: "remove_duplicates",
                hint: ".sort() first, then .dedup() (dedup only collapses adjacent dupes)",
            },
            ExerciseHint {
                fn_name: "max_window_sum",
                hint: "v.windows(k).map(|w| w.iter().sum()).max().unwrap_or(0)",
            },
        ],
    },
    FileHints {
        module: "vectors",
        file: "ex04_two_pointers",
        hints: &[
            ExerciseHint {
                fn_name: "reverse_in_place",
                hint: "left=0, right=len-1; while left<right: v.swap(left,right); left+=1; right-=1",
            },
            ExerciseHint {
                fn_name: "two_sum_sorted",
                hint: "two pointers; sum<target → left+=1, sum>target → right-=1, equal → return",
            },
            ExerciseHint {
                fn_name: "remove_duplicates_in_place",
                hint: "slow k=1 (write head), fast i scans; advance k only when v[i] != v[k-1]",
            },
            ExerciseHint {
                fn_name: "two_sum_unsorted_brute",
                hint: "nested loops i in 0..n, j in i+1..n; check nums[i]+nums[j]==target",
            },
        ],
    },
    FileHints {
        module: "vectors",
        file: "ex05_prefix_sum",
        hints: &[
            ExerciseHint {
                fn_name: "build_prefix_sum",
                hint: "result of length nums.len()+1; result[0]=0; result[i]=result[i-1]+nums[i-1]",
            },
            ExerciseHint {
                fn_name: "range_sum",
                hint: "prefix[r+1] - prefix[l] gives the sum of nums[l..=r]",
            },
            ExerciseHint {
                fn_name: "subarray_sum_exists",
                hint: "running prefix sum + HashSet of seen sums; if (sum - target) is in the set, found",
            },
            ExerciseHint {
                fn_name: "product_except_self",
                hint: "prefix products L→R, suffix products R→L; result[i] = prefix[i] * suffix[i]",
            },
        ],
    },
    FileHints {
        module: "vectors",
        file: "quiz",
        hints: &[
            ExerciseHint {
                fn_name: "new",
                hint: "RunningStats { data: Vec::new() }",
            },
            ExerciseHint {
                fn_name: "push",
                hint: "self.data.push(val)",
            },
            ExerciseHint {
                fn_name: "min",
                hint: "self.data.iter().min().copied() — Option<&i32> → Option<i32>",
            },
            ExerciseHint {
                fn_name: "max",
                hint: "self.data.iter().max().copied()",
            },
            ExerciseHint {
                fn_name: "mean",
                hint: "sum as i64 to avoid i32 overflow, then divide by len() as f64",
            },
            ExerciseHint {
                fn_name: "median",
                hint: "clone+sort; odd len → middle; even len → average of the two middles",
            },
            ExerciseHint {
                fn_name: "mode",
                hint: "HashMap<i32,usize> frequency map, then .iter().max_by_key(|(_,&c)| c)",
            },
        ],
    },
    // ======================================================================
    // 02_hashmaps
    // ======================================================================
    FileHints {
        module: "hashmaps",
        file: "ex01_basics",
        hints: &[
            ExerciseHint {
                fn_name: "build_map",
                hint: "for &(k,v) in pairs: map.insert(k.to_string(), v)",
            },
            ExerciseHint {
                fn_name: "get_or_zero",
                hint: "map.get(key).copied().unwrap_or(0)",
            },
            ExerciseHint {
                fn_name: "remove_key",
                hint: "map.remove(key) — already returns Option<i32>",
            },
            ExerciseHint {
                fn_name: "sorted_keys",
                hint: "map.keys().cloned().collect::<Vec<_>>(), then .sort()",
            },
            ExerciseHint {
                fn_name: "merge_maps",
                hint: "start with `a`; for (k,v) in b: *a.entry(k).or_insert(0) += v",
            },
        ],
    },
    FileHints {
        module: "hashmaps",
        file: "ex02_frequency",
        hints: &[
            ExerciseHint {
                fn_name: "char_frequency",
                hint: "for c in s.chars(): *map.entry(c).or_insert(0) += 1",
            },
            ExerciseHint {
                fn_name: "most_frequent_char",
                hint: "char_frequency, then .into_iter().max_by_key(|(_,c)| *c).map(|(k,_)| k)",
            },
            ExerciseHint {
                fn_name: "is_anagram",
                hint: "their character frequency maps must be equal",
            },
            ExerciseHint {
                fn_name: "group_by_first_char",
                hint: "first = w.chars().next().unwrap(); map.entry(first).or_insert_with(Vec::new).push(w.to_string())",
            },
            ExerciseHint {
                fn_name: "find_duplicates",
                hint: "frequency map, then collect keys whose count > 1 (set, so each appears once)",
            },
        ],
    },
    FileHints {
        module: "hashmaps",
        file: "ex03_entry_api",
        hints: &[
            ExerciseHint {
                fn_name: "increment",
                hint: "*map.entry(key.to_string()).or_insert(0) += 1",
            },
            ExerciseHint {
                fn_name: "sum_by_parity",
                hint: "bucket = if x%2==0 {\"even\"} else {\"odd\"}; *m.entry(bucket.to_string()).or_insert(0) += x",
            },
            ExerciseHint {
                fn_name: "highest_score",
                hint: "m.entry(name).and_modify(|old| *old = (*old).max(score)).or_insert(score)",
            },
            ExerciseHint {
                fn_name: "first_repeated_word",
                hint: "split_whitespace; entry counter; return first word whose new count equals 2",
            },
        ],
    },
    FileHints {
        module: "hashmaps",
        file: "ex04_two_sum",
        hints: &[
            ExerciseHint {
                fn_name: "two_sum",
                hint: "HashMap<value, index>; for each i: if (target-nums[i]) in map → return (j,i); else map.insert(nums[i], i)",
            },
            ExerciseHint {
                fn_name: "has_pair_with_sum",
                hint: "HashSet<i32> of seen; for n: if (target-n) seen → true; else insert n",
            },
            ExerciseHint {
                fn_name: "all_pairs_with_sum",
                hint: "HashSet of seen; on hit, push (min(a,b), max(a,b)) into a result HashSet to dedupe",
            },
            ExerciseHint {
                fn_name: "three_sum",
                hint: "sort; for each i: l=i+1, r=end two-pointer for the pair; skip duplicates of nums[i] and on each found triplet",
            },
        ],
    },
    FileHints {
        module: "hashmaps",
        file: "quiz",
        hints: &[
            ExerciseHint {
                fn_name: "new",
                hint: "WordCounter { counts: HashMap::new() }",
            },
            ExerciseHint {
                fn_name: "add_word",
                hint: "*self.counts.entry(word.to_lowercase()).or_insert(0) += 1",
            },
            ExerciseHint {
                fn_name: "add_text",
                hint: "for w in text.split_whitespace(): self.add_word(w)",
            },
            ExerciseHint {
                fn_name: "count",
                hint: "*self.counts.get(&word.to_lowercase()).unwrap_or(&0)",
            },
            ExerciseHint {
                fn_name: "most_frequent",
                hint: "self.counts.iter().max_by_key(|(_,&c)| c).map(|(k,_)| k.as_str())",
            },
            ExerciseHint {
                fn_name: "words_with_count",
                hint: "filter values == n, collect keys, then .sort()",
            },
            ExerciseHint {
                fn_name: "intersection",
                hint: "self.counts.keys().filter(|k| other.counts.contains_key(*k)).cloned().collect(), then sort",
            },
        ],
    },
    // ======================================================================
    // 03_strings
    // ======================================================================
    FileHints {
        module: "strings",
        file: "ex01_basics",
        hints: &[
            ExerciseHint {
                fn_name: "is_valid_identifier",
                hint: "non-empty + first char .is_alphabetic()||'_' + rest .all(|c| c.is_alphanumeric()||c=='_')",
            },
            ExerciseHint {
                fn_name: "capitalize",
                hint: "split first char with .chars().next(); .to_uppercase() it, then push lowercase rest",
            },
            ExerciseHint {
                fn_name: "word_count",
                hint: "s.split_whitespace().count() — collapses runs of whitespace automatically",
            },
            ExerciseHint {
                fn_name: "ends_with_any",
                hint: "suffixes.iter().any(|suf| s.ends_with(suf))",
            },
        ],
    },
    FileHints {
        module: "strings",
        file: "ex02_chars",
        hints: &[
            ExerciseHint {
                fn_name: "count_vowels",
                hint: "s.chars().filter(|c| \"aeiouAEIOU\".contains(*c)).count()",
            },
            ExerciseHint {
                fn_name: "reverse_string",
                hint: "s.chars().rev().collect() — char-aware (don't use .bytes() with non-ASCII)",
            },
            ExerciseHint {
                fn_name: "letters_only",
                hint: "s.chars().filter(|c| c.is_alphabetic()).collect()",
            },
            ExerciseHint {
                fn_name: "caesar_cipher",
                hint: "lowercase: ((c as u8 - b'a' + n) % 26 + b'a') as char; uppercase same with b'A'; non-letters unchanged",
            },
            ExerciseHint {
                fn_name: "loose_equal",
                hint: "on each: .chars().filter(|c| !c.is_whitespace()).collect::<String>().to_lowercase(); compare",
            },
        ],
    },
    FileHints {
        module: "strings",
        file: "ex03_manipulation",
        hints: &[
            ExerciseHint {
                fn_name: "split_csv",
                hint: "line.split(',').map(str::to_string).collect()",
            },
            ExerciseHint {
                fn_name: "reverse_words",
                hint: "s.split_whitespace().rev().collect::<Vec<_>>().join(\" \")",
            },
            ExerciseHint {
                fn_name: "snake_to_camel",
                hint: "split('_'); take first part as-is; capitalize each remaining part; concat",
            },
            ExerciseHint {
                fn_name: "longest_common_prefix",
                hint: "start with strs[0]; for each other s: while !s.starts_with(prefix), shrink prefix by 1 char",
            },
            ExerciseHint {
                fn_name: "run_length_encode",
                hint: "iterate chars with (current_char, run_len); on change, push count+char to output and reset",
            },
        ],
    },
    FileHints {
        module: "strings",
        file: "ex04_palindrome",
        hints: &[
            ExerciseHint {
                fn_name: "is_palindrome_exact",
                hint: "collect to Vec<char>; two pointers (l, r); on mismatch return false",
            },
            ExerciseHint {
                fn_name: "is_palindrome_lc",
                hint: "filter(|c| c.is_alphanumeric()), lowercase, then two-pointer check on Vec<char>",
            },
            ExerciseHint {
                fn_name: "is_palindrome_one_deletion",
                hint: "two pointers; on first mismatch return check(l+1,r) || check(l,r-1) using the helper",
            },
            ExerciseHint {
                fn_name: "all_palindrome_substrings",
                hint: "collect chars to Vec<char>; for each i, j>=i: build substring from slice; keep if palindrome",
            },
        ],
    },
    FileHints {
        module: "strings",
        file: "quiz",
        hints: &[
            ExerciseHint {
                fn_name: "new",
                hint: "TextAnalyzer { text: text.to_string() }",
            },
            ExerciseHint {
                fn_name: "word_count",
                hint: "self.text.split_whitespace().count()",
            },
            ExerciseHint {
                fn_name: "char_frequency",
                hint: "for c in self.text.chars(): *map.entry(c).or_insert(0) += 1",
            },
            ExerciseHint {
                fn_name: "longest_word",
                hint: "self.text.split_whitespace().max_by_key(|w| w.len())",
            },
            ExerciseHint {
                fn_name: "is_pangram",
                hint: "collect lowercase alphabetic chars into a HashSet; check len() == 26",
            },
            ExerciseHint {
                fn_name: "run_length_encode",
                hint: "iterate chars; track (current, run_length); flush count+char on change and at end",
            },
        ],
    },
    // ======================================================================
    // 04_stacks
    // ======================================================================
    FileHints {
        module: "stacks",
        file: "ex01_basics",
        hints: &[
            ExerciseHint {
                fn_name: "eval_rpn",
                hint: "Vec<i32> stack; .parse::<i32>() to detect numbers and push; on operator pop b then a, push (a op b)",
            },
            ExerciseHint {
                fn_name: "new",
                hint: "MinStack { stack: Vec::new(), min_stack: Vec::new() }",
            },
            ExerciseHint {
                fn_name: "push",
                hint: "stack.push(val); let new_min = (*self.min_stack.last().unwrap_or(&val)).min(val); min_stack.push(new_min)",
            },
            ExerciseHint {
                fn_name: "pop",
                hint: "self.stack.pop(); self.min_stack.pop() — keep them in lockstep",
            },
            ExerciseHint {
                fn_name: "top",
                hint: "*self.stack.last().unwrap()",
            },
            ExerciseHint {
                fn_name: "get_min",
                hint: "*self.min_stack.last().unwrap()",
            },
            ExerciseHint {
                fn_name: "balanced_parens",
                hint: "counter: '(' → +1, ')' → -1; if counter < 0 return false; final return counter == 0",
            },
        ],
    },
    FileHints {
        module: "stacks",
        file: "ex02_valid_parens",
        hints: &[
            ExerciseHint {
                fn_name: "is_valid",
                hint: "Vec<char> stack; openers push; for closer, pop and require matching opener; final stack must be empty",
            },
            ExerciseHint {
                fn_name: "min_add_to_make_valid",
                hint: "open and close counters; '(' → open+=1; ')' → if open>0 open-=1 else close+=1; return open+close",
            },
            ExerciseHint {
                fn_name: "check_valid_string",
                hint: "track [lo, hi] of possible unmatched '('; '(' → both+=1; ')' → both-=1 (lo clamped to 0); '*' → lo-=1, hi+=1; if hi<0 false; final lo==0",
            },
        ],
    },
    FileHints {
        module: "stacks",
        file: "ex03_monotonic",
        hints: &[
            ExerciseHint {
                fn_name: "next_greater_element",
                hint: "result init to -1s; stack of indices; while top.value < nums[i]: result[stack.pop()] = nums[i]; push i",
            },
            ExerciseHint {
                fn_name: "daily_temperatures",
                hint: "monotonic decreasing stack of indices; on warmer day: result[popped] = i - popped",
            },
            ExerciseHint {
                fn_name: "largest_rectangle_in_histogram",
                hint: "stack of indices, monotonic increasing; on heights[i] < top: pop, area = popped_height * (i - new_top - 1); drain remaining at end",
            },
        ],
    },
    FileHints {
        module: "stacks",
        file: "quiz",
        hints: &[
            ExerciseHint {
                fn_name: "eval_rpn",
                hint: "Vec<i32> stack; numbers parsed via .parse::<i32>().unwrap(); operators pop b then a, push (a op b)",
            },
            ExerciseHint {
                fn_name: "min_brackets_to_add",
                hint: "open and close counters; '(' → open+=1; ')' → open>0 ? open-=1 : close+=1; return open+close",
            },
            ExerciseHint {
                fn_name: "next_greater_element",
                hint: "monotonic stack of indices; while top.value < nums[i]: result[stack.pop()] = nums[i]; push i; remaining indices stay -1",
            },
        ],
    },
    // ======================================================================
    // 05_queues
    // ======================================================================
    FileHints {
        module: "queues",
        file: "ex01_basics",
        hints: &[
            ExerciseHint {
                fn_name: "fifo_order",
                hint: "VecDeque; push_back each task; pop_front into a result Vec<String>",
            },
            ExerciseHint {
                fn_name: "new",
                hint: "CircularBuffer { data: VecDeque::new(), capacity }",
            },
            ExerciseHint {
                fn_name: "enqueue",
                hint: "if self.data.len() == self.capacity, pop_front first; then push_back(val)",
            },
            ExerciseHint {
                fn_name: "rotate",
                hint: "VecDeque::from(items); for c in moves.chars(): 'L' → pop_front+push_back, 'R' → pop_back+push_front",
            },
        ],
    },
    FileHints {
        module: "queues",
        file: "ex02_sliding_window",
        hints: &[
            ExerciseHint {
                fn_name: "max_sum_fixed_window",
                hint: "sum first k; slide: sum += nums[i] - nums[i-k]; track running max",
            },
            ExerciseHint {
                fn_name: "longest_ones_after_one_deletion",
                hint: "sliding window with zeros_in_window; when zeros>1 shrink left; answer = max(window_size) - 1",
            },
            ExerciseHint {
                fn_name: "longest_unique_substring",
                hint: "HashMap<char,usize> last-seen; on duplicate, left = max(left, last_seen+1); track max(right-left+1)",
            },
            ExerciseHint {
                fn_name: "sliding_window_maximum",
                hint: "VecDeque<usize> of indices, decreasing nums values; pop_front out-of-window; pop_back smaller; record nums[front] from i==k-1",
            },
        ],
    },
    FileHints {
        module: "queues",
        file: "quiz",
        hints: &[
            ExerciseHint {
                fn_name: "new",
                hint: "StreamProcessor { window: VecDeque::new() }",
            },
            ExerciseHint {
                fn_name: "enqueue",
                hint: "self.window.push_back(val)",
            },
            ExerciseHint {
                fn_name: "dequeue",
                hint: "self.window.pop_front()",
            },
            ExerciseHint {
                fn_name: "window_max",
                hint: "self.window.iter().max().copied()",
            },
            ExerciseHint {
                fn_name: "sliding_window_maxes",
                hint: "VecDeque<usize> of indices, decreasing nums values; pop_front out-of-window; pop_back ≤; push i; from i==k-1 record nums[front]",
            },
            ExerciseHint {
                fn_name: "longest_subarray_sum_le",
                hint: "two pointers; expand right (sum += nums[r]); while sum > k shrink left; track max(r-l+1)",
            },
        ],
    },
    // ======================================================================
    // 06_leetcode
    // ======================================================================
    FileHints {
        module: "leetcode",
        file: "lc001_two_sum",
        hints: &[
            ExerciseHint {
                fn_name: "two_sum",
                hint: "HashMap<i32, i32> mapping value → index; for each i: if (target - nums[i]) is in the map, return [that, i]; else insert nums[i] → i",
            },
        ],
    },
    FileHints {
        module: "leetcode",
        file: "lc020_valid_parentheses",
        hints: &[
            ExerciseHint {
                fn_name: "is_valid",
                hint: "Vec<char> stack; push openers; for closers, pop and require the matching opener (helper or match); final stack must be empty",
            },
        ],
    },
    FileHints {
        module: "leetcode",
        file: "lc026_remove_duplicates",
        hints: &[
            ExerciseHint {
                fn_name: "remove_duplicates",
                hint: "handle empty → 0; let k=1; for i in 1..len: if nums[i] != nums[k-1] { nums[k] = nums[i]; k+=1 }; return k as i32",
            },
        ],
    },
    FileHints {
        module: "leetcode",
        file: "lc121_best_time_stocks",
        hints: &[
            ExerciseHint {
                fn_name: "max_profit",
                hint: "min_price=i32::MAX, max_profit=0; for p: min_price=min(min_price,p); max_profit=max(max_profit, p - min_price)",
            },
            ExerciseHint {
                fn_name: "max_profit_ii",
                hint: "for w in prices.windows(2): if w[1] > w[0] { profit += w[1] - w[0] }",
            },
        ],
    },
    FileHints {
        module: "leetcode",
        file: "lc217_contains_duplicate",
        hints: &[
            ExerciseHint {
                fn_name: "contains_duplicate_brute",
                hint: "two nested loops i in 0..n, j in i+1..n; if nums[i]==nums[j] return true",
            },
            ExerciseHint {
                fn_name: "contains_duplicate_sort",
                hint: "nums.sort(); nums.windows(2).any(|w| w[0] == w[1])",
            },
            ExerciseHint {
                fn_name: "contains_duplicate",
                hint: "HashSet::new(); for n in nums { if !set.insert(n) { return true } }; false",
            },
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reviewer::catalog::SECTIONS;
    use std::collections::HashSet;

    #[test]
    fn hints_match_catalog() {
        let mut catalog_pairs: HashSet<(&str, &str)> = HashSet::new();
        for section in SECTIONS {
            for file in section.exercises {
                catalog_pairs.insert((section.module, *file));
            }
        }

        let mut hint_pairs: HashSet<(&str, &str)> = HashSet::new();
        for entry in HINTS {
            assert!(
                hint_pairs.insert((entry.module, entry.file)),
                "duplicate hints entry for {}::{}",
                entry.module,
                entry.file
            );
        }

        let missing_hints: Vec<_> = catalog_pairs.difference(&hint_pairs).collect();
        assert!(
            missing_hints.is_empty(),
            "catalog files without hints: {:?}",
            missing_hints
        );

        let orphan_hints: Vec<_> = hint_pairs.difference(&catalog_pairs).collect();
        assert!(
            orphan_hints.is_empty(),
            "hints entries not in catalog: {:?}",
            orphan_hints
        );
    }

    #[test]
    fn every_file_has_at_least_one_hint() {
        for entry in HINTS {
            assert!(
                !entry.hints.is_empty(),
                "{}::{} has no hints",
                entry.module,
                entry.file
            );
            for h in entry.hints {
                assert!(!h.fn_name.is_empty(), "empty fn_name in {}::{}", entry.module, entry.file);
                assert!(!h.hint.is_empty(), "empty hint for {}::{}::{}", entry.module, entry.file, h.fn_name);
            }
        }
    }

    #[test]
    fn hints_for_lookup_works() {
        assert!(hints_for("vectors", "ex01_basics").is_some());
        assert!(hints_for("leetcode", "lc001_two_sum").is_some());
        assert!(hints_for("nope", "nada").is_none());
    }
}
