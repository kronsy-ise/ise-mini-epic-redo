package dev.kronsy.ise.epic2.nodes;

import dev.kronsy.ise.epic2.calculation.ExecutionEngine;
import dev.kronsy.ise.epic2.errors.CalculatorError;
import dev.kronsy.ise.epic2.text_processing.Span;

public class Assignment extends Statement{

  public Assignment(VariableNode var_name, ArithmeticNode assigned_value){
    super(var_name.span.up_to_end(assigned_value.span));
    this.target_value = var_name;
    this.assigned_value = assigned_value;
  }


  public VariableNode target_value;
  public ArithmeticNode assigned_value;
  public Span span;

  public String toString(){
    return "SET '" + this.target_value + "' = " + this.assigned_value.into_rpn();
  }


  public void execute(ExecutionEngine e) throws CalculatorError{
    Double result = this.assigned_value.resolve(e.variables);
    e.variables.set(this.target_value.var_name, result);
    e.feedback_confirm();
  }
}
