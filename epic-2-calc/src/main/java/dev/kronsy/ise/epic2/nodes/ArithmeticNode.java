package dev.kronsy.ise.epic2.nodes;

import dev.kronsy.ise.epic2.calculation.ExecutionEngine;
import dev.kronsy.ise.epic2.calculation.VariableStore;
import dev.kronsy.ise.epic2.errors.CalculatorError;
import dev.kronsy.ise.epic2.text_processing.Span;

public abstract class ArithmeticNode extends Statement{

  public ArithmeticNode(Span span){
    super(span);
  }

  /**
   * Resolve this arithmetic node, returning its resolved value as a 
   * Double Floating Point Number 
   */
  public abstract Double resolve(VariableStore vars) throws CalculatorError;

  public void execute(ExecutionEngine e) throws CalculatorError{
    var result = this.resolve(e.variables);
    e.display_result(result);
  }

  public String toString(){
    return this.into_rpn();
  }


  /**
   *
   * Rewrite this arithmetic operation in reverse polish notation 
   *
   */
  public abstract String into_rpn();
}
