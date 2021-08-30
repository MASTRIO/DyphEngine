use console;

// Map related stuff
pub mod map {
    // Crates
    use std::fs::File;
    use std::fs;
    use std::io::Write;

    // Create the map
    pub fn create(_map_name: &str) -> std::io::Result<()> {
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
}

// Echos a message to the console (For Testing)
pub fn echo(message: &str) {
    console::print::echo(message);
}