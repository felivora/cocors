use super::CommitType;
use regex::Regex;

#[derive(Eq, PartialEq, Debug)]
pub struct ConventionalCommit {
    pub breaking: bool,
    pub commit_type: CommitType,
    pub scope: Option<String>,
    pub description: Option<String>,
}

impl ConventionalCommit {
    pub fn new(commit: &str) -> Option<ConventionalCommit> {
        let regex = Regex::new(r"([a-z,A-Z]+)(\(([a-z,A-Z]+)\))?(!)?: (.+)?").unwrap();
        let caps_option = regex.captures(commit);

        // return early if the regex did not find anything
        if caps_option.is_none() {
            eprintln!("The commit message could not be tokenized, make sure that the message conforms to the conventional commit specification");
            return None;
        }

        let caps = caps_option.unwrap();

        let commit: Option<CommitType> = match caps.get(1) {
            None => None,
            Some(t) => match t.as_str().to_lowercase().as_str() {
                "fix" => Some(CommitType::Fix),
                "feat" => Some(CommitType::Feature),
                "breaking change" => Some(CommitType::BreakingChange),
                "build" => Some(CommitType::Build),
                "chore" => Some(CommitType::Chore),
                "style" => Some(CommitType::Style),
                "docs" => Some(CommitType::Docs),
                "refactor" => Some(CommitType::Refactor),
                "perf" => Some(CommitType::Performance),
                "test" => Some(CommitType::Test),
                "ci" => Some(CommitType::Ci),
                _ => None,
            },
        };

        if commit.is_none() {
            eprintln!("The commit type could not be parsed into the specified types, make sure that the message is annotated with the types");
            return None;
        };
        let commit_type = commit.unwrap();
        Some(ConventionalCommit {
            breaking: caps.get(4).is_some() || commit_type == CommitType::BreakingChange,
            commit_type: commit_type,
            scope: caps.get(3).map(|m| m.as_str().to_owned()),
            description: caps.get(5).map(|m| m.as_str().to_owned()),
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::{CommitType, ConventionalCommit};

    #[test]
    fn parse_correct_commit() {
        let commit_string = "feat: allow provided config object to extend other configs";

        let commit = ConventionalCommit {
            breaking: false,
            commit_type: CommitType::Feature,
            scope: None,
            description: Some(String::from(
                "allow provided config object to extend other configs",
            )),
        };

        assert_eq!(ConventionalCommit::new(commit_string).unwrap(), commit);
    }
}
