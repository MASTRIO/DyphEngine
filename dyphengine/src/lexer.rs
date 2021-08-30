use console;
use parser;

// Lexers whatever you wrote
pub fn lex_stuff(script: &str) {
    // Get each line of the script
    let script_lines: Vec<&str> = script.split("\n").collect();
    let total_lines: usize = script_lines.capacity();
    console::print::log((String::from("Parsing ") + &total_lines.to_string() + " lines of code").as_str());

    // Parse each line
    for lines in script_lines {
        let line_components: Vec<&str> = lines.split(" ").collect();
        parser::parse(line_components);
        let line_components: Vec<&str> = lines.split("").collect();
    }
}