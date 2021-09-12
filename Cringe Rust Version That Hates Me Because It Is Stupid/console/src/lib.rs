// Message module
pub mod print {
    // Init message type
    pub fn init(message: &str) {
        println!("[Init]: {}", message)
    }

    // Log message type
    pub fn log(message: &str) {
        println!("[Log]: {}", message)
    }

    // Echo message type
    pub fn echo(message: &str) {
        println!("[Echo]: {}", message)
    }

    // Error message type
    pub fn error(message: &str) {
        println!("[Error]: {}", message);
        panic!("{}", message)
    }
}

// Window module
pub mod window {
    // Crates
    use proctitle::set_title;

    // Module to clear parts of the window
    pub mod clear {
        // Clears the screen
        pub fn screen() {
            clearscreen::clear().unwrap();
        }
    }
       
    // Change the window title
    pub fn title(title: &str) {
        set_title(format!("{}", title));
    }
}