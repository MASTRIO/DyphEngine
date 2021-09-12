package me.mastrio.dyphengine;

import java.io.IOException;

public class Parser {

  // "Parsing your code one line at a time"
  //                          - DyphEngine, 2021
  static void parseLine(String[] line) throws IOException {
    switch (line[0]) {
      // Map
      case "map":
        switch (line[1]) {
          // Create new map
          case "create":
            FileManager.createBaseDatapck();
            break;
          default:
            Console.error("ERR_UNKNOWN_COMMAND");
        }
        break;
      // Echo a message to the console
      case "echo":
        assert line[1] != null;
        Console.echo(line[1]);
        break;
      // If the parser cannot recognise the code
      default:
        Console.error("ERR_UNKNOWN_COMMAND");
    }
  }

}
