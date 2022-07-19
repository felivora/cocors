use regex::Regex;
use std::fmt;

use super::commit::{CommitType, ConventionalCommit};

#[derive(Eq, PartialEq, Debug)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub build: Option<String>,
}

impl Version {
    /// Parses a string containing a semantic version, returns a Option<Version>
    ///
    /// The function takes the version string in the follwing format (int.int.int-string)
    /// with the components being [major.minor.patch-build] and parses them into the
    /// Version struct representing one semantic version.
    /// If major, minor or patch fields are not found the return is None, the build tag is optional
    ///
    /// # Arguments
    /// * `version` - A string slice that holds a version in the format numeric.numeric.numeric-string
    ///
    ///
    /// # Examples
    ///
    ///
    /// ```rust
    ///
    /// let version_correct = "1.2.3-build";
    /// assert!(parse_version(version).is_some())
    /// let version_incorrect = "2.3";
    /// assert!(parse_version(version_incorrect).is_none())
    ///
    /// ```
    pub fn parse(version: &str) -> Option<Version> {
        let version_regex = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-.+)?").unwrap();

        let caps_option = version_regex.captures(version);

        // return early if the regex did not find anything
        if caps_option.is_none() {
            return None;
        }

        let caps = caps_option.unwrap();

        let major = caps.get(1).map(|m| m.as_str());
        let minor = caps.get(2).map(|m| m.as_str());
        let patch = caps.get(3).map(|m| m.as_str());
        let build = caps.get(4).map(|m| {
            let mut build_string = m.as_str().to_owned();
            build_string.remove(0);
            build_string
        });

        // If one of the integral parts of the version is missing
        // return none here already
        if major.is_none() || minor.is_none() || patch.is_none() {
            return None;
        }

        let mut semver = Version {
            major: 0,
            minor: 0,
            patch: 0,
            build: build,
        };

        match major.unwrap().parse::<u64>() {
            Ok(n) => semver.major = n,
            Err(_) => return None,
        }

        match minor.unwrap().parse::<u64>() {
            Ok(n) => semver.minor = n,
            Err(_) => return None,
        }

        match patch.unwrap().parse::<u64>() {
            Ok(n) => semver.patch = n,
            Err(_) => return None,
        }

        Some(semver)
    }

    pub fn bump(&mut self, commit: &ConventionalCommit) {
        if commit.breaking {
            self.major += 1;
            return;
        }

        match commit.commit_type {
            CommitType::Fix => self.patch += 1,
            CommitType::Feature => {
                self.minor += 1;
                self.patch = 0;
            }
            CommitType::BreakingChange => {
                self.major += 1;
                self.minor = 0;
                self.patch = 0
            }
            _ => return,
        }

        self.build = None;
    }

    pub fn rollback(&mut self, last_commit: &ConventionalCommit) {
        if last_commit.breaking {
            self.major -= 1;
            return;
        }

        match last_commit.commit_type {
            CommitType::Fix => self.patch -= 1,
            CommitType::Feature => self.minor -= 1,
            CommitType::BreakingChange => self.major -= 1,
            _ => return,
        }
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.build.is_some() {
            write!(
                f,
                "{}.{}.{}-{}",
                self.major,
                self.minor,
                self.patch,
                self.build.as_ref().unwrap()
            )
        } else {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }
}
