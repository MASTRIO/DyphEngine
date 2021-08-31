use std::fs::File;
use std::io::Write;

// Add data to temp folder
pub fn add_to_temp(file_name: &str, data_to_add: &str) -> std::io::Result<()> {
    let mut file_path = String::from("temp/");
    file_path.push_str(file_name);
    let mut file = File::create(file_path)
        .expect("BRUH");
    file.write_all(data_to_add.as_bytes())?;

    Ok(())
}
