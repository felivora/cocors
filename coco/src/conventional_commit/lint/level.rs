use clap::ValueEnum;
use std::fmt;
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy, ValueEnum)]
pub enum Level {
    Error,
    Warning,
    Info,
    Suggestion,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Level::Error => write!(f, "❌ "),
            Level::Warning => write!(f, "⚠️ "),
            Level::Info => write!(f, "ℹ️ "),
            Level::Suggestion => write!(f, "💡 "),
        }
    }
}
