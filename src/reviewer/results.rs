use std::collections::HashMap;

use super::catalog::{SECTIONS, Section};

#[derive(Debug, Clone)]
pub struct TestResult {
    pub passed: bool,
    pub hint: Option<String>,
}

pub type FileResults = HashMap<String, TestResult>;
pub type ModuleResults = HashMap<String, FileResults>;
pub type Results = HashMap<String, ModuleResults>;

pub fn current_exercise(results: &Results) -> Option<(&'static Section, &'static str)> {
    for section in SECTIONS {
        let module_results = results.get(section.module);
        if let Some(file) = section.exercises.iter().find(|file| {
            !ExerciseStatus::from_file(module_results.and_then(|m| m.get(**file))).is_complete()
        }) {
            return Some((section, *file));
        }
    }
    None
}

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
