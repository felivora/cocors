use super::CommitType;
use crate::Version;
use lazy_static::lazy_static;
use regex::Regex;
use log::{info, error, warn, debug};

/// Represents a commit message according to the
/// [conventional commit specification](https://www.conventionalcommits.org/en/v1.0.0/#specification)
///
/// Can be used to bump/ rollback a semantic version and to generate changelogs.
#[derive(Eq, PartialEq, Debug, Default)]
pub struct Commit {
    /// Defines if the changes in the commit are breaking the backwards compatibility
    /// of the Public API (annotated by `!` after the type or BREAKING CHANGE type)
    pub breaking: bool,
    /// Type of the changes made in the commit that are used to bump the version
    /// (e.g. `fix`, `feat` or `BREAKING CHANGE`) or types that do not affect the versioning (e.g. `docs`, `chore`)
    pub commit_type: CommitType,
    /// Optional scope, that defines where the changes in the code happened (e.g. parser)
    pub scope: Option<String>,
    /// A short string summarizing the changes in the commit
    pub description: String,
    /// Stores the string of the type if the type is [CommitType::Other] as otherwise there
    /// would be no way to get the type for changelog
    raw_type: Option<String>,
}

impl Commit {
    /// Takes a commit string and parses it according to the conventional commit
    /// specification.
    ///
    /// The function uses regex to extract all relevant tags according to the
    /// specification, if the commit message does not conform `None` will
    /// be returned.
    pub fn parse(commit: &str) -> Option<Commit> {
        lazy_static! {
            static ref COMMIT_RE: Regex =
                Regex::new(r"([a-z,A-Z]+)?(\(([a-z,A-Z]+)\))?(!)?: (.+)(\n\n(.|\n)*)?").unwrap();
        }
        let caps_option = COMMIT_RE.captures(commit);

        this_target = "commit_parser";

        debug!(target: this_target, "Starting the commit parsing")

        // return early if the regex did not find anything
        // TODO: Add specific log message for each failure point for
        //      later usage in linter
        if caps_option.is_none() {
            error!(target: this_target, "Commit message could not be tokenized, make sure that mandatory fields type and description are included in the message. 
            Format: type: description")
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
                "other" => Some(CommitType::Other),
            },
        };

        if commit.is_none() {
            eprintln!("The commit type could not be parsed into the specified types, make sure that the message is annotated with the types");
            return None;
        };
        let commit_type = commit.unwrap();

        if caps.get(5).is_none {
            eprintln!
        }

        Some(Commit {
            breaking: caps.get(4).is_some() || commit_type == CommitType::BreakingChange,
            commit_type: commit_type,
            scope: caps.get(3).map(|m| m.as_str().to_owned()),
            description: caps.get(5).map(|m| m.as_str().to_owned()),
        })
    }

    /// Bumps the given version according to the commit message
    pub fn bump(&self, version: &mut Version) {
        if self.breaking {
            let major = version.major + 1;

            version.reset();
            version.major = major;
            return;
        }
        match self.commit_type {
            CommitType::Fix => version.patch += 1,
            CommitType::Feature => {
                version.minor += 1;
                version.patch = 0;
            }
            CommitType::BreakingChange => {
                version.major += 1;
                version.minor = 0;
                version.patch = 0
            }
            _ => return,
        }

        version.pre_release = None;
        version.metadata = None;
    }
}

#[cfg(test)]
mod tests {

    use crate::{Commit, CommitType};

    #[test]
    fn parse_correct_commit() {
        let commit_string = "feat: allow provided config object to extend other configs";

        let commit = Commit {
            breaking: false,
            commit_type: CommitType::Feature,
            scope: None,
            description: String::from("allow provided config object to extend other configs"),
        };

        assert_eq!(Commit::parse(commit_string).unwrap(), commit);
    }
}
