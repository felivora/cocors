
use std::process::Command;

/// Checks if git is installed and can be found by the system
///
/// Does a simple version check in the background and only looks for the
/// output Result of the [std::process::Command]; no other checks are
/// executed (e.g. ExitCode check)
pub fn is_installed() -> bool {
    let mut cmd = Command::new("git");

    cmd.arg("version");

    match cmd.output() {
        Ok(_o) => {
            true
        }
        Err(_e) => false,
    }
}
