package me.mastrio.dyphengine;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.util.Scanner;

public class DyphEngine {

  // Main method
  public static void main(String[] args) throws IOException {
    // Get the file to compile (ooh that rhymes)
    System.out.println("Input .dyph file path:");
    Scanner scanner = new Scanner(System.in);
    String filePath = scanner.nextLine();

    // Read the file you want to compile (more rhyming!)
    File fileToRead = new File(filePath);
    Scanner fileReader = new Scanner(fileToRead);

    // Run each line through the lexer and stuff
    while (fileReader.hasNextLine()) {
      Lexer.lex(fileReader.nextLine());
    }

  }

}
