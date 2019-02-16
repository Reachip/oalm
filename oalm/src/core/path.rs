extern crate dirs;
extern crate username;

use std::ffi::OsString;

pub fn according_os() -> OsString {
    if cfg!(unix) || cfg!(macos) {
        let mut home_dir = dirs::home_dir().unwrap().into_os_string();

        home_dir.push("/Arduino/libraries/");
        home_dir
    } else if cfg!(windows) {
        let username = username::get_user_name().unwrap();
        OsString::from(format!(
            "C:/Users/{}/Documents/Arduino/libraries/",
            username
        ))
    } else {
        OsString::from("/")
    }
}
