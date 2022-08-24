use crate::conventional_commit::lint::Level;
use std::cmp::Ordering;
use std::fmt;
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Violation {
    pub level: Level,
    pub message: String,
    pub description: Option<String>,
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}{}",
            self.level,
            self.message,
            self.description
                .as_ref()
                .map_or_else(String::new, |d| { format!("\n\t{}", d) })
        )
    }
}

impl Ord for Violation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.level.cmp(&other.level)
    }
}

impl PartialOrd for Violation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod format_test {

    use crate::conventional_commit::lint::{Level, Violation};

    #[test]
    fn test_format() {
        let lint = Violation {
            level: Level::Error,
            message: String::from("Something happened"),
            description: None,
        };

        assert_eq!(format!("{lint}"), "❌ Error: Something happened");
    }
    #[test]
    fn test_format_description() {
        let lint = Violation {
            level: Level::Error,
            message: String::from("Something happened"),
            description: Some(String::from(
                "This is an error and should not happen! Make sure you do it right next time",
            )),
        };

        assert_eq!(format!("{lint}"), "❌ Error: Something happened\n\tThis is an error and should not happen! Make sure you do it right next time");
    }
}
