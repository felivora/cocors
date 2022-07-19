use std::{
    fs::{self},
    path::PathBuf,
};

pub fn read_manifest(path: PathBuf) -> Option<(String, PathBuf)> {
    // if the given path is a direct reference to a file
    // check if it is the manifest file, otherwise return none
    if path.is_file() {
        if path.file_name().unwrap() == "apax.yml" {
            let manifest = fs::read_to_string(path.clone());
            if manifest.is_ok() {
                return Some((manifest.unwrap(), path));
            }
        }
    } else if path.is_dir() {
        // iterate over every entry in the directory
        // which might be a link, file or directory itself
        for entry in fs::read_dir(path).unwrap() {
            match entry {
                Ok(dir) => {
                    if !dir
                        .path()
                        .as_os_str()
                        .to_str()
                        .unwrap()
                        .contains(String::from(".apax").as_str())
                    {
                        let found_in_subfolder = read_manifest(dir.path());
                        if found_in_subfolder.is_some() {
                            return found_in_subfolder;
                        }
                    }
                }
                Err(_) => continue,
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {

    use assert_fs::fixture::{FileTouch, PathChild};

    use crate::utility::fs_helper::read_manifest;

    #[test]
    fn empty_directory_no_manifest_exists() {
        // Creates empty temporary directory
        let temp = assert_fs::TempDir::new().unwrap();
        assert_eq!(None, read_manifest(temp.path().to_path_buf()));
        temp.close().unwrap();
    }

    #[test]
    fn manifest_file_directly_is_some() {
        // Creates empty temporary directory
        let temp_dir = assert_fs::TempDir::new().unwrap();

        let path = temp_dir.child("apax.yml");

        path.touch().unwrap();

        println!("{:?}", path.path());
        assert!(read_manifest(path.to_path_buf()).is_some());

        temp_dir.close().unwrap();
    }

    #[test]
    fn manifest_file_in_subdir_is_some() {
        // Creates empty temporary directory
        let temp_dir = assert_fs::TempDir::new().unwrap();

        let path = temp_dir.child("packages/libraries/LBC/apax.yml");

        path.touch().unwrap();

        println!("{:?}", path.path());
        assert!(read_manifest(temp_dir.to_path_buf()).is_some());

        temp_dir.close().unwrap();
    }
}
