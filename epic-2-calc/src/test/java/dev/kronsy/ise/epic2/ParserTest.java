package dev.kronsy.ise.epic2;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import dev.kronsy.ise.epic2.errors.CalculatorError;
import dev.kronsy.ise.epic2.text_processing.Lexer;
import dev.kronsy.ise.epic2.text_processing.Parser;
import dev.kronsy.ise.epic2.text_processing.Token;
import dev.kronsy.ise.epic2.text_processing.TokenKind;

import java.awt.desktop.AppReopenedListener;
import java.util.ArrayList;
import org.junit.Test;

public class ParserTest{
  ///
  /// Parses the input, returning it in pseudo-RPN
  ///
  private String parse(String input) throws CalculatorError{
    var toks = new Lexer(input).all();
    var parsed = new Parser(toks).parse();
    return parsed.toString();
  }


  @Test 
  public void assignments() throws CalculatorError{
    assertEquals(parse("a = b"), "SET a = v:b");
    assertEquals(parse("foo = 1 + 1"), "SET foo = ( 1.0 1.0 + )");
  }

  @Test 
  public void expressions() throws CalculatorError{
    assertEquals(parse("8 / 2 * (2 + 2)"), "( ( 8.0 2.0 / ) ( 2.0 2.0 + ) * )");

    assertEquals(parse("sqrt(21 + 4)"), "( ( 21.0 4.0 + ) >| sqrt )");
  }
}
