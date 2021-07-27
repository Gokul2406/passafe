pub mod commands {
    #[macro_use]
    use magic_crypt;
    use magic_crypt::{MagicCryptTrait, new_magic_crypt};
    use serde_json;
    use home;
    use std::io;

    use crate::MasterPassword;

    pub fn create_password() {
        let path_to_dir = home::home_dir().unwrap().join(".local/share/passafe");
        let path_to_file = path_to_dir.join("passwords.json");
        let file  = std::fs::read_to_string(&path_to_file).unwrap();
        let encryption_key: MasterPassword = serde_json::from_str(&file).unwrap();
        let mckey = new_magic_crypt!(&encryption_key.master_password, 256);
        let mut user_input_password = String::new();
        println!("New password");
        io::stdin()
            .read_line(&mut user_input_password)
            .expect("Error while reading input");
        let encrypted_password = mckey.encrypt_str_to_base64(user_input_password.trim());
        println!("encryp {}",encrypted_password);
        let decrypted = mckey.decrypt_base64_to_string(encrypted_password).unwrap();
        println!("decr {:?}", decrypted);
    }
}
