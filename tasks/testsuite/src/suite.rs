use std::{
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
};

use crate::util::crate_root;

pub struct Case {
    pub path: PathBuf,
    pub code: String,
    pub should_fail: bool,
}

impl Case {
    pub fn relative_path(&self) -> &Path {
        self.path.strip_prefix(crate_root()).unwrap()
    }
}

pub enum TestResult {
    Passed { path: PathBuf },
    Failed { path: PathBuf, error: String },
    Ignored { path: PathBuf },
}

impl Display for TestResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TestResult::Passed { path } => write!(f, "Passed: {}", path.display()),
            TestResult::Failed { path, error } => {
                write!(f, "Failed: {} - {}", path.display(), error)
            }
            TestResult::Ignored { path } => write!(f, "Ignored: {}", path.display()),
        }
    }
}
