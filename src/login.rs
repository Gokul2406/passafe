pub mod login {
    use bcrypt;
    use rpassword::read_password;

    pub fn login(hash: &str) -> bool {
        println!("Master Password: ");
        let user_password = read_password().unwrap();
        bcrypt::verify(user_password.trim(), hash).unwrap()
    }
}
