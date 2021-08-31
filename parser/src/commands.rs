use console;

// Map related stuff
pub mod map {
    // Crates
    use file_manager;

    // Create the map
    pub fn create(_map_name: &str) -> std::io::Result<()> {
        file_manager::create_datapack_files()
    }
}

// Script related stuff
pub mod script {
  use std::io::Error;

  // Create a new script
    pub fn create(script_name: &str) {
        console::print::log("Created a new script");
        let mut script_path = String::from("script_");
        script_path.push_str(&script_name);
        match file_manager::temp::add_to_temp(script_path.as_str(), script_name) {
          Ok(_) => {}
          Err(_) => console::print::error("ERR_CREATE_SCRIPT")
        }
    }
}

// Echos a message to the console (For Testing)
pub fn echo(message: &str) {
    console::print::echo(message);
}
