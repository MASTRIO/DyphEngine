use console;
mod commands;

// Parse each line of the file
pub fn parse(components: Vec<&str>) {
    match components[0] {
        "map" => {
            match components[1] {
                "create" => {
                    match commands::map::create(components[2]) {
                        Ok(_) => console::print::log("Created Base datapack files"),
                        Err(_e) => console::print::log("Either the base datapack files already exist or there was an error, ignoring"),
                    }
                }
                _ => console::print::error("ERR_UNKNOWN_COMMAND")
            }
        }
        "script" => {
            match components[1] {
                "create" => commands::script::create(components[2]),
                _  => console::print::error("ERR_UNKNOWN_COMMAND")
            }
        }
        "echo" => commands::echo(components[1]),
        _ => console::print::error("ERR_UNKNOWN_COMMAND")
    }
}
