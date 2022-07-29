use dunce;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::Lines;

use super::{git, utility};

pub struct Repository {
    path: String,
}

impl Repository {
    /// Returns a new Repository with an associated root path from a path, that can be anywhere in the repo hierarchy
    ///
    /// If the provided path is not in a repository, there is no access to the path or git is not installed
    /// an [io::Error] will be returned detailling the issue
    pub fn new(path: &Path) -> Result<Repository, io::Error> {
        let mut s = Self::repo_root(path)?;

        while s.ends_with('\n') || s.ends_with('\r') {
            s.pop();
        }
        Ok(Repository { path: s })
    }

    pub fn log(&self, from: &str, to: &str, format: &str) -> io::Result<String> {
        let mut cmd = Command::new("git");

        let mut range = from.to_string();

        if !to.is_empty() {
            range.push_str(format!("..{}", to).as_str());
        }

        cmd.arg("log");

        if !range.is_empty() {
            println!("{}", &range);
            cmd.arg(range);
        }

        if !format.is_empty() {
            cmd.arg(format!("--format={}", format));
        }
        cmd.current_dir(&self.path);
        let output = cmd.output()?;

        if output.status.success() {
            return Ok(String::from_utf8_lossy(&output.stdout).into_owned());
        }
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "git log command failed with error; {}",
                String::from_utf8_lossy(&output.stderr).into_owned()
            ),
        ));
    }

    /// Queries all tags in the repository and returns them sorted in alphanumerical order [Ord for str](https://doc.rust-lang.org/std/cmp/trait.Ord.html#impl-Ord-15)
    ///
    /// Fails if the path in the Repository is not actually a repository
    pub fn tags(&self) -> io::Result<Vec<String>> {
        let mut cmd = Command::new("git");

        cmd.arg("tag");
        cmd.current_dir(&self.path);

        let output = cmd.output()?;

        if output.status.success() {
            let tags_raw = String::from_utf8_lossy(&output.stdout).into_owned();
            if tags_raw.is_empty() {
                return Ok(Vec::<String>::new());
            }
            let mut tags = tags_raw
                .split_whitespace()
                .map(|t| String::from(t.trim_end()))
                .collect::<Vec<String>>();

            tags.sort_unstable();
            return Ok(tags);
        }
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "git log command failed with error; {}",
                String::from_utf8_lossy(&output.stderr).into_owned()
            ),
        ));
    }

    pub fn latest_tag(&self) -> io::Result<String> {
        let mut cmd = Command::new("git");

        cmd.arg("describe");
        cmd.current_dir(&self.path);

        let output = cmd.output()?;

        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout).into_owned();

            return Ok(s.trim_end().to_string());
        }
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "git log command failed with error; {}",
                String::from_utf8_lossy(&output.stderr).into_owned()
            ),
        ));
    }

    /// Checks wether or not the given path (file or directory) is in a repository
    ///
    /// The path can be either a path to a file or a directory and does not need to be
    /// the repo root path, any path descendant of a repository will return true
    pub fn is_repository(path: &Path) -> Result<bool, io::Error> {
        if !git::is_installed() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "The git command could not be found, make sure git is installed and can be found by the system",
            ));
        }

        let cannon_path = utility::normalize_pathname(path)?;
        let mut cmd = Command::new("git");

        cmd.arg("rev-parse").current_dir(cannon_path);

        match cmd.output() {
            Ok(o) => {
                if o.status.success() {
                    return Ok(true);
                }
                return Ok(false);
            }
            Err(e) => return Err(e),
        }
    }

    /// Returns the root path for a repository, which is an ancestor of the given path
    ///
    /// The path can be either a file or directory, but must be a descendant of a repository
    pub fn repo_root(path: &Path) -> Result<String, io::Error> {
        let cannon_path = utility::normalize_pathname(path)?;
        if !Self::is_repository(Path::new(&cannon_path))? {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Given path is not a repository, therefore root can not be determined",
            ));
        }

        let mut cmd = Command::new("git");
        cmd.arg("rev-parse")
            .arg("--show-toplevel")
            .current_dir(cannon_path);

        if cmd.output().is_err() {
            return Err(cmd.output().err().unwrap());
        }

        let output = cmd.output().unwrap();

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                format!(
                    "git rev-parse returned non zero exit code; \n {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            ));
        }

        return match String::from_utf8(output.stdout) {
            Ok(p) => {
                return Ok(p);
            }
            ,
            Err(e) => {
                Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Returned data in stdout contained non-UTF8 characters which is not supported, {}",
                e),
            ))
            }
        };
    }
}

#[cfg(test)]
mod parse_test {

    use crate::core::{utility, Repository};
    use dirs;
    use std::path::PathBuf;

    #[test]
    fn check_if_home_is_repo() {
        let home_path = dirs::home_dir().unwrap();

        assert!(!Repository::is_repository(&home_path.as_path()).unwrap());
    }

    #[test]
    fn check_if_manifest_dir_is_repo() {
        let home_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        assert!(Repository::is_repository(&home_path.as_path()).unwrap());
    }

    #[test]
    fn check_if_manifest_dir_returns_history() {
        let manifest_path = PathBuf::from(r"C:\Users\z003yw7n\Desktop\MISC\LearnGit\LearnGitRepo");
        let root = utility::normalize_pathname(&manifest_path).unwrap();
        println!("{}", root);
        let repo = Repository::new(&manifest_path).unwrap();
        println!("{}", repo.path.display());
        let log = repo.log("", "", "%h»¦«%cn»¦«%ce»¦«%ct»¦«%s»¦«%b").unwrap();

        println!("{}", log);

        assert!(!log.is_empty());
    }
}
