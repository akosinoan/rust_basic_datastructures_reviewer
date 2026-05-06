use std::collections::HashMap;

use super::results::{Results, TestResult};

const EXERCISES_ROOT: &str = "exercises";
const HINT_PREFIX: &str = "not yet implemented: ";
const PANIC_MARKER: &str = "panicked at ";

#[derive(Debug, Clone)]
struct TestPath {
    module: String,
    file: String,
    test: String,
}

fn from_path_parts(parts: &[&str]) -> Option<TestPath> {
    (parts.len() >= 5 && parts[0] == EXERCISES_ROOT).then(|| TestPath {
        module: parts[1].to_string(),
        file: parts[2].to_string(),
        test: parts[parts.len() - 1].to_string(),
    })
}

// "test exercises::module::file::tests::name ... ok"
fn parse_test_line(line: &str) -> Option<(TestPath, bool)> {
    let rest = line.strip_prefix("test ")?;
    let (path, status) = rest.rsplit_once(" ... ")?;
    let parts: Vec<&str> = path.split("::").collect();
    let test_path = from_path_parts(&parts)?;
    Some((test_path, status.trim() == "ok"))
}

// "---- exercises::module::file::tests::name stdout ----"
fn parse_stdout_header(line: &str) -> Option<TestPath> {
    let inner = line.strip_prefix("---- ")?.strip_suffix(" stdout ----")?;
    let parts: Vec<&str> = inner.split("::").collect();
    from_path_parts(&parts)
}

pub fn parse_results(raw: &str) -> Results {
    let mut results: Results = HashMap::new();
    let mut current: Option<TestPath> = None;
    let mut expect_hint = false;

    for line in raw.lines() {
        if let Some((path, passed)) = parse_test_line(line) {
            results
                .entry(path.module)
                .or_default()
                .entry(path.file)
                .or_default()
                .insert(path.test, TestResult { passed, hint: None });
            continue;
        }

        if let Some(path) = parse_stdout_header(line) {
            current = Some(path);
            expect_hint = false;
            continue;
        }

        if line.contains(PANIC_MARKER) {
            expect_hint = true;
            continue;
        }

        if expect_hint {
            expect_hint = false;
            apply_hint(&mut results, current.as_ref(), line);
        }
    }

    results
}

fn apply_hint(results: &mut Results, ctx: Option<&TestPath>, line: &str) {
    let Some(ctx) = ctx else { return };
    let Some(hint) = line.strip_prefix(HINT_PREFIX) else {
        return;
    };
    if let Some(t) = results
        .get_mut(&ctx.module)
        .and_then(|m| m.get_mut(&ctx.file))
        .and_then(|f| f.get_mut(&ctx.test))
    {
        t.hint = Some(hint.trim().to_string());
    }
}
