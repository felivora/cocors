use crate::conventional_commit::lint::Level;
use std::cmp::Ordering;
use std::fmt;
#[derive(Debug, Clone)]
pub struct Lint {
    pub source: String,
    pub level: Level,
    pub location: u32,
    pub message: String,
    pub description: Option<String>,
}

impl Lint {
    pub fn get_source(raw_source: &str, location: usize) -> Option<String> {
        let start = if location < 10 { 0 } else { location - 1 };
        let end = if raw_source.len() - location < 10 {
            raw_source.len()
        } else {
            location + 10
        };

        match raw_source.get(start..end) {
            Some(v) => return Some(v.to_string()),
            None => return None,
        }
    }
}

impl fmt::Display for Lint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} on:\n", self.level)
    }
}

#[cfg(test)]
mod format_test {

    use crate::conventional_commit::lint::{Level, Lint};

    #[test]
    fn test_format() {
        let commit = r#"fix: prevent racing of requests

Introduce a request id and a reference to latest request. Dismiss
incoming responses other than from latest request.

Remove timeouts which were used to mitigate the racing issue but are
obsolete now.

Reviewed-by: Z
Refs: #123"#;

        let lint = Lint {
            source: commit.to_string(),
            level: Level::Error,
            location: 10,
            message: String::from("Something happened"),
            description: None,
        };

        assert_eq!(format!("{lint}"), "");
    }

    #[test]
    fn test_source() {
        let commit = "0123456i9876543210 1234567";

        let lint = Lint::get_source(commit, 7).unwrap();
        assert_eq!(format!("{lint}"), "0123456i9876543210");
    }
}
