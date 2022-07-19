use log::{info, trace, warn};
use std::path::Path;
use std::process::Command;
/// Is checking if git is installed and can be found as an executable by the
/// system
pub fn installed() -> bool {
    let mut cmd = Command::new("git");

    trace!("Running command \"git --version\"");

    cmd.arg("--version");

    if cmd.output().is_err() {
        warn!("Command \"git --version\" can not be executed");
        return false;
    }
    true
}

pub fn is_repo(path: &Path) -> bool {
    let mut cmd = Command::new("git");

    trace!("Running command \"git -C {} rev-parse\"", path.display());

    cmd.arg(format!("rev-parse")).current_dir(path);

    let out = cmd.output();

    if out.is_err() {
        warn!("Path is not part of a git repository");
        return false;
    }

    let result = out.unwrap();
    println!("{:?}", String::from_utf8(result.clone().stderr).unwrap());
    println!("{:?}", String::from_utf8(result.clone().stdout).unwrap());
    println!("{}", result.status.code().unwrap());

    if result.status.success() {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::{installed, is_repo};
    use std::path::PathBuf;

    #[test]
    fn check_if_git_is_installed() {
        assert_eq!(installed(), true);
    }
}
