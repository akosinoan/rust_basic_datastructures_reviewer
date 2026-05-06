use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TestResult {
    pub passed: bool,
    pub hint: Option<String>,
}

pub type FileResults = HashMap<String, TestResult>;
pub type ModuleResults = HashMap<String, FileResults>;
pub type Results = HashMap<String, ModuleResults>;

#[derive(Debug, Clone, Copy)]
pub enum ExerciseStatus {
    Complete,
    Partial,
    Pending,
}

impl ExerciseStatus {
    pub fn from_file(file_results: Option<&FileResults>) -> Self {
        match file_results {
            Some(t) if !t.is_empty() && t.values().all(|r| r.passed) => Self::Complete,
            Some(t) if !t.is_empty() => Self::Partial,
            _ => Self::Pending,
        }
    }

    pub fn from_counts(done: usize, total: usize) -> Self {
        if total > 0 && done == total {
            Self::Complete
        } else if done > 0 {
            Self::Partial
        } else {
            Self::Pending
        }
    }

    pub fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }
}
