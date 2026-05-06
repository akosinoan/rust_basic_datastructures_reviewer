mod exercises;

#[cfg(feature = "solutions")]
mod solutions;

use crossterm::{cursor, execute, style::Stylize, terminal};
use notify::{Event, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::sync::mpsc;
use std::time::Duration;

// ── Exercise sequence ────────────────────────────────────────────────────────

static SECTIONS: &[(&str, &str, &[&str])] = &[
    (
        "01_vectors",
        "vectors",
        &[
            "ex01_basics",
            "ex02_iteration",
            "ex03_sorting",
            "ex04_two_pointers",
            "ex05_prefix_sum",
            "quiz",
        ],
    ),
    (
        "02_hashmaps",
        "hashmaps",
        &[
            "ex01_basics",
            "ex02_frequency",
            "ex03_entry_api",
            "ex04_two_sum",
            "quiz",
        ],
    ),
    (
        "03_strings",
        "strings",
        &[
            "ex01_basics",
            "ex02_chars",
            "ex03_manipulation",
            "ex04_palindrome",
            "quiz",
        ],
    ),
    (
        "04_stacks",
        "stacks",
        &["ex01_basics", "ex02_valid_parens", "ex03_monotonic", "quiz"],
    ),
    (
        "05_queues",
        "queues",
        &["ex01_basics", "ex02_sliding_window", "quiz"],
    ),
    (
        "06_leetcode",
        "leetcode",
        &[
            "lc001_two_sum",
            "lc020_valid_parentheses",
            "lc026_remove_duplicates",
            "lc121_best_time_stocks",
            "lc217_contains_duplicate",
        ],
    ),
];

// ── Data model ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct TestResult {
    passed: bool,
    hint: Option<String>,
}

// module → file → test_name → result
type Results = HashMap<String, HashMap<String, HashMap<String, TestResult>>>;

// ── Test runner ──────────────────────────────────────────────────────────────

// Returns Ok(combined output) on success, Err(formatted error) on compile failure.
fn run_cargo_test() -> Result<String, String> {
    let output = Command::new("cargo")
        .args(["test"])
        .env("RUSTFLAGS", "-Awarnings")
        .output()
        .expect("failed to spawn cargo test");

    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

    if stderr.contains("error[") || stderr.contains("error: could not compile") {
        return Err(extract_compile_error(&stderr));
    }

    let mut combined = stdout;
    combined.push_str(&stderr);
    Ok(combined)
}

fn extract_compile_error(stderr: &str) -> String {
    let lines: Vec<&str> = stderr.lines().collect();

    // Find where the first real error starts
    let start = lines
        .iter()
        .position(|l| l.starts_with("error[") || l.starts_with("error: "))
        .unwrap_or(0);

    // Find where "could not compile" ends the block
    let end = lines
        .iter()
        .rposition(|l| l.contains("could not compile"))
        .map(|i| i + 1)
        .unwrap_or(lines.len());

    lines[start..end.min(lines.len())]
        .iter()
        // Drop trailing "aborting due to" noise — "could not compile" is cleaner
        .filter(|l| !l.trim_start().starts_with("aborting due to"))
        // Drop pure warning lines that slipped in
        .filter(|l| !l.starts_with("warning"))
        .cloned()
        .collect::<Vec<_>>()
        .join("\n")
}

// ── Parser ───────────────────────────────────────────────────────────────────

fn parse_results(raw: &str) -> Results {
    let mut results: Results = HashMap::new();
    // (module, file, test) of the failure block we're currently inside
    let mut ctx: Option<(String, String, String)> = None;
    let mut expect_hint = false;

    for line in raw.lines() {
        // "test exercises::module::file::tests::name ... ok/FAILED"
        if let Some(rest) = line.strip_prefix("test ") {
            if let Some((path, status)) = rest.rsplit_once(" ... ") {
                let parts: Vec<&str> = path.split("::").collect();
                if parts.len() >= 5 && parts[0] == "exercises" {
                    let module = parts[1].to_string();
                    let file = parts[2].to_string();
                    let test = parts[parts.len() - 1].to_string();
                    let passed = status.trim() == "ok";
                    results
                        .entry(module)
                        .or_default()
                        .entry(file)
                        .or_default()
                        .insert(test, TestResult { passed, hint: None });
                }
            }
            continue;
        }

        // "---- exercises::module::file::tests::name stdout ----"
        if line.starts_with("---- ") && line.ends_with(" stdout ----") {
            let path = &line[5..line.len() - 12];
            let parts: Vec<&str> = path.split("::").collect();
            if parts.len() >= 5 && parts[0] == "exercises" {
                ctx = Some((
                    parts[1].to_string(),
                    parts[2].to_string(),
                    parts[parts.len() - 1].to_string(),
                ));
                expect_hint = false;
            }
            continue;
        }

        // "... panicked at src/...:line:col:" → next line has the message
        if line.contains("panicked at ") {
            expect_hint = true;
            continue;
        }

        // "not yet implemented: hint text"
        if expect_hint {
            expect_hint = false;
            if let Some(hint) = line.strip_prefix("not yet implemented: ") {
                if let Some((ref module, ref file, ref test)) = ctx {
                    if let Some(t) = results
                        .get_mut(module)
                        .and_then(|m| m.get_mut(file))
                        .and_then(|f| f.get_mut(test))
                    {
                        t.hint = Some(hint.trim().to_string());
                    }
                }
            }
        }
    }

    results
}

// ── Renderer ─────────────────────────────────────────────────────────────────

const BAR_WIDTH: usize = 20;
const RULE: &str = "──────────────────────────────────────────────────────────────";

fn clear_screen() {
    execute!(
        io::stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();
}

fn show_running() {
    clear_screen();
    println!(
        "{}",
        "┌──────────────────────────────────────────────────────────────┐".dark_grey()
    );
    println!(
        "{}",
        "│          Rust DSA Reviewer · Phase 1 Foundations            │".dark_grey()
    );
    println!(
        "{}",
        "└──────────────────────────────────────────────────────────────┘".dark_grey()
    );
    println!();
    println!("  {}", "Running tests...".dark_yellow());
    io::stdout().flush().unwrap();
}

// "test_build_vec" → "build_vec()"
fn format_fn_name(raw: &str) -> String {
    let name = raw.strip_prefix("test_").unwrap_or(raw);
    format!("{}()", name)
}

fn progress_bar(done: usize, total: usize) -> String {
    let filled = if total > 0 {
        done * BAR_WIDTH / total
    } else {
        0
    };
    format!("{}{}", "█".repeat(filled), "░".repeat(BAR_WIDTH - filled))
}

fn render(results: &Results, compile_error: Option<&str>) {
    clear_screen();
    let mut stdout = io::stdout();

    let total_files: usize = SECTIONS.iter().map(|(_, _, files)| files.len()).sum();
    let mut completed_files = 0usize;
    let mut current: Option<(&str, &str, &str)> = None; // (dir, module, file)

    // ── Header ──────────────────────────────────────────────────────────────
    println!(
        "{}",
        "┌──────────────────────────────────────────────────────────────┐".dark_grey()
    );
    println!(
        "{}",
        "│          Rust DSA Reviewer ·                                 │".dark_grey()
    );
    println!(
        "{}",
        "└──────────────────────────────────────────────────────────────┘".dark_grey()
    );
    println!();

    // ── Section progress ─────────────────────────────────────────────────────
    println!("  {}", "Progress".bold());
    println!("  {}", RULE.dark_grey());

    for &(dir, module, files) in SECTIONS {
        let module_map = results.get(module);

        let file_done: usize = files
            .iter()
            .filter(|&&f| {
                module_map
                    .and_then(|m| m.get(f))
                    .map(|tests| !tests.is_empty() && tests.values().all(|t| t.passed))
                    .unwrap_or(false)
            })
            .count();

        completed_files += file_done;

        // First failing file in this section
        if current.is_none() {
            for &file in files {
                let all_pass = module_map
                    .and_then(|m| m.get(file))
                    .map(|tests| !tests.is_empty() && tests.values().all(|t| t.passed))
                    .unwrap_or(false);
                if !all_pass {
                    current = Some((dir, module, file));
                    break;
                }
            }
        }

        let bar = progress_bar(file_done, files.len());
        let count = format!("{} / {}", file_done, files.len());
        let label = format!("{:<14}", dir);

        if file_done == files.len() {
            println!(
                "  {} {} {}",
                label.green().bold(),
                bar.green(),
                count.green()
            );
        } else if file_done > 0 {
            println!(
                "  {} {} {}",
                label.yellow().bold(),
                bar.yellow(),
                count.yellow()
            );
        } else {
            println!(
                "  {} {} {}",
                label.dark_grey(),
                bar.dark_grey(),
                count.dark_grey()
            );
        }
    }

    println!();
    println!(
        "  {}  {} / {} files complete",
        "Total:".bold(),
        completed_files.to_string().bold(),
        total_files
    );
    println!();

    // ── Current exercise or completion ───────────────────────────────────────
    if completed_files == total_files {
        println!("  {}", RULE.green());
        println!(
            "  {}",
            "  All 28 exercises complete! Phase 1 done.  "
                .black()
                .on_green()
                .bold()
        );
        println!("  {}", RULE.green());
    } else if let Some((dir, module, file)) = current {
        let heading = format!(" Current ──── {}::{} ", module, file);
        let pad = RULE.len().saturating_sub(heading.len());
        println!("  {}{}", heading.cyan().bold(), "─".repeat(pad).dark_grey());
        println!();
        println!(
            "  {}",
            format!("src/exercises/{}/{}.rs", dir, file).dark_grey()
        );
        println!();

        if let Some(test_map) = results.get(module).and_then(|m| m.get(file)) {
            let mut tests: Vec<(&String, &TestResult)> = test_map.iter().collect();
            tests.sort_by_key(|(name, _)| name.as_str());

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
                    let hint = result.hint.as_deref().unwrap_or("implement the todo!()");
                    println!(
                        "  {}  {:<32} {}",
                        "✗".red().bold(),
                        fn_name.red(),
                        hint.dark_grey()
                    );
                }
            }
            println!();
        }

        println!("  {}", RULE.dark_grey());
    }

    // ── Compile error panel ──────────────────────────────────────────────────
    if let Some(err) = compile_error {
        println!();
        println!(
            "  {} {}",
            "✗ Compilation Error".black().on_red().bold(),
            "─".repeat(40).red()
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

    println!();
    println!(
        "  {}",
        "Watching src/exercises/ · save a file to re-run · Ctrl+C to quit".dark_grey()
    );

    stdout.flush().unwrap();
}

// ── Entry point ──────────────────────────────────────────────────────────────

fn main() {
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx).expect("failed to create watcher");
    watcher
        .watch(Path::new("src/exercises"), RecursiveMode::Recursive)
        .expect("failed to watch src/exercises");

    // Initial run
    show_running();
    let mut last_results: Results = HashMap::new();
    match run_cargo_test() {
        Ok(raw) => {
            last_results = parse_results(&raw);
            render(&last_results, None);
        }
        Err(err) => render(&last_results, Some(&err)),
    }

    let mut pending = false;

    loop {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(Ok(_)) => {
                pending = true;
            }
            Ok(Err(e)) => eprintln!("watcher error: {e:?}"),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if pending {
                    // Debounce: drain remaining events then re-run
                    std::thread::sleep(Duration::from_millis(300));
                    while rx.try_recv().is_ok() {}

                    show_running();
                    match run_cargo_test() {
                        Ok(raw) => {
                            last_results = parse_results(&raw);
                            render(&last_results, None);
                        }
                        Err(err) => render(&last_results, Some(&err)),
                    }
                    pending = false;
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
}
