pub mod commands {
    use colored::Colorize;
    use crate::data::PasswordJson;
    use regex::Regex;
    use rpassword::read_password;
    use std::io;

    pub fn create_password() {
        let url_regex = Regex::new(
            r"[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)
",
        )
        .unwrap();
        let mut url = String::new();
        let mut name = String::new();
        println!("URL: ");
        io::stdin().read_line(&mut url).expect("An error occurred");
        if url_regex.is_match(&url) {
            println!("name: ");
            io::stdin().read_line(&mut name).expect("An error occurred");
            println!("Password: ");
            let password = read_password().unwrap();

            PasswordJson::insert(
                String::from(name.trim()),
                String::from(url.trim()),
                String::from(password.trim()),
            );
        }

        println!("{}", "Use correct syntax for URL".red().bold())
    }

    pub fn list_all_passwords() {
        PasswordJson::list();
    }
}
