use crate::conventional_commit::lint::Level;

use std::fmt;
#[derive(Debug, Clone)]
pub struct Violation {
    pub source: String,
    pub level: Level,
    pub location: u32,
    pub message: String,
    pub description: Option<String>,
}

impl Violation {
    pub fn get_source(raw_source: &str, location: usize) -> Option<String> {
        let start = if location < 10 { 0 } else { location - 1 };
        let end = if raw_source.len() - location < 10 {
            raw_source.len()
        } else {
            location + 10
        };

        raw_source.get(start..end).map(|v| v.to_string())
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:?} on:", self.level)
    }
}

#[cfg(test)]
mod format_test {

    use crate::conventional_commit::lint::{Level, Violation};

    #[test]
    fn test_format() {
        let commit = r#"fix: prevent racing of requests

Introduce a request id and a reference to latest request. Dismiss
incoming responses other than from latest request.

Remove timeouts which were used to mitigate the racing issue but are
obsolete now.

Reviewed-by: Z
Refs: #123"#;

        let lint = Violation {
            source: commit.to_string(),
            level: Level::Error,
            location: 10,
            message: String::from("Something happened"),
            description: None,
        };

        assert_eq!(format!("{lint}"), "Error on:\n");
    }

    #[test]
    fn test_source() {
        let commit = "0123456i9876543210 1234567";

        let lint = Violation::get_source(commit, 7).unwrap();
        assert_eq!(format!("{lint}"), "0123456i987654321");
    }
}
