mod dashboard;
mod initial_setup;

fn main() {
    // The inital setup covers creating the file for passwords etc.
    initial_setup::initial_setup::check_if_file_exists();
    dashboard::dashboard_options::list_option();
}
