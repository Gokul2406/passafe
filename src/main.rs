use dashboard::dashboard_options;
use serde::{Deserialize, Serialize};

mod commands;
mod dashboard;
mod initial_setup;
mod login;

fn main() {
    // The inital setup covers creating the file for passwords etc.
    let path_to_dir = home::home_dir().unwrap().join(".local/share/passafe");
    let path_to_file = path_to_dir.join("passwords.json");

    if !path_to_file.exists() {
        initial_setup::initial_setup::check_if_file_exists();
    } else {
        let hashfile = std::fs::read_to_string(path_to_file).unwrap();
        let hashjson: MasterPassword = serde_json::from_str(&hashfile.as_str()).unwrap();
        let status = login::login::login(hashjson.master_password);
        if status {
            commands::commands::create_password();
        } else {
        }
    }
}

#[derive(Serialize, Deserialize)]
struct MasterPassword<'a> {
    master_password: &'a str,
}
