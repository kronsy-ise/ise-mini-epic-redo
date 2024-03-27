package dev.kronsy.ise.epic2;

import static org.junit.Assert.assertTrue;

import dev.kronsy.ise.epic2.errors.CalculatorError;
import dev.kronsy.ise.epic2.text_processing.Lexer;
import dev.kronsy.ise.epic2.text_processing.Token;
import dev.kronsy.ise.epic2.text_processing.TokenKind;
import java.util.ArrayList;
import org.junit.Test;

public class LexerTest {

  private ArrayList<Token> lexify(String input) throws CalculatorError {
    return new Lexer(input).all();
  }

  private ArrayList<Token> toks(Token... toks) {
    var l = new ArrayList<Token>();

    for (var t : toks) {
      l.add(t);
    }

    return l;
  }

  @Test
  public void shouldBeEmpty() throws CalculatorError {
    assertTrue(this.lexify("").equals(new ArrayList<Token>()));
    assertTrue(this.lexify(" ").equals(new ArrayList<Token>()));
    assertTrue(this.lexify("  ").equals(new ArrayList<Token>()));
    assertTrue(this.lexify("\t").equals(new ArrayList<Token>()));
    assertTrue(this.lexify(" \t").equals(new ArrayList<Token>()));
    assertTrue(!this.lexify("foo").equals(new ArrayList<Token>()));
  }

  @Test
  public void numbers() throws CalculatorError {
    assertTrue(
        this.lexify("1").equals(toks(new Token(TokenKind.Number, "1", null))));
    assertTrue(this.lexify("1 2 3").equals(
        toks(new Token(TokenKind.Number, "1", null),
            new Token(TokenKind.Number, "2", null),
            new Token(TokenKind.Number, "3", null))));

    assertTrue(this.lexify("123").equals(
        toks(new Token(TokenKind.Number, "123", null))));

    assertTrue(this.lexify("4.729").equals(
        toks(new Token(TokenKind.Number, "4.729", null))));
  }

  @Test
  public void words() throws CalculatorError {
    assertTrue(this.lexify("foo").equals(
        toks(new Token(TokenKind.Word, "foo", null))));
    assertTrue(this.lexify("foobar foo bar")
        .equals(toks(new Token(TokenKind.Word, "foobar", null),
            new Token(TokenKind.Word, "foo", null),
            new Token(TokenKind.Word, "bar", null))));
  }

  @Test
  public void breaks() throws CalculatorError {
    assertTrue(
        this.lexify("a+b").equals(toks(new Token(TokenKind.Word, "a", null),
            new Token(TokenKind.Plus, "+", null),
            new Token(TokenKind.Word, "b", null))));

    assertTrue(lexify("-b a+4").equals(toks(
      new Token(TokenKind.Minus, "-", null),
      new Token(TokenKind.Word, "b", null),
      new Token(TokenKind.Word, "a", null),
      new Token(TokenKind.Plus, "+", null),
      new Token(TokenKind.Number, "4", null)
    )));
  }
}
