use super::commit::commit::Commit;
use log;
use std::io;
use std::path::Path;

pub struct Gitlog {
    pub from: Option<String>,
    pub to: Option<String>,
    pub commits: Option<Vec<Commit>>,
}

impl Gitlog {
    /// Creates a new Log from an existing repository with history.
    /// This is done using the git log command and the provided history of
    /// the repository. There is an optional limitation for the log history
    /// used in order to display only a range within the entire history.
    ///
    /// note: this constructor is purposely not named to `new` to emphasize the fact
    /// that the log is created using an external method that can fail
    #[inline]
    pub fn from_repo(path: &Path, from: String, to: String) -> Result<Gitlog, io::Error> {
        // Path is only valid if it is a directory and exists, therefore
        // at this point the function can fail
        if !path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "The path {} does not exist or is not a directory",
                    path.display()
                ),
            ));
        }

        let format = "%h,%cn,%ce,%ct";

        let log = super::git::log(format, cannon_path.as_path(), from.as_str(), to.as_str());

        todo!()
    }
}
