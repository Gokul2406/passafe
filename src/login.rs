pub mod login {
    use bcrypt;
    use std::io;

    pub fn login(hash: &str) -> bool {
        let mut user_password = String::new();
        println!("Master Password: ");
        io::stdin()
            .read_line(&mut user_password)
            .expect("Error while reading line");
        bcrypt::verify(user_password.trim(), hash).unwrap()
    }
}
