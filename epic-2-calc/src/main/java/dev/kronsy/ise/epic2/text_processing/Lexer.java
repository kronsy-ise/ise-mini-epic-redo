package dev.kronsy.ise.epic2.text_processing;

import java.util.ArrayList;
import java.util.Scanner;

import dev.kronsy.ise.epic2.errors.CalculatorError;



public class Lexer{
  public String input;
  public int index = 0;
  private Integer span_start = null;


  public Lexer(String input){
    this.input = input;
  }



  public ArrayList<Token> all() throws CalculatorError{
    ArrayList<Token> r = new ArrayList<>();

    while(true){
      var next = this.next();
      if(next == null)break;
      r.add(next);
    }

    return r;
  }

  public Token next() throws CalculatorError{

    try{

      this.begin_span();
      char start = this.next_char();

      // Skip all whitespace
      while(Lexer.is_whitespace(start)){
        this.end_span();
        this.begin_span();
        start = this.next_char();
      }


      // We are processing a 'word' token
      if(Lexer.is_alpha(start)){
        String rest = this.accumulate_consecutive(a -> Lexer.is_alpha(a));
        String word = new String();
        word += start;

        if(rest != null){
          word += rest;
        }

        return new Token(TokenKind.Word, word, this.end_span());
      }

      // We are processing a 'literal' token
      if(Lexer.is_num(start)){
        String rest = this.accumulate_consecutive(n -> Lexer.is_num(n));
        String num = new String();
        num += start;

        if(rest != null){
          num += rest;
        }

        try{
          if(this.peek() == '.' && Lexer.is_num(this.peek(2))){
            // We have ourselves a decimal literal 'A.B'
            num += this.next_char();

            num += this.accumulate_consecutive(n -> Lexer.is_num(n));

          }
        }
        catch(EndOfInput e){}
        return new Token(TokenKind.Number, num, this.end_span());
      }

      TokenKind kind = switch(start){
        case '+' -> TokenKind.Plus;
        case '-' -> TokenKind.Minus;
        case '*' -> TokenKind.Star;
        case '/' -> TokenKind.Slash;
        case '^' -> TokenKind.Karat;
        case '=' -> TokenKind.Equals;
        case '(' -> TokenKind.OpenParen;
        case ')' -> TokenKind.CloseParen;
        case ',' -> TokenKind.Comma;
        default  -> null;
      };

      if(kind == null){
        // Unrecognized token 
        Span error_span = this.end_span();
        throw new CalculatorError(error_span, "Unrecognized Input Token");
      }
      else{
        String s = new String();
        s += start;
        return new Token(kind, s, this.end_span());
      }
    }
    catch(EndOfInput e){
      return null;
    }


  }


  private interface Accumulator{
    public boolean op(char test);
  }
  private class EndOfInput extends Throwable{}

  private String accumulate_consecutive(Accumulator test){
    try{
      if(!test.op(this.peek()))return null;
    }
    catch(EndOfInput e){
      return null;
    }
    String word = new String();

    try{
      while(test.op(this.peek())){
        word += this.next_char();
      }
    }
    catch(EndOfInput e){
      // Just continue
      return word;
    }

    return word;
  }



  private static boolean is_whitespace(char c){
    return c <= 0x20 || c == 0x7F;
  }

  private static boolean is_alpha(char c){
    return (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z');
  }

  private static boolean is_num(char c){
    return c >= '0' && c <= '9';
  }

  private char next_char() throws EndOfInput{
    if(this.index >= this.input.length()) throw new EndOfInput();
    return this.input.charAt(this.index++);
  }

  private char peek() throws EndOfInput{
    if(this.index >= this.input.length()) throw new EndOfInput();
    return this.input.charAt(this.index);
  }
  private char peek(int n){
    return this.input.charAt(this.index + n - 1);
  }

  private void begin_span(){
    this.span_start = this.index;
  }

  private Span end_span(){
    assert this.span_start != null;
    int ss = this.span_start;
    this.span_start = null;
    return new Span(ss, this.index);
  }
}
