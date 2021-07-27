use std::{path::PathBuf, process};

use dashboard::dashboard_options;
use serde::{Deserialize, Serialize};

mod commands;
mod dashboard;
pub mod data;
mod initial_setup;
mod login;

fn main() {
    let path_to_file = locate_file();
    if !path_to_file.exists() {
        initial_setup::initial_setup::check_if_file_exists();
    } else {
        let hashfile = std::fs::read_to_string(path_to_file).unwrap();
        let hashjson: MasterPassword = serde_json::from_str(&hashfile.as_str()).unwrap();
        let status = login::login::login(hashjson.master_password);
        if status {
            let mut option = dashboard_options::list_option();
            let option: u8 = option.trim().parse::<u8>().unwrap();
            commands::commands::create_password();
        } else {
            eprint!("Incorrect Password");
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

// TODO Probably change this mess into another function
