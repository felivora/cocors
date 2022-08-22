#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone)]
pub enum Level {
    Error,
    Warning,
    Info,
    Suggestion,
}
