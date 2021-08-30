use structopt::StructOpt;
use console;
mod lexer;

// CLI structure
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    script_path: std::path::PathBuf,
}

// Main function
fn main() {
    // Start up compiler
    console::window::title("DyphEngine Compiler");
    console::print::init("Starting up compiler");

    // Get CLI arguments
    console::print::init("Attempting to read provided .dyph file");
    let args = Cli::from_args();
    let script_file = std::fs::read_to_string(&args.script_path)
        .expect("Could not read file");
    
    if script_file.split(".").last().unwrap() != "dyph" {
        lexer::lex_stuff(script_file.as_str());
    } else {
        console::print::error("ERR_INCORRECT_FILE_TYPE");
    }
}