pub mod commands {
    use crate::data::PasswordJson;
    use std::io;
    use rpassword::read_password;

    pub fn create_password() {
        let mut url = String::new();
        let mut name = String::new();
        println!("URL: ");
        io::stdin().read_line(&mut url).expect("An error occurred");
        println!("name: ");
        io::stdin().read_line(&mut name).expect("An error occurred");
        println!("Password: ");
        let password = read_password().unwrap();

        PasswordJson::insert(String::from(name.trim()), String::from(url.trim()), String::from(password.trim()));
    }

    pub fn list_all_passwords() {
        PasswordJson::list();
    }
}
