package dev.kronsy.ise.epic2.nodes;

import dev.kronsy.ise.epic2.calculation.ExecutionEngine;
import dev.kronsy.ise.epic2.errors.CalculatorError;
import dev.kronsy.ise.epic2.text_processing.Span;

public abstract class Statement{

  public Statement(Span span){
    this.span = span;
  }


  public Span span;

  public abstract void execute(ExecutionEngine engine) throws CalculatorError; 
  public abstract String toString();
}
