package dev.kronsy.ise.epic2.nodes;

import dev.kronsy.ise.epic2.calculation.VariableStore;
import dev.kronsy.ise.epic2.errors.CalculatorError;
import dev.kronsy.ise.epic2.text_processing.Span;

public class VariableNode extends ArithmeticNode{

  public VariableNode(String name, Span span){
    super(span);
    this.var_name = name;
  }

  public String var_name;



  public String into_rpn(){
    return "v:"+var_name;
  }

  public Double resolve(VariableStore vars) throws CalculatorError{
    Double value = vars.get(this.var_name);

    if(value == null){
      throw new CalculatorError(this.span, "Undefined variable '"+this.var_name+"'");
    }
    return value;
  }
}
