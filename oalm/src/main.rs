extern crate clap;

use std::ffi::OsString;

use std::env;
use std::fs;
use std::process;

mod core;
use core::{download, install, libraries, path};

fn main() {
    let app = clap::App::new("Arduino Library Manager")
        .version("1.0")
        .author("Rached")
        .about("A simple library manager for Arduino.")
        .arg(
            clap::Arg::with_name("path")
                .long("path")
                .value_name("installation path")
                .help("Set manually the path where the libraries have to be installed."),
        )
        .subcommand(
            clap::SubCommand::with_name("install")
                .about("Install a specified library with a given url or zip file.")
                .arg(clap::Arg::with_name("library").takes_value(true)),
        )
        .subcommand(
            clap::SubCommand::with_name("remove")
                .about("Remove a specified library with a given name of that.")
                .arg(clap::Arg::with_name("library").takes_value(true)),
        )
        .subcommand(
            clap::SubCommand::with_name("new")
                .about("Create a new library project for Arduino.")
                .arg(clap::Arg::with_name("project name").takes_value(true)),
        )
        .subcommand(
            clap::SubCommand::with_name("list").about("List all installed libraries for Arduino."),
        )
        .get_matches();

    let libraries_path: OsString = match app.value_of("installation path") {
        Some(value) => OsString::from(value),
        None => path::according_os(),
    };

    env::set_var("ARDUINO_LIBRARIES_PATH", libraries_path);

    match app.subcommand() {
        ("install", Some(matches)) => {
            let library_path = matches.value_of("library").unwrap();

            match install::determine_arg(library_path) {
                Some(install::Arg::IsHttpOrHttps) => {
                    println!("Download the zip file at {}", library_path);

                    match download::from_http(library_path) {
                        Ok(http_data) => {
                            println!("Successfully downloaded !");
                            println!("Extract the zip file for {}", library_path);

                            match install::zip_file(http_data) {
                                Ok(path_to_installation) => {
                                    let display_path = path_to_installation.into_string().unwrap();

                                    println!("Zip file successfully installed at {}", display_path);
                                }

                                Err(why) => {
                                    eprintln!("Can not unzip this file because : {}", why);
                                    process::exit(1);
                                }
                            }
                        }

                        Err(why) => {
                            eprintln!("{}", why);
                            process::exit(1);
                        }
                    }
                }

                Some(install::Arg::IsGitRepo) => match install::git_repository(library_path) {
                    Ok(repo_location) => {
                        println!(
                            "Successfully installed the repository project at {:?}",
                            repo_location
                        );
                    }

                    Err(why) => {
                        eprintln!("Error : {}", why);
                        process::exit(1);
                    }
                },

                Some(install::Arg::IsZipFile) => {
                    let file = fs::File::open(library_path).expect("No such file");

                    match install::zip_file(file) {
                        Ok(path_to_installation) => {
                            let display_path = path_to_installation.into_string().unwrap();

                            println!("Zip file successfully installed at {}", display_path);
                        }

                        Err(why) => {
                            eprintln!("Can not unzip this file because : {}", why);
                            process::exit(1);
                        }
                    }
                }

                _ => {
                    eprintln!(
                        "Your input doesn't match with any target to install a Arduino library."
                    );
                    process::exit(1);
                }
            }
        }

        ("remove", Some(matches)) => {
            match libraries::remove(matches.value_of("library").unwrap()) {
                Ok(uninstalled_lib) => println!("Successfully removed {}", uninstalled_lib),
                Err(why) => {
                    eprintln!("Can not remove this library because : {}", why);
                    process::exit(1);
                }
            }
        }

        ("new", Some(matches)) => {
            let project_name = matches.value_of("project name").unwrap();

            match libraries::create(project_name) {
                Ok(installation_path) => {
                    println!(
                        "Successfully created {:?} at {:?}",
                        project_name, installation_path
                    );
                }

                Err(why) => {
                    eprintln!(
                        "{}",
                        format!("Can not created {:?} because : {}", project_name, why)
                    );
                    process::exit(1);
                }
            }
        }

        ("list", Some(_)) => match libraries::get_all() {
            Ok(all_libraries) => {
                if all_libraries.is_empty() {
                    println!("Any library was found.");
                } else {
                    println!("Libraries\n-----------");

                    for lib in all_libraries {
                        println!("{}", lib.into_string().unwrap());
                    }
                }
            }

            Err(why) => {
                eprintln!("Can not list the libraries because : {}", why);
                process::exit(1);
            }
        },

        _ => eprintln!("You have to type a command, type oapm --help for a help."),
    }
}
