#![warn(missing_docs)]

use regex::Regex;
use std::fmt;

use crate::{Commit, CommitType};

/// A representation of a [semantic version](https://semver.org/) with convenience functions
///
/// It represents multiple 'levels' of version differences and compatibility between versions (`major`.`minor`.`patch`-`pre_release`+`metadata`):
/// - major:
#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub struct Version {
    /// Represents breaking change in the public API, every bump in this version will reset the minor and patch fields
    /// to 0. When starting with development and while the public API is still considered unstable the major version
    /// should remain 0.
    pub major: u64,
    /// Must be incremented with the  introduction of a new, backwards compatible feature that is introduced
    /// in the Public API; it may be incremented with significant changes in the private code.
    /// Whenever it is incremented the patch version is reset to 0
    pub minor: u64,
    /// Patch will be incremented with the introduction of a bug fix, that is backwards compatible.
    pub patch: u64,
    /// The pre release tag indicates a version that is considered unstable and might not be usable with
    /// public API; the tag is optional and can be left empty. Tags must not be empty and must not start
    /// with 0
    pub pre_release: Option<String>,
    /// The metadata information helps in determining the exact build version in larger pipelines,
    /// where only small changes are included. With the metadata the exact build can be identified,
    /// e.g. by appending the commit hash. The metadata does not impact the precedence.
    pub metadata: Option<String>,
}

impl Version {
    /// Convenience function that simply resets every field to its default value
    ///
    /// Can be used when incrementing higher version bumps, when all other fields
    /// need to be reset.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use coco::Version;
    /// let mut version = Version::parse("1.2.3-alpha+d408340").unwrap();
    /// let empty = Version::default();
    /// let initial = Version::parse("0.0.0").unwrap();
    ///
    /// version.reset();
    ///
    /// assert_eq!(version, empty);
    /// assert_eq!(version, initial);
    /// ```
    pub fn reset(&mut self) {
        self.major = 0;
        self.minor = 0;
        self.patch = 0;
        self.pre_release = None;
        self.metadata = None;
    }

    /// Parses a string containing a semantic version, returns a Option<Version>
    ///
    /// The function takes the version string in the follwing format (int.int.int-string)
    /// with the components being [major.minor.patch-build] and parses them into the
    /// Version struct representing one semantic version. As defined in the specification all fields
    /// non optional fields MUST be present.
    /// If major, minor or patch fields are not found the return is `None`, the build tag is optional
    /// When a less strict parsing is required, use the function [`Version::parse_unchecked()`].
    ///
    /// `version` - A string slice that holds a version in the format numeric.numeric.numeric-string
    ///
    ///
    /// # Examples
    ///
    ///
    /// ```rust
    /// # use coco::Version;
    /// let version_correct = "1.2.3";
    /// assert!(Version::parse(version_correct).is_some());
    ///
    /// let with_optional = "1.2.3-build";
    /// assert!(Version::parse(with_optional).is_some());
    ///
    /// let version = Version {
    ///     major: 1,
    ///     minor: 2,
    ///     patch: 3,
    ///     pre_release: Some(String::from("build")),
    ///     metadata: None
    /// };
    ///
    /// let version_incorrect = "2.3";
    /// assert!(Version::parse(version_incorrect).is_none());
    /// ```
    pub fn parse(version: &str) -> Option<Version> {
        let version_regex =
            Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[0-9A-Za-z-]+)?(\+[0-9A-Za-z-]+)?").unwrap();

        let caps_option = version_regex.captures(version);

        // return early if the regex did not find anything
        if caps_option.is_none() {
            return None;
        }

        let caps = caps_option.unwrap();

        let major = caps.get(1).map(|m| m.as_str());
        let minor = caps.get(2).map(|m| m.as_str());
        let patch = caps.get(3).map(|m| m.as_str());
        let pre_release = caps.get(4).map(|m| {
            let mut pre_release_string = m.as_str().to_owned();
            pre_release_string.remove(0);
            pre_release_string
        });
        let metadata = caps.get(5).map(|m| {
            let mut metadata_string = m.as_str().to_owned();
            metadata_string.remove(0);
            metadata_string
        });

        // If one of the integral parts of the version is missing
        // return none here already
        // TODO: Set a specific log message for each point of failure?
        if major.is_none() || minor.is_none() || patch.is_none() {
            return None;
        }

        let mut semver = Version {
            major: 0,
            minor: 0,
            patch: 0,
            pre_release: pre_release,
            metadata: metadata,
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
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}{}{}",
            self.major,
            self.minor,
            self.patch,
            self.pre_release
                .as_ref()
                .map_or_else(|| String::new(), |p| { format!("-{}", p) }),
            self.metadata
                .as_ref()
                .map_or_else(|| String::new(), |p| { format!("+{}", p) })
        )
    }
}

#[cfg(test)]
mod format_test {

    use crate::Version;

    #[test]
    fn with_optional_pre_release_label() {
        let version = Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: Some(String::from("alpha")),
            metadata: None,
        };

        assert_eq!(format!("{version}"), "1.2.3-alpha");
    }

    #[test]
    fn with_optional_pre_release_and_metadata_label() {
        let version = Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: Some(String::from("alpha")),
            metadata: Some(String::from("d408340")),
        };
        assert_eq!(format!("{version}"), "1.2.3-alpha+d408340");
    }

    #[test]
    fn with_optional_metadata_label() {
        let version = Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: None,
            metadata: Some(String::from("d408340")),
        };
        assert_eq!(format!("{version}"), "1.2.3+d408340");
    }

    #[test]
    fn without_optional() {
        let version = Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: None,
            metadata: None,
        };
        assert_eq!(format!("{version}"), "1.2.3");
    }
}

#[cfg(test)]
mod parse_test {

    use crate::Version;

    #[test]
    fn with_optional_pre_release_label() {
        let version = Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: Some(String::from("alpha")),
            metadata: None,
        };

        assert_eq!(Version::parse("1.2.3-alpha"), Some(version));
    }

    #[test]
    fn with_optional_pre_release_and_metadata_label() {
        let version = Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: Some(String::from("alpha")),
            metadata: Some(String::from("d408340")),
        };
        assert_eq!(Version::parse("1.2.3-alpha+d408340"), Some(version));
    }

    #[test]
    fn with_optional_metadata_label() {
        let version = Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: None,
            metadata: Some(String::from("d408340")),
        };
        assert_eq!(Version::parse("1.2.3+d408340"), Some(version));
    }

    #[test]
    fn without_optional() {
        let version = Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre_release: None,
            metadata: None,
        };
        assert_eq!(format!("{version}"), "1.2.3");
    }
}
