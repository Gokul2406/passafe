pub mod dashboard_options {
    use std::io;
    pub fn list_option() {
        let mut selected_option = String::new();
        println!("1) Create Password \n2) Delete Password \n3) View Password");
        println!("\nSelect an option");
        io::stdin()
            .read_line(&mut selected_option)
            .expect("An error occured ");
    }
}
