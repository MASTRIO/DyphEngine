package me.mastrio.dyphengine;

import java.io.IOException;

// Is this a lexer though? I don't think so
// TODO make this good? na
public class Lexer {

  // Lexes stuff in the lexer
  // Pretty sure lex isn't a word, oh well
  static void lex(String line) throws IOException {
    // Is the line a comment instead of code? maybe
    if (line.startsWith("#") || line.equals("")) {
      return;
    } else {
      // Seperate the code from it's family :(
      String[] lexedCode = line.split(" ");
      Parser.parseLine(lexedCode);
    }
  }

}
