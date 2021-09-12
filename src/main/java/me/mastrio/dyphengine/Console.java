package me.mastrio.dyphengine;

public class Console {

  // Message types
  // Init
  static void init(String message) {
    System.out.println("[Init]: " + message);
  }

  // Log
  static void log(String message) {
    System.out.println("[Log]: " + message);
  }

  // Echo
  static void echo(String message) {
    System.out.println("[Echo]: " + message);
  }

  // Error
  static void error(String errorType) {
    System.out.println("[Error]: " + errorType);
    System.exit(69);
  }

}
