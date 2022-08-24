use super::lint::{Level, LintResult, Violation};
use super::CommitType;
use crate::Version;
use lazy_static::lazy_static;
use log::{debug, error};
use regex::Regex;
use std::collections::HashMap;

type CommitBody = (Option<String>, Option<HashMap<String, String>>);

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
    pub body: Option<String>,
    pub footer: Option<HashMap<String, String>>,
}

impl Commit {
    /// Takes a commit string and parses it according to the conventional commit
    /// specification.
    ///
    /// The function uses regex to extract all relevant tags according to the
    /// specification, if the commit message does not conform `None` will
    /// be returned.
    pub fn parse(commit: &str) -> Option<Commit> {
        todo!()
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

    pub fn lint(commit: &str) -> LintResult {
        let mut res = LintResult {
            commit: None,
            lints: Vec::<Violation>::new(),
        };

        lazy_static! {
            static ref COMMIT_RE: Regex =
                Regex::new(r"([a-z,A-Z]+)?(\((.+)?\))?(!)?(?>: )(.+)?(\n\n(?:.|\n)*)?").unwrap();
        }
        let caps_option = COMMIT_RE.captures(commit);

        // return early if the regex did not find anything
        // TODO: Add specific log message for each failure point for
        //      later usage in linter
        if caps_option.is_none() {
            res.lints.push(Violation {
                level: Level::Error,
                message: String::from("The format of the commit message is not conformant to conventional commit specification"),
                description: Some(String::from(
                    r#"Make sure that the specification follows the specification:<type>[optional scope]: <description>

                    [optional body]

                    [optional footer(s)]"#))
            });
            return res;
        }

        let caps = caps_option.unwrap();

        let commit_type = get_commit_type(&mut res, &caps);

        let scope = get_commit_scope(&mut res, &caps);

        let header = get_commit_header(&mut res, &caps);

        let body = get_commit_body_footer(&mut res, &caps);
        let description = body.0;
        let footer = body.1;

        res.lints.sort_unstable();

        if !res.lints.iter().any(|l| l.level == Level::Error)
            && commit_type.is_some()
            && header.is_some()
        {
            let header_unwrapped = header.unwrap();
            let commit_type_unwrapped = commit_type.unwrap();

            res.commit = Some(Commit {
                breaking: caps.get(4).is_some()
                    || commit_type_unwrapped == CommitType::BreakingChange,
                commit_type: commit_type_unwrapped,
                scope,
                description: header_unwrapped,
                body: description,
                footer: footer,
            })
        }

        return res;
    }
}

fn get_commit_type(result: &mut LintResult, caps: &regex::Captures) -> Option<CommitType> {
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
            _ => Some(CommitType::Other),
        },
    };

    if commit.is_none() {
        result.lints.push(Violation {
                level: Level::Error,
                message: String::from("Mandatory commit type is missing"),
                description: Some(String::from("Make sure you provide a commit type that describes of what type the change is (e.g. fix, feat, BREAKING CHANGE). Type must be ascii letters only"))
            });
    };

    return commit;
}

fn get_commit_scope(result: &mut LintResult, caps: &regex::Captures) -> Option<String> {
    match caps.get(2) {
        None => {
            result.lints.push(Violation {
                level: Level::Suggestion,
                message: String::from("Optional scope is missing"),
                description: Some(String::from("Consider adding a scope to the commit message to specify where the changes have been made"))
            });
            None
        }
        Some(s) => {
            if caps.get(3).is_none() {
                result.lints.push(Violation {
                level: Level::Error,
                message: String::from("Scope is empty"),
                description: Some(String::from("Scope is an optional parameter, but if not given the parenthesis must be removed"))
                });
                None
            } else {
                Some(caps.get(3).unwrap().as_str().to_string())
            }
        }
    }
}

fn get_commit_header(result: &mut LintResult, caps: &regex::Captures) -> Option<String> {
    match caps.get(5) {
        None => {
            result.lints.push(Violation {
            level: Level::Error,
            message: String::from("Mandatory description is missing"),
            description: Some(String::from("The short description of the commit is missing; this is mandatory field and must be provided")),
        });
            None
        }
        Some(d) => Some(d.as_str().to_string()),
    }
}

fn get_commit_body_footer(result: &mut LintResult, caps: &regex::Captures) -> CommitBody {
    let mut res: CommitBody = (None, None);

    if caps.get(6).is_none() {
        return res;
    }
    let mut body = match caps.get(6) {
        None => return res,
        Some(m) => m.as_str().to_string(),
    };

    body.trim();

    lazy_static! {
        static ref BODY_FOOTER_RE: Regex = Regex::new(r"(.*)(?>: )(.*)").unwrap();
    }

    let mut footer = HashMap::<String, String>::new();

    let start = match BODY_FOOTER_RE.find(&body) {
        None => {
            res.0 = Some(body);
            result.lints.push(Violation {
                level: Level::Info,
                message: String::from("No footer found"),
                description: None,
            });
            return res;
        }
        Some(m) => m.start(),
    };

    for cap in BODY_FOOTER_RE.captures_iter(&body) {
        footer.insert(
            cap.get(1)
                .map_or_else(String::new, |k| k.as_str().to_string()),
            cap.get(2)
                .map_or_else(String::new, |v| v.as_str().to_string()),
        );
    }

    res.0 = Some(body.split_at(start).0.to_string());

    return res;
}

#[cfg(test)]
mod tests {

    use crate::{Commit, CommitType};

    #[test]
    fn commit_type_valid() {
        let commit_string = "feat: allow provided config object to extend other configs";

        assert_eq!();
    }
}
