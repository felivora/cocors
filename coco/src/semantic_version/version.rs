#![warn(missing_docs)]

use regex::Regex;
use std::fmt;
use std::cmp::Ordering;

use crate::{CommitType, ConventionalCommit};

/// A representation of a [semantic version](https://semver.org/) with convenience functions
///
/// It represents multiple 'levels' of version differences and compatibility between versions (`major`.`minor`.`patch`-`pre_release`+`metadata`):
/// - major:
#[derive(Eq, PartialEq, Debug, Clone)]
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
            Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[0-9A-Za-z-\.]+)?(\+[0-9A-Za-z-\.]+)?").unwrap();

        let caps_option = version_regex.captures(version);

        // return early if the regex did not find anything
        caps_option.as_ref()?;
        
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
        if major.is_none() || minor.is_none() || patch.is_none() {
            return None;
        }

        let mut semver = Version {
            major: 0,
            minor: 0,
            patch: 0,
            pre_release,
            metadata,
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

    /// Increments the version according to the [conventional commit specification](https://www.conventionalcommits.org/en/v1.0.0/#specification)
    ///
    /// Using a human readable format
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

        self.pre_release = None;
        self.metadata = None;
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
            _ => (),
        }
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
                .map_or_else(String::new, |p| { format!("-{}", p) }),
            self.metadata
                .as_ref()
                .map_or_else(String::new, |p| { format!("+{}", p) })
        )
    }
}

impl Ord for Version{
    fn cmp(&self, other: &Self) -> Ordering {
        
        if self.major != other.major {
            return self.major.cmp(&other.major)
        }

        if self.minor != other.minor {
            return self.minor.cmp(&other.minor)
        }

        if self.patch != other.patch {
            return self.patch.cmp(&other.patch)
        }
        
        if !(self.pre_release.is_some() && other.pre_release.is_some()){
           return self.pre_release.cmp(&other.pre_release).reverse()
        }
       

        // Pre Release precedence is calculated as follows:
        // 1.0.0-alpha < 1.0.0-alpha.1 < 1.0.0-alpha.beta < 1.0.0-beta < 1.0.0-beta.2 <
        // 1.0.0-beta.11 < 1.0.0-rc.1 < 1.0.0
        // Option was checked above (returned only if not both are some)
        // so unwrapping here is fine 
        let self_prerelease_clone = self.pre_release.clone().unwrap();
        let mut self_prerelease = self_prerelease_clone.split('.');
        let other_prerelease_clone = other.pre_release.clone().unwrap();
        let mut other_prerelease = other_prerelease_clone.split('.');
      
        loop {
           let s = self_prerelease.next();
           let o = other_prerelease.next();
           
           if s.is_none() && o.is_none(){
                break;
           }; 

           if s.is_none() {
               return Ordering::Less;
           }

           if o.is_none() {
               return Ordering::Greater;
           }

           if s.cmp(&o) != Ordering::Equal{
               return s.cmp(&o)
           }
        }
       
        Ordering::Equal
        
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[cfg(test)]
mod ordering_test {

    use crate::Version;
    use std::cmp::Ordering;
    
    #[test]
    fn major_version_diff() {
        let version_greater = Version::parse("2.2.3").unwrap();
        let version_less = Version::parse("1.2.3").unwrap();

        assert_eq!(version_greater.cmp(&version_less), Ordering::Greater);
    }
   
    #[test]
    fn minor_version_diff() {
        let version_greater = Version::parse("1.3.3").unwrap();
        let version_less = Version::parse("1.2.3").unwrap();

        assert_eq!(version_greater.cmp(&version_less), Ordering::Greater);
    }
 
    #[test]
    fn patch_version_diff() {
        let version_greater = Version::parse("1.2.4").unwrap();
        let version_less = Version::parse("1.2.3").unwrap();

        assert_eq!(version_greater.cmp(&version_less), Ordering::Greater);
    }

    #[test]
    fn one_with_pre_release_diff() {
        let version_greater = Version::parse("1.2.3").unwrap();
        let version_less = Version::parse("1.2.3-alpha").unwrap();

        assert_eq!(version_greater.cmp(&version_less), Ordering::Greater);
    }

    #[test]
    fn pre_release_equal() {
        let version_greater = Version::parse("1.2.3-alpha").unwrap();
        let version_less = Version::parse("1.2.3-alpha").unwrap();

        assert_eq!(version_greater.cmp(&version_less), Ordering::Equal);
    }
    
    #[test]
    fn pre_release_same_amount_alpha_diff() {
        let version_greater = Version::parse("1.2.3-beta").unwrap();
        let version_less = Version::parse("1.2.3-alpha").unwrap();

        assert_eq!(version_greater.cmp(&version_less), Ordering::Greater);
    }
    
    #[test]
    fn pre_release_same_amount_numeric_diff() {
        let version_greater = Version::parse("1.2.3-alpha.beta").unwrap();
        let version_less = Version::parse("1.2.3-alpha.1111").unwrap();

        assert_eq!(version_greater.cmp(&version_less), Ordering::Greater);
    }

    #[test]
    fn pre_release_different_amount() {
        let version_greater = Version::parse("1.2.3-alpha.1").unwrap();
        let version_less = Version::parse("1.2.3-alpha").unwrap();

        assert_eq!(version_greater.cmp(&version_less), Ordering::Greater);
    }

    #[test]
    fn pre_release_difference_exists() {
        let version_greater = Version::parse("1.2.3").unwrap();
        let version_less = Version::parse("1.2.3-alpha.1").unwrap();

        assert_eq!(version_greater.cmp(&version_less), Ordering::Greater);
    }

    #[test]
    fn pre_release_sanity_check() {
        let version_greater = Version::parse("1.2.3-beta.11").unwrap();
        let version_less = Version::parse("1.2.3-alpha.1").unwrap();

        assert_eq!(version_greater.cmp(&version_less), Ordering::Greater);
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
            pre_release: Some(String::from("alpha.beta.3")),
            metadata: Some(String::from("d408340")),
        };
        assert_eq!(format!("{version}"), "1.2.3-alpha.beta.3+d408340");
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
            pre_release: Some(String::from("alpha.beta.3")),
            metadata: None,
        };

        assert_eq!(Version::parse("1.2.3-alpha.beta.3"), Some(version));
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
