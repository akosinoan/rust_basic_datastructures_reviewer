pub const BAR_WIDTH: usize = 20;

pub const RULE: &str =
    "──────────────────────────────────────────────────────────────";

pub const HEADER_TOP: &str =
    "┌──────────────────────────────────────────────────────────────┐";
pub const HEADER_TITLE_RUNNING: &str =
    "│          Rust DSA Reviewer · Phase 1 Foundations            │";
pub const HEADER_TITLE_RESULTS: &str =
    "│          Rust DSA Reviewer ·                                 │";
pub const HEADER_BOTTOM: &str =
    "└──────────────────────────────────────────────────────────────┘";

pub const FOOTER: &str =
    "save to re-run · type 'hint' for help · Ctrl+C to quit";

pub const DEBOUNCE_MS: u64 = 300;

pub fn progress_bar(done: usize, total: usize) -> String {
    let filled = if total > 0 {
        done * BAR_WIDTH / total
    } else {
        0
    };
    format!("{}{}", "█".repeat(filled), "░".repeat(BAR_WIDTH - filled))
}
