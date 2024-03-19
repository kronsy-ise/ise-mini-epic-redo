package dev.kronsy.ise.epic2.nodes;

import dev.kronsy.ise.epic2.calculation.VariableStore;
import dev.kronsy.ise.epic2.errors.CalculatorError;
import dev.kronsy.ise.epic2.text_processing.Span;

public class LiteralNode extends ArithmeticNode{

  public LiteralNode(Double value, Span span){
    super(span);
    this.value = value;
  }


  public Double value;

  public String into_rpn(){
    return this.value.toString();
  }

  public Double resolve(VariableStore vars) throws CalculatorError{
    return this.value;
  }
}
