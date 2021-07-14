pub mod initial_setup {
    use home::home_dir;
    use std::fs::File;

    // The function checks if the passwords.json file exists
    // and if it doesn't, it creates the file
    // If the file exists also the program wont panic which is the intedended behaviour
    pub fn check_if_file_exists() {
        let password_dir = home_dir().unwrap().join(".local/share/passafe");
        let password_file = password_dir.join("passwords.json");

        // Create the program dir where we can store the passwords file
        if !password_dir.exists() {
            std::fs::create_dir_all(&password_dir).unwrap();
        }

        // Create the file where we can store the passwords
        if !password_file.exists() {
            File::create(&password_file).unwrap();
        }
    }
}
