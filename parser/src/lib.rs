use std::process::Command;

use console;
mod commands;

// Parse each line of the file
pub fn parse(components: Vec<&str>) {
    for _lines in components.clone() {
        match components[0] {
            "echo" => commands::echo(components[1]),
            _ => console::print::error("ERR_UNKNOWN_COMMAND")
        }
    }
}