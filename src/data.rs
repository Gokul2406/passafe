use std::{fmt, fs::{write}};

use serde::{Deserialize, Serialize};

use crate::locate_file;

#[derive(Deserialize, Serialize)]
pub struct PasswordJson {
    pub url: String,
    pub name: String,
    pub password: String,
}

impl PasswordJson {
    pub fn new(name: String, url: String, password: String) -> PasswordJson {
        PasswordJson {
            name, url, password
        }
    }

    pub fn insert(name: String, url: String, password: String) {
        let file_contents = std::fs::read_to_string(&locate_file()).unwrap();
        let mut file_json: Data = serde_json::from_str(&file_contents).unwrap();
        let new_password = PasswordJson::new(name, url, password);
        file_json.password.push(new_password);
        file_json.update();
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.master_password)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub(crate) master_password: String,
    pub(crate) password: Vec<PasswordJson>,
}

impl Data {
    pub fn update(&self) {
        write(locate_file(), serde_json::to_string_pretty(&self).unwrap()).unwrap();
    }
}
