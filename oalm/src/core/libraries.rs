use std::ffi::OsString;
use std::fs;
use std::io;

use path;

pub fn create(project_name: &str) -> Result<OsString, io::Error> {
    let parent_directory = path::according_os();
    let mut path_to_installation = path::according_os();

    path_to_installation.push(project_name);

    fs::create_dir(&path_to_installation)?;
    path_to_installation.push("/main.c");

    fs::File::create(&path_to_installation)?;

    Ok(parent_directory)
}

pub fn remove(library: &str) -> Result<&str, io::Error> {
    let mut libraries_dir = path::according_os();
    libraries_dir.push(library);

    let target_path = libraries_dir;

    fs::remove_dir_all(target_path)?;
    Ok(library)
}

pub fn get_all() -> Result<Vec<OsString>, io::Error> {
    let mut libraries = Vec::new();

    for entry in fs::read_dir(path::according_os())? {
        let entry = entry?;

        if entry.path().is_dir() {
            libraries.push(entry.file_name());
        }
    }

    Ok(libraries)
}
