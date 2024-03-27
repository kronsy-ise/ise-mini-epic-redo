package dev.kronsy.ise.epic2;

import dev.kronsy.ise.epic2.calculation.ExecutionEngine;
import dev.kronsy.ise.epic2.errors.CalculatorError;
import dev.kronsy.ise.epic2.text_processing.Lexer;
import dev.kronsy.ise.epic2.text_processing.Parser;
import dev.kronsy.ise.epic2.util.Color;
import java.util.Scanner;

public class App {
  public static void main(String[] args) throws CalculatorError {
    String prompt = "Eval    > ";
    int prompt_len = prompt.length();
    System.out.println(Color.green("Welcome to Super Duper Calculator 10,000"));

    Scanner scan = new Scanner(System.in);

    ExecutionEngine engine = new ExecutionEngine();
    while (true) {
      System.out.print(Color.blue(prompt));
      String input = scan.nextLine();

      if (input.length() == 0) {
        break;
      } else if (input.equals("exit")) {
        break;
      }
      try {
        var tokens = new Lexer(input).all();

        // We were given an empty line, just exit
        if(tokens.size() == 0){
          break;
        }
        var parsed = new Parser(tokens).parse();

        parsed.execute(engine);
      } catch (CalculatorError ce) {

        // We make a little underline for the error location, and print it out
        String underline = " ".repeat(prompt_len + ce.pos.begin) +
                           Color.red("~".repeat(ce.pos.width())) +
                           Color.yellow(" < " + ce.message) + "\n";
        System.out.println(underline);
      }
    }

    scan.close();

    System.out.println(Color.green("Thank you for using Super Duper Calculator 10,000"));

  }
}
