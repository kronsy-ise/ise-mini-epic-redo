package dev.kronsy.ise.epic2;

import java.util.ArrayList;
import java.util.Scanner;

import dev.kronsy.ise.epic2.calculation.ExecutionEngine;
import dev.kronsy.ise.epic2.errors.CalculatorError;
import dev.kronsy.ise.epic2.text_processing.Lexer;
import dev.kronsy.ise.epic2.text_processing.Parser;
import dev.kronsy.ise.epic2.text_processing.Token;

public class App 
{
    public static void main( String[] args ) throws CalculatorError
    {
      String prompt = "Eval    > ";
      String red = "\033[0;31m";
      String reset = "\033[0;0m";
      String yellow = "\033[0;33m";
      String blue = "\033[0;34m";
      int prompt_len = prompt.length();
      System.out.println("Welcome to Super Duper Calculator 10,000");

      Scanner scan = new Scanner(System.in);

      ExecutionEngine engine = new ExecutionEngine();
      while(true){
        System.out.print(blue + prompt + reset);
        String input = scan.nextLine();

        try{
          var tokens = new Lexer(input).all();
          var parsed = new Parser(tokens).parse();

          parsed.execute(engine);
          // System.out.println(parsed);
        }
        catch(CalculatorError ce){
          String underline = " ".repeat(prompt_len + ce.pos.begin) + red + "~".repeat(ce.pos.width()) + yellow + " < " + ce.message +reset + "\n";
          System.out.println(underline);
        }
      }


      // String input = scan.nextLine();
      // Lexer l = new Lexer(input);
      //
      // var toks = l.all();
      //
      // // while(true){
      // //   Token t = l.next();
      // //   if(t == null) break;
      // //   System.out.println("> " + t.value +" :: " + t.kind + " << " + t.location);
      // //   toks.add(t);
      // // }
      //
      // Parser p = new Parser(toks);
      //
      // var result = p.parse();
      //
      // System.out.println("Parsed: " + result.toString());
    }
}
