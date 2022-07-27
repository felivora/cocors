use log::debug;
use std::{io, path::Path};

pub fn normalize_pathname(path: &Path) -> Result<String, io::Error> {
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "The path given does not exist, valid path must be provided",
        ));
    }

    let mut dir_path = path.to_owned();

    if path.is_file() {
        dir_path = match path.parent() {
            Some(p) => p.to_path_buf(),
            None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path given was file and parent is either root or resolves in prefix, enter valid directory path"))
        }
    }

    // Should not panic as the check if the directory exists is already done above
    let mut cannon_path = match dir_path.canonicalize().unwrap().to_str() {
            Some(s) => s.to_string(),
            None => return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Pathname contains invalid UTF8 or is too long, make sure that the pathname is valid. Non UTF8 characters are not supported"))
        };

    if cfg!(target_os = "windows") {
        cannon_path = cannon_path.as_str().replace("\\\\?\\", "");

        debug!(target: "git_repository", "Cleaned up long path name on windows: {}", &cannon_path);
    }

    Ok(cannon_path)
}
