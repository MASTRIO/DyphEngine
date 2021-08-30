use console;
mod commands;

// Parse each line of the file
pub fn parse(components: Vec<&str>) {
    match components[0] {
        "map" => {
            match components[1] {
                "create" => commands::map::create(components[2]),
                _ => console::print::error("ERR_UNKNOWN_COMMAND")
            }
        }
        "echo" => commands::echo(components[1]),
        _ => console::print::error("ERR_UNKNOWN_COMMAND")
    }
}