pub struct Section {
    pub dir: &'static str,
    pub module: &'static str,
    pub exercises: &'static [&'static str],
}

impl Section {
    pub fn file_path(&self, exercise: &str) -> String {
        format!("src/exercises/{}/{}.rs", self.dir, exercise)
    }
}

pub const SECTIONS: &[Section] = &[
    Section {
        dir: "01_vectors",
        module: "vectors",
        exercises: &[
            "ex01_basics",
            "ex02_iteration",
            "ex03_sorting",
            "ex04_two_pointers",
            "ex05_prefix_sum",
            "quiz",
        ],
    },
    Section {
        dir: "02_hashmaps",
        module: "hashmaps",
        exercises: &[
            "ex01_basics",
            "ex02_frequency",
            "ex03_entry_api",
            "ex04_two_sum",
            "quiz",
        ],
    },
    Section {
        dir: "03_strings",
        module: "strings",
        exercises: &[
            "ex01_basics",
            "ex02_chars",
            "ex03_manipulation",
            "ex04_palindrome",
            "quiz",
        ],
    },
    Section {
        dir: "04_stacks",
        module: "stacks",
        exercises: &["ex01_basics", "ex02_valid_parens", "ex03_monotonic", "quiz"],
    },
    Section {
        dir: "05_queues",
        module: "queues",
        exercises: &["ex01_basics", "ex02_sliding_window", "quiz"],
    },
    Section {
        dir: "06_leetcode",
        module: "leetcode",
        exercises: &[
            "lc001_two_sum",
            "lc020_valid_parentheses",
            "lc026_remove_duplicates",
            "lc121_best_time_stocks",
            "lc217_contains_duplicate",
        ],
    },
];

pub fn total_exercises() -> usize {
    SECTIONS.iter().map(|s| s.exercises.len()).sum()
}
