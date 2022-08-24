use super::Violation;
use crate::Commit;
use std::fmt;
pub struct LintResult {
    pub commit: Option<Commit>,
    pub lints: Vec<Violation>,
}
