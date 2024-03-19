package dev.kronsy.ise.epic2.errors;

import dev.kronsy.ise.epic2.text_processing.Span;

public class CalculatorError extends Exception{

  public CalculatorError(Span pos, String message){
    super(message + " @ " + pos);
    this.pos = pos;
    this.message = message;

  }


  public Span pos;
  public String message;
}
