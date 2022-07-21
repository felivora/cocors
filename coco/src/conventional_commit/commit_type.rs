#[derive(Eq, PartialEq, Debug)]
pub enum CommitType {
    Fix,
    Feature,
    BreakingChange,
    Build,
    Chore,
    Ci,
    Docs,
    Style,
    Refactor,
    Performance,
    Test,
}
