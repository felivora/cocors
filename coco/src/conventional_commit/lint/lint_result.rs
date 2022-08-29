use super::Violation;
use crate::Commit;
pub struct LintResult {
    pub commit: Option<Commit>,
    pub lints: Vec<Violation>,
}
