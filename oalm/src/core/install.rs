extern crate git2;
extern crate tempfile;
extern crate zip;

use std::env;
use std::fs;
use std::io;
use std::path::Path;

use std::ffi::{OsStr, OsString};

pub enum Arg {
    IsHttpOrHttps,
    IsGitRepo,
    IsZipFile,
}

pub fn determine_arg(path: &str) -> Option<Arg> {
    let extension_of_lib = Path::new(&path).extension().and_then(OsStr::to_str);

    if (path.starts_with("http") || path.starts_with("https")) && path.ends_with(".zip") {
        Some(Arg::IsHttpOrHttps)
    } else if path.starts_with("https://github.com/") {
        Some(Arg::IsGitRepo)
    } else if let Some("zip") = extension_of_lib {
        Some(Arg::IsZipFile)
    } else {
        None
    }
}

pub fn zip_file<T: Sized + io::Read>(zip_path: T) -> Result<OsString, io::Error> {
    let mut path = zip_path;
    let mut temp_file = tempfile::tempfile().expect("Failed to create a temporary file ...");

    io::copy(&mut path, &mut temp_file)?;

    let mut archive = zip::ZipArchive::new(temp_file)?;
    let path_to_installation = env::var_os("ARDUINO_LIBRARIES_PATH").unwrap();

    for folder_into_zip in 0..archive.len() {
        let mut file = archive.by_index(folder_into_zip)?;

        let outpath = file.sanitized_name();

        if file.name().ends_with("/") {
            fs::create_dir_all(Path::new(&path_to_installation).join(&outpath))?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    fs::create_dir_all(&parent)?;
                }
            }

            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(path_to_installation)
}

pub fn git_repository(url: &str) -> Result<OsString, git2::Error> {
    let url_split: Vec<&str> = url.split("/").collect();

    let libraries_location = env::var_os("ARDUINO_LIBRARIES_PATH").unwrap();

    let mut install_path = libraries_location;
    install_path.push("/");
    install_path.push(url_split[4]);

    match git2::Repository::clone(url, &install_path) {
        Ok(_) => Ok(install_path),
        Err(why) => Err(why),
    }
}
