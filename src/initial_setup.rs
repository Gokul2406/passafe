pub mod initial_setup {
    use home::home_dir;
    use std::fs::{File, write};
    use serde_json;
    use std::io;

    // The function checks if the passwords.json file exists
    // and if it doesn't, it creates the file
    // If the file exists also the program wont panic which is the intedended behaviour
    pub fn check_if_file_exists() {
        let password_dir = home_dir().unwrap().join(".local/share/passafe");
        let password_file = password_dir.join("passwords.json");

        // Create the program dir where we can store the passwords file
         std::fs::create_dir_all(&password_dir).unwrap();

        // Create the file where we can store the passwords
            File::create(&password_file).unwrap();

        let master_password = get_master_password_from_user().unwrap();
        write(&password_file, serde_json::to_string_pretty(&master_password).unwrap()).unwrap();
    }

    fn get_master_password_from_user() -> Result<String, io::Error> {
        let mut master_password = String::new();
        println!("Lets setup your new password for this app");
        io::stdin()
            .read_line(&mut master_password)
            .expect("Some error occurred");
        let master_password = String::from(master_password.trim());

        Ok(master_password)
    }
}
