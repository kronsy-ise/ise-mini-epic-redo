package dev.kronsy.ise.epic2.text_processing;


public class Token{
  public TokenKind kind;
  public String value;
  public Span location;


  public Token(TokenKind kind, String value, Span location){
    this.kind = kind;
    this.value = value;
    this.location = location;
  }


  public boolean equals(Object o){
    Token t = (Token)o;
    return t.kind.equals(this.kind) && t.value.equals(this.value);
  }
}
