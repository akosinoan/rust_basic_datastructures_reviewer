use std::io;
use std::process::Command;

pub enum CargoOutcome {
    Tests(String),
    CompileError(String),
}

pub fn run_cargo_test() -> io::Result<CargoOutcome> {
    let output = Command::new("cargo")
        .args(["test"])
        .env("RUSTFLAGS", "-Awarnings")
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if stderr.contains("error[") || stderr.contains("error: could not compile") {
        return Ok(CargoOutcome::CompileError(extract_compile_error(&stderr)));
    }

    let mut combined = String::with_capacity(stdout.len() + stderr.len());
    combined.push_str(&stdout);
    combined.push_str(&stderr);
    Ok(CargoOutcome::Tests(combined))
}

fn extract_compile_error(stderr: &str) -> String {
    let lines: Vec<&str> = stderr.lines().collect();

    let start = lines
        .iter()
        .position(|l| l.starts_with("error[") || l.starts_with("error: "))
        .unwrap_or(0);

    let end = lines
        .iter()
        .rposition(|l| l.contains("could not compile"))
        .map(|i| i + 1)
        .unwrap_or(lines.len());

    lines[start..end.min(lines.len())]
        .iter()
        .filter(|l| !l.trim_start().starts_with("aborting due to"))
        .filter(|l| !l.starts_with("warning"))
        .copied()
        .collect::<Vec<_>>()
        .join("\n")
}
