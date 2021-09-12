package me.mastrio.dyphengine;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;

public class FileManager {

  // Variable vomit
  static String currentDir = System.getProperty("user.dir");
  static File newFile;
  static FileWriter fillFile;
  static Path path;

  // TEMP Load / Tick functions
  static String[] load = new String[] {
    "# Load"
  };
  static String[] tick = new String[] {
    "# Tick"
  };

  // Create a base datapack
  static void createBaseDatapck() throws IOException {
    // Announce stuff
    Console.log("Creating new Map");

    // Set pack path
    String packPath = currentDir + "/Export/";

    // Create base folder
    path = Paths.get(packPath);
    Files.createDirectories(path);

    // Set pack path (again)
    packPath = packPath + "/Map/";

    // Create data folder
    path = Paths.get(packPath + "/data/");
    Files.createDirectories(path);

    // Create pack.mcmeta file
    newFile = new File(packPath + "/pack.mcmeta");
    fillFile = new FileWriter(packPath + "/pack.mcmeta");
    fillFile.write("{\n");
    fillFile.write("  \"pack\": {\n");
    fillFile.write("    \"pack_format\": 7,\n");
    fillFile.write("    \"description\": \"A Game made using DyphEngine\"\n");
    fillFile.write("  }\n");
    fillFile.write("}\n");
    fillFile.close();

    // Minecraft
    path = Paths.get(packPath + "/data/minecraft/");
    Files.createDirectories(path);

    path = Paths.get(packPath + "/data/minecraft/tags/");
    Files.createDirectories(path);

    path = Paths.get(packPath + "/data/minecraft/tags/functions/");
    Files.createDirectories(path);

    // Create load.json file
    newFile = new File(packPath + "/data/minecraft/tags/functions/load.json");
    fillFile = new FileWriter(packPath + "/data/minecraft/tags/functions/load.json");
    fillFile.write("{\n");
    fillFile.write("  \"values\": [\n");
    fillFile.write("    \"map:load\"\n");
    fillFile.write("  ]\n");
    fillFile.write("}\n");
    fillFile.close();

    // Create tick.json file
    newFile = new File(packPath + "/data/minecraft/tags/functions/tick.json");
    fillFile = new FileWriter(packPath + "/data/minecraft/tags/functions/tick.json");
    fillFile.write("{\n");
    fillFile.write("  \"values\": [\n");
    fillFile.write("    \"map:tick\"\n");
    fillFile.write("  ]\n");
    fillFile.write("}\n");
    fillFile.close();

    // DyphEngine
    path = Paths.get(packPath + "/data/dyphengine.");
    Files.createDirectories(path);

    // Map
    path = Paths.get(packPath + "/data/map/");
    Files.createDirectories(path);

    path = Paths.get(packPath + "/data/map/functions/");
    Files.createDirectories(path);

    // Create tick.mcfunction file
    newFile = new File(packPath + "/data/map/functions/tick.mcfunction");
    fillFile = new FileWriter(packPath + "/data/map/functions/tick.mcfunction");
    int tf = 0;
    for (int i = tick.length - 1; i >= 0; i--) {
      fillFile.write(tick[tf] + "\n");
      tf++;
    }
    fillFile.close();

    // Create load.mcfunction files
    newFile = new File(packPath + "/data/map/functions/load.mcfunction");
    fillFile = new FileWriter(packPath + "/data/map/functions/load.mcfunction");
    int lf = 0;
    for (int i = load.length - 1; i >= 0; i--) {
      fillFile.write(load[lf] + "\n");
      lf++;
    }
    fillFile.close();
  }

}
