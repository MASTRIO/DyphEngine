use std::fs::File;
use std::fs;
use std::io::{Write, Error};
use console;
pub mod temp;

// Create datapack files
pub fn create_datapack_files() -> std::io::Result<()> {
    // Delete old directories
    match fs::remove_dir_all("./Build") {
        Ok(_) => console::print::log("Deleted previous Build directory"),
        Err(_e) => console::print::log("Could not delete previous Build directory")
    }

    // Root folders
    fs::create_dir("./Build")?;
    fs::create_dir("./Build/Map")?;
    let mut file = File::create("Build/Map/pack.mcmeta")?;
    let pack_meta = String::from("{\n  \"pack\": {\n    \"pack_format\": 7,\n    \"description\": \"A Game generated using DyphEngine\"\n  }\n}");
    file.write_all(pack_meta.as_bytes())?;
    fs::create_dir("./Build/Map/data")?;

    // Default folders
    fs::create_dir("./Build/Map/data/minecraft")?;
    fs::create_dir("./Build/Map/data/minecraft/tags")?;
    fs::create_dir("./Build/Map/data/minecraft/tags/functions")?;
    let mut file = File::create("Build/Map/data/minecraft/tags/functions/load.json")?;
    file.write_all(b"{\n    \"values\": [\n       \"map:load\"\n    ]\n}")?;
    let mut file = File::create("Build/Map/data/minecraft/tags/functions/tick.json")?;
    file.write_all(b"{\n    \"values\": [\n       \"map:tick\"\n    ]\n}")?;

    // Map folders
    fs::create_dir("./Build/Map/data/map")?;
    fs::create_dir("./Build/Map/data/map/functions")?;
    let mut file = File::create("Build/Map/data/map/functions/load.mcfunction")?;
    file.write_all(b"")?;
    let mut file = File::create("Build/Map/data/map/functions/tick.mcfunction")?;
    file.write_all(b"")?;
    fs::create_dir("./Build/Map/data/map/functions/scripts")?;
    fs::create_dir("./Build/Map/data/map/functions/structures")?;
    fs::create_dir("./Build/Map/data/map/functions/entities")?;

    Ok(())
}

// Temp folder stuff
pub fn create_temp_folder() {
    match fs::create_dir("./temp") {
        Ok(_) => console::print::log("Created temp folder"),
        Err(_) => console::print::error("ERR_CREATE_TEMP_FOLDER")
    }
}

pub fn delete_temp_folder() {
    match fs::remove_dir_all("./temp") {
        Ok(_) => console::print::log("Deleted temp folder"),
        Err(_) => {}
    }
}
