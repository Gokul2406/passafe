mod dashboard;
mod initial_setup;

fn main() {
    // The inital setup covers creating the file for passwords etc.
    let path_to_dir = home::home_dir().unwrap().join(".local/share/passafe");
    let path_to_file = path_to_dir.join("passwords.json");

    if !path_to_file.exists() {
        println!("no file");
        initial_setup::initial_setup::check_if_file_exists();
    }
    dashboard::dashboard_options::list_option();
}
