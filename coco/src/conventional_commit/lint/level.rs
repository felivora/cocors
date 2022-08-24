use std::fmt;
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone)]
pub enum Level {
    Error,
    Warning,
    Info,
    Suggestion,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Level::Error => write!(f, "❌ Error"),
            Level::Warning => write!(f, "⚠️ Warning"),
            Level::Info => write!(f, "ℹ️ Info"),
            Level::Suggestion => write!(f, "💡 Suggestion"),
        }
    }
}
