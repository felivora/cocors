use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::{git, utility};

pub struct Repository {
    path: PathBuf,
}

impl Repository {
    /// Returns a new Repository with an associated root path from a path, that can be anywhere in the repo hierarchy
    ///
    /// If the provided path is not in a repository, there is no access to the path or git is not installed
    /// an [io::Error] will be returned detailling the issue
    pub fn new(path: &Path) -> Result<Repository, io::Error> {
        let repo_root = Self::repo_root(path)?;

        Ok(Repository { path: repo_root })
    }

    pub fn log(&self, from: &str, to: &str, format: &str) -> io::Result<String> {
        if !Self.is_repository(self.path)? {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Given path is not a repository, must be a valid filesystem path within to a repositor root"));
        }

        let mut cmd = Command::new("git");
        cmd.arg("log");

        

        git log release..main --format="%h»¦«%cn»¦«%ce»¦«%ct»¦«%s»¦«%b"  
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
    pub fn repo_root(path: &Path) -> Result<PathBuf, io::Error> {
        if !Self::is_repository(path)? {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Given path is not a repository, therefore root can not be determined",
            ));
        }
        let cannon_path = utility::normalize_pathname(path)?;
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
                println!("{}", p);
                Ok(PathBuf::from(p))
            },
            Err(e) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Returned data in stdout contained non-UTF8 characters which is not supported, {}",
                e),
            )),
        };
    }
}

#[cfg(test)]
mod parse_test {

    use crate::core::Repository;
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
}
