use console;

// Map related stuff
pub mod map {
    use std::fs;

    // Create the map
    pub fn create(map_name: &str) {
        let _map = fs::create_dir((String::from("./") + map_name).as_str());
        console::print::log("Generated Base map datapack");
    }
}

// Echos a message to the console (For Testing)
pub fn echo(message: &str) {
    console::print::echo(message);
}