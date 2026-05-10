use std::io::{self, Write};

use crossterm::{cursor, execute, style::Stylize, terminal};

use super::catalog::{Section, SECTIONS, total_exercises};
use super::hints::{self, ExerciseHint};
use super::results::{ExerciseStatus, FileResults, Results, TestResult, current_exercise};
use super::theme::{
    FOOTER, HEADER_BOTTOM, HEADER_TITLE_RESULTS, HEADER_TITLE_RUNNING, HEADER_TOP, RULE, RULE_HALF,
    progress_bar,
};

const ERROR_RULE_LEN: usize = 40;
const FN_NAME_PAD: usize = 32;
const SECTION_LABEL_PAD: usize = 14;
const HINT_RULE_LEN: usize = 50;
const DEFAULT_HINT: &str = "type `hint` for help";

const LEFT_COL_WIDTH: usize = 50;
const COL_GAP: usize = 4;
const RIGHT_COL_WIDTH: usize = 60;
const SIDE_BY_SIDE_MIN_COLS: usize = LEFT_COL_WIDTH + COL_GAP + RIGHT_COL_WIDTH;

struct Line {
    text: String,
    width: usize,
}

impl Line {
    fn blank() -> Self {
        Self {
            text: String::new(),
            width: 0,
        }
    }
}

pub struct Renderer {
    stdout: io::Stdout,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            stdout: io::stdout(),
        }
    }

    pub fn show_running(&mut self) {
        self.clear_screen();
        println!("{}", HEADER_TOP.dark_grey());
        println!("{}", HEADER_TITLE_RUNNING.dark_grey());
        println!("{}", HEADER_BOTTOM.dark_grey());
        println!();
        println!("  {}", "Running tests...".dark_yellow());
        let _ = self.stdout.flush();
    }

    pub fn render(&mut self, results: &Results, compile_error: Option<&str>) {
        self.clear_screen();
        self.render_header();

        let (summary, progress_lines) = build_progress(results);
        let all_done = summary.completed_files == summary.total_files;

        match (all_done, summary.current) {
            (false, Some((section, file))) => {
                let exercise_lines = build_current_exercise(section, file, results);
                let term_cols = terminal::size().map(|(c, _)| c as usize).unwrap_or(80);
                if term_cols >= SIDE_BY_SIDE_MIN_COLS {
                    self.render_columns(&progress_lines, &exercise_lines);
                } else {
                    print_lines(&progress_lines);
                    print_lines(&exercise_lines);
                }
            }
            _ => {
                print_lines(&progress_lines);
                if all_done {
                    self.render_complete_banner();
                }
            }
        }

        if let Some(err) = compile_error {
            self.render_compile_error(err);
        }

        self.render_footer();
        let _ = self.stdout.flush();
    }

    fn clear_screen(&mut self) {
        let _ = execute!(
            self.stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        );
    }

    fn render_header(&mut self) {
        println!("{}", HEADER_TOP.dark_grey());
        println!("{}", HEADER_TITLE_RESULTS.dark_grey());
        println!("{}", HEADER_BOTTOM.dark_grey());
        println!();
    }

    fn render_columns(&mut self, left: &[Line], right: &[Line]) {
        let rows = left.len().max(right.len());
        for i in 0..rows {
            let l_text = left.get(i).map(|x| x.text.as_str()).unwrap_or("");
            let l_width = left.get(i).map(|x| x.width).unwrap_or(0);
            let r_text = right.get(i).map(|x| x.text.as_str()).unwrap_or("");

            if r_text.is_empty() {
                println!("{}", l_text);
            } else {
                let pad = LEFT_COL_WIDTH.saturating_sub(l_width) + COL_GAP;
                println!("{}{}{}", l_text, " ".repeat(pad), r_text);
            }
        }
    }

    fn render_complete_banner(&mut self) {
        let banner = format!(
            "  All {} exercises complete! Phase 1 done.  ",
            total_exercises()
        );
        println!("  {}", RULE.green());
        println!("  {}", banner.black().on_green().bold());
        println!("  {}", RULE.green());
    }

    fn render_compile_error(&mut self, err: &str) {
        println!();
        println!(
            "  {} {}",
            "✗ Compilation Error".black().on_red().bold(),
            "─".repeat(ERROR_RULE_LEN).red()
        );
        println!();
        for line in err.lines() {
            println!("  {}", line.red());
        }
        println!();
        println!(
            "  {}",
            "Fix the error above and save to continue.".dark_yellow()
        );
        println!("  {}", RULE.red());
    }

    fn render_footer(&mut self) {
        println!();
        println!("  {}", FOOTER.dark_grey());
    }

    pub fn show_hint(&mut self, results: &Results) {
        println!();
        let Some((section, file)) = current_exercise(results) else {
            println!(
                "  {}",
                "All exercises complete — no hint to show.".green().bold()
            );
            let _ = self.stdout.flush();
            return;
        };

        let Some(file_hints) = hints::hints_for(section.module, file) else {
            println!(
                "  {} {}::{}",
                "No hint available for".dark_yellow(),
                section.module.dark_yellow(),
                file.dark_yellow()
            );
            let _ = self.stdout.flush();
            return;
        };

        let file_results = results.get(section.module).and_then(|m| m.get(file));
        let target = first_failing_hint(file_hints.hints, file_results);

        let bar = "─".repeat(HINT_RULE_LEN);
        let location = format!("{}::{}", section.module, file);
        let header_label = format!(" Hint  {} ", location);
        let header_pad = HINT_RULE_LEN.saturating_sub(header_label.chars().count() + 2);
        println!(
            "  {}{}{}",
            "──".dark_grey(),
            header_label.cyan().bold(),
            "─".repeat(header_pad).dark_grey()
        );

        match target {
            Some(h) => {
                println!("    {}", h.fn_name.bold());
                println!("    {} {}", "→".cyan(), h.hint.dark_grey());
            }
            None => {
                println!(
                    "  {}",
                    format!(
                        "No failing function in {}::{} — save the file to re-run tests.",
                        section.module, file
                    )
                    .dark_yellow()
                );
            }
        }

        println!("  {}", bar.dark_grey());
        let _ = self.stdout.flush();
    }
}

fn print_lines(lines: &[Line]) {
    for line in lines {
        println!("{}", line.text);
    }
}

fn build_progress(results: &Results) -> (ProgressSummary, Vec<Line>) {
    let mut lines: Vec<Line> = Vec::new();

    let title = "Progress";
    lines.push(Line {
        text: format!("  {}", title.bold()),
        width: 2 + title.chars().count(),
    });
    lines.push(Line {
        text: format!("  {}", RULE_HALF.dark_grey()),
        width: 2 + RULE_HALF.chars().count(),
    });

    let mut completed_files = 0;
    for section in SECTIONS {
        let module_results = results.get(section.module);
        let file_done = section
            .exercises
            .iter()
            .filter(|file| {
                ExerciseStatus::from_file(module_results.and_then(|m| m.get(**file)))
                    .is_complete()
            })
            .count();
        completed_files += file_done;
        lines.push(section_row_line(section, file_done));
    }

    let current = current_exercise(results);
    let total_files: usize = total_exercises();

    lines.push(Line::blank());

    let completed_str = completed_files.to_string();
    let total_str = total_files.to_string();
    let total_text = format!(
        "  {}  {} / {} files complete",
        "Total:".bold(),
        completed_str.clone().bold(),
        total_str
    );
    let total_width = 2 + "Total:".chars().count()
        + 2
        + completed_str.chars().count()
        + 3
        + total_str.chars().count()
        + " files complete".chars().count();
    lines.push(Line {
        text: total_text,
        width: total_width,
    });
    lines.push(Line::blank());

    (
        ProgressSummary {
            completed_files,
            total_files,
            current,
        },
        lines,
    )
}

fn build_current_exercise(section: &Section, file: &str, results: &Results) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();

    let heading = format!(" Current ──── {}::{} ", section.module, file);
    let heading_width = heading.chars().count();
    let pad = RIGHT_COL_WIDTH.saturating_sub(heading_width);
    lines.push(Line {
        text: format!(
            "  {}{}",
            heading.cyan().bold(),
            "─".repeat(pad).dark_grey()
        ),
        width: 2 + heading_width + pad,
    });
    lines.push(Line::blank());

    let path = section.file_path(file);
    let path_width = 2 + path.chars().count();
    lines.push(Line {
        text: format!("  {}", path.dark_grey()),
        width: path_width,
    });
    lines.push(Line::blank());

    if let Some(test_map) = results.get(section.module).and_then(|m| m.get(file)) {
        lines.extend(test_list_lines(test_map));
    }

    let bottom_rule_width = RIGHT_COL_WIDTH;
    let bottom_rule = "─".repeat(bottom_rule_width);
    lines.push(Line {
        text: format!("  {}", bottom_rule.dark_grey()),
        width: 2 + bottom_rule_width,
    });

    lines
}

fn test_list_lines(test_map: &FileResults) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let mut tests: Vec<(&String, &TestResult)> = test_map.iter().collect();
    tests.sort_unstable_by(|(a, _), (b, _)| a.as_str().cmp(b.as_str()));

    let pass_count = tests.iter().filter(|(_, r)| r.passed).count();
    let pass_str = pass_count.to_string();
    let total_str = tests.len().to_string();
    let summary_width =
        2 + pass_str.chars().count() + 3 + total_str.chars().count() + " passing".chars().count();
    lines.push(Line {
        text: format!("  {} / {} passing", pass_str.clone().bold(), total_str),
        width: summary_width,
    });
    lines.push(Line::blank());

    for (name, result) in &tests {
        let fn_name = format_fn_name(name);
        if result.passed {
            let width = 2 + 1 + 2 + fn_name.chars().count();
            let text = format!("  {}  {}", "✓".green().bold(), fn_name.clone().green());
            lines.push(Line { text, width });
        } else {
            let hint = result.hint.as_deref().unwrap_or(DEFAULT_HINT);
            let padded_fn = format!("{:<width$}", fn_name, width = FN_NAME_PAD);
            let visible_padded = padded_fn.chars().count();
            let width = 2 + 1 + 2 + visible_padded + 1 + hint.chars().count();
            let text = format!(
                "  {}  {} {}",
                "✗".red().bold(),
                padded_fn.red(),
                hint.dark_grey(),
            );
            lines.push(Line { text, width });
        }
    }
    lines.push(Line::blank());

    lines
}

fn first_failing_hint<'a>(
    hints: &'a [ExerciseHint],
    file_results: Option<&FileResults>,
) -> Option<&'a ExerciseHint> {
    if hints.is_empty() {
        return None;
    }
    let Some(tests) = file_results else {
        return hints.first();
    };
    if !tests.values().any(|r| !r.passed) {
        return None;
    }
    if let Some(h) = hints.iter().find(|h| {
        tests.iter().any(|(name, r)| !r.passed && test_belongs_to_fn(name, h.fn_name))
    }) {
        return Some(h);
    }
    hints.first()
}

fn test_belongs_to_fn(test_name: &str, fn_name: &str) -> bool {
    let Some(rest) = test_name.strip_prefix("test_") else {
        return false;
    };
    if rest == fn_name {
        return true;
    }
    rest.strip_prefix(fn_name)
        .map(|tail| tail.starts_with('_'))
        .unwrap_or(false)
}

struct ProgressSummary {
    completed_files: usize,
    total_files: usize,
    current: Option<(&'static Section, &'static str)>,
}

fn section_row_line(section: &Section, file_done: usize) -> Line {
    let total = section.exercises.len();
    let bar = progress_bar(file_done, total);
    let count = format!("{} / {}", file_done, total);
    let label = format!("{:<width$}", section.dir, width = SECTION_LABEL_PAD);

    let width = 2 + label.chars().count() + 1 + bar.chars().count() + 1 + count.chars().count();

    let text = match ExerciseStatus::from_counts(file_done, total) {
        ExerciseStatus::Complete => format!(
            "  {} {} {}",
            label.green().bold(),
            bar.green(),
            count.green()
        ),
        ExerciseStatus::Partial => format!(
            "  {} {} {}",
            label.yellow().bold(),
            bar.yellow(),
            count.yellow()
        ),
        ExerciseStatus::Pending => format!(
            "  {} {} {}",
            label.dark_grey(),
            bar.dark_grey(),
            count.dark_grey()
        ),
    };

    Line { text, width }
}

fn format_fn_name(raw: &str) -> String {
    let stripped = raw.strip_prefix("test_").unwrap_or(raw);
    format!("{}()", stripped)
}
