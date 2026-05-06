use std::io::{self, Write};

use crossterm::{cursor, execute, style::Stylize, terminal};

use super::catalog::{Section, SECTIONS, total_exercises};
use super::results::{ExerciseStatus, Results, TestResult};
use super::theme::{
    FOOTER, HEADER_BOTTOM, HEADER_TITLE_RESULTS, HEADER_TITLE_RUNNING, HEADER_TOP, RULE,
    progress_bar,
};

const ERROR_RULE_LEN: usize = 40;
const FN_NAME_PAD: usize = 32;
const SECTION_LABEL_PAD: usize = 14;
const DEFAULT_HINT: &str = "implement the todo!()";

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
        let summary = self.render_progress(results);

        if summary.completed_files == summary.total_files {
            self.render_complete_banner();
        } else if let Some((section, file)) = summary.current {
            self.render_current_exercise(section, file, results);
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

    fn render_progress<'a>(&mut self, results: &Results) -> ProgressSummary<'a> {
        println!("  {}", "Progress".bold());
        println!("  {}", RULE.dark_grey());

        let mut completed_files = 0;
        let mut current: Option<(&'static Section, &'static str)> = None;

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

            if current.is_none() {
                current = section
                    .exercises
                    .iter()
                    .find(|file| {
                        !ExerciseStatus::from_file(module_results.and_then(|m| m.get(**file)))
                            .is_complete()
                    })
                    .map(|file| (section, *file));
            }

            render_section_row(section, file_done);
        }

        let total_files: usize = total_exercises();

        println!();
        println!(
            "  {}  {} / {} files complete",
            "Total:".bold(),
            completed_files.to_string().bold(),
            total_files
        );
        println!();

        ProgressSummary {
            completed_files,
            total_files,
            current,
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

    fn render_current_exercise(&mut self, section: &Section, file: &str, results: &Results) {
        let heading = format!(" Current ──── {}::{} ", section.module, file);
        let pad = RULE.len().saturating_sub(heading.len());
        println!("  {}{}", heading.cyan().bold(), "─".repeat(pad).dark_grey());
        println!();
        println!("  {}", section.file_path(file).dark_grey());
        println!();

        if let Some(test_map) = results.get(section.module).and_then(|m| m.get(file)) {
            self.render_test_list(test_map);
        }

        println!("  {}", RULE.dark_grey());
    }

    fn render_test_list(&mut self, test_map: &super::results::FileResults) {
        let mut tests: Vec<(&String, &TestResult)> = test_map.iter().collect();
        tests.sort_unstable_by(|(a, _), (b, _)| a.as_str().cmp(b.as_str()));

        let pass_count = tests.iter().filter(|(_, r)| r.passed).count();
        println!(
            "  {} / {} passing",
            pass_count.to_string().bold(),
            tests.len()
        );
        println!();

        for (name, result) in &tests {
            let fn_name = format_fn_name(name);
            if result.passed {
                println!("  {}  {}", "✓".green().bold(), fn_name.green());
            } else {
                let hint = result.hint.as_deref().unwrap_or(DEFAULT_HINT);
                println!(
                    "  {}  {:<width$} {}",
                    "✗".red().bold(),
                    fn_name.red(),
                    hint.dark_grey(),
                    width = FN_NAME_PAD,
                );
            }
        }
        println!();
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
}

struct ProgressSummary<'a> {
    completed_files: usize,
    total_files: usize,
    current: Option<(&'a Section, &'a str)>,
}

fn render_section_row(section: &Section, file_done: usize) {
    let total = section.exercises.len();
    let bar = progress_bar(file_done, total);
    let count = format!("{} / {}", file_done, total);
    let label = format!("{:<width$}", section.dir, width = SECTION_LABEL_PAD);

    match ExerciseStatus::from_counts(file_done, total) {
        ExerciseStatus::Complete => println!(
            "  {} {} {}",
            label.green().bold(),
            bar.green(),
            count.green()
        ),
        ExerciseStatus::Partial => println!(
            "  {} {} {}",
            label.yellow().bold(),
            bar.yellow(),
            count.yellow()
        ),
        ExerciseStatus::Pending => println!(
            "  {} {} {}",
            label.dark_grey(),
            bar.dark_grey(),
            count.dark_grey()
        ),
    }
}

fn format_fn_name(raw: &str) -> String {
    let stripped = raw.strip_prefix("test_").unwrap_or(raw);
    format!("{}()", stripped)
}
