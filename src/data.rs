use std::{fmt, fs::{write}};

use magic_crypt::{MagicCryptTrait, new_magic_crypt};
use serde::{Deserialize, Serialize};

use crate::locate_file;

#[derive(Deserialize, Serialize, Debug)]
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
        let mut file_json = convert_password_file_to_json();
        let mckey = new_magic_crypt!(&file_json.master_password, 256);
        let encrypted_password = mckey.encrypt_to_base64(&password);
        let new_password = PasswordJson::new(name, url, encrypted_password);
        file_json.password.push(new_password);
        file_json.update();
    }

    pub fn list() {
        let file_json = convert_password_file_to_json();
        let passwords = file_json.password;
        for password in &passwords {
            println!("{}", password)
        }
    }
}

impl fmt::Display for PasswordJson {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let master_password = get_master_password();
        let mc = new_magic_crypt!(master_password, 256);
        let decrypted_password = mc.decrypt_base64_to_string(&self.password).unwrap();
        write!(f, "URL: {}, Name: {}, Password: {}", self.url, self.name, decrypted_password)
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


fn convert_password_file_to_json() -> Data {
        let file_contents = std::fs::read_to_string(&locate_file()).unwrap();
        let mut file_json: Data = serde_json::from_str(&file_contents).unwrap();
        return file_json
}

fn get_master_password() -> String {
    let file = convert_password_file_to_json();
    file.master_password
}
