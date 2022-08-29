use dunce;
use std::{io, path::Path};

pub fn normalize_pathname(path: &Path) -> Result<String, io::Error> {
    let mut dir_path = path.to_owned();

    if path.is_file() {
        dir_path = match path.parent() {
            Some(p) => p.to_path_buf(),
            None => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path given was file and parent is either root or resolves in prefix, enter valid directory path"))
        }
    }
    // Should not panic as the check if the directory exists is already done above
    let cannon_path = match dunce::canonicalize(dir_path).unwrap().to_str() {
            Some(s) => s.to_string(),
            None => return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Pathname does not exist, contains invalid UTF8 or is too long, make sure that the pathname is valid. Non UTF8 characters are not supported"))
        };

    Ok(cannon_path)
}
