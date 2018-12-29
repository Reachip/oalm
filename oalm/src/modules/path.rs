extern crate dirs;
use std::ffi::OsString;

pub fn according_os() -> OsString {
    if cfg!(unix) || cfg!(macos) {
        let mut home_dir = dirs::home_dir()
            .unwrap()
            .into_os_string(); 
        
        home_dir.push("/Arduino/libraries/");
        home_dir
    }

    else if cfg!(windows) {
        OsString::from("C:/Program Files (x86)/Arduino/libraries")
    }

    else {
        OsString::from("/")
    }
}