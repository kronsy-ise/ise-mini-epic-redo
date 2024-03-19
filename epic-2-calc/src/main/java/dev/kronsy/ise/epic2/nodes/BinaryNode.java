package dev.kronsy.ise.epic2.nodes;

import dev.kronsy.ise.epic2.calculation.VariableStore;
import dev.kronsy.ise.epic2.errors.CalculatorError;
import dev.kronsy.ise.epic2.text_processing.Span;

public class BinaryNode extends ArithmeticNode{

  public BinaryNode(BinaryOperator op, ArithmeticNode lhs, ArithmeticNode rhs){
    super(lhs.span.up_to_end(rhs.span));
    this.operator = op;
    this.lhs = lhs;
    this.rhs = rhs;
  }


  public BinaryOperator operator;

  public ArithmeticNode lhs;
  public ArithmeticNode rhs;


  public String into_rpn(){
    return "( " + this.lhs.into_rpn() + " " + this.rhs.into_rpn() + " " + this.opname() + " )";
  }

  public Double resolve(VariableStore vars) throws CalculatorError{

    var left = this.lhs.resolve(vars);
    var right = this.rhs.resolve(vars);

    return this.apply(left, right);
  }

  private String opname(){
    return switch(this.operator){
      case Add -> "+";
      case Sub -> "-";
      case Mul -> "*";
      case Div -> "/";
      case Pow -> "^";
    };
  }

  private Double apply(Double o1, Double o2){
    return switch(this.operator){
      case Add -> o1 + o2;
      case Sub -> o1 - o2;
      case Mul -> o1 * o2;
      case Div -> o1 / o2;
      // o1 to the power of o2
      case Pow -> Math.pow(o1, o2);
    };
  }
}
