pub mod commands {
    use crate::data::PasswordJson;
    use std::io;

    pub fn create_password() {
        let mut password = String::new();
        let mut url = String::new();
        let mut name = String::new();
        println!("URL: ");
        io::stdin().read_line(&mut url).expect("An error occurred");
        println!("name: ");
        io::stdin().read_line(&mut name).expect("An error occurred");
        println!("Password: ");
        io::stdin()
            .read_line(&mut password)
            .expect("An error occurred");

        PasswordJson::insert(String::from(name.trim()), String::from(url.trim()), String::from(password.trim()));
    }

    pub fn list_all_passwords() {
        PasswordJson::list();
    }
}
