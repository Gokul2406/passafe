mod dashboard;
mod commands;
mod login;
mod initial_setup;
mod data;

use std::{path::PathBuf, process};
use serde::{Deserialize, Serialize};

use crate::dashboard::dashboard_options;

pub fn run() {
    let path_to_file = locate_file();
    if !path_to_file.exists() {
        initial_setup::initial_setup::check_if_file_exists();
    } else {
        let hashfile = std::fs::read_to_string(path_to_file).unwrap();
        let hashjson: MasterPassword = serde_json::from_str(&hashfile.as_str()).unwrap();
        let status = login::login::login(hashjson.master_password);
        if status {
            loop {
            let option = dashboard_options::list_option();
            let option: u8 = option.trim().parse::<u8>().unwrap();
            match option {
                1 =>  commands::commands::create_password(),
                2 =>  {
                    commands::commands::list_all_passwords();
                },
                3 => process::exit(1),
                _ => {
                    eprintln!("Select from the given option \n");
                        continue;
                }
            }
            }

        } else {
            eprint!("Incorrect Password \n");
            process::exit(1);
        }
    }
}

#[derive(Serialize, Deserialize)]
struct MasterPassword<'a> {
    master_password: &'a str,
}

pub fn locate_file() -> PathBuf {
    let path_to_dir = home::home_dir().unwrap().join(".local/share/passafe");
    path_to_dir.join("passwords.json")
}
