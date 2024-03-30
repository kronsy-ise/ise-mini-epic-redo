package dev.kronsy.ise.epic2.nodes;

import java.util.ArrayList;

import dev.kronsy.ise.epic2.calculation.VariableStore;
import dev.kronsy.ise.epic2.errors.CalculatorError;
import dev.kronsy.ise.epic2.text_processing.Span;

public class FunctionNode extends ArithmeticNode{

  private interface Cb{
    Double call(Span s, Double[] args) throws CalculatorError;
  }

  private class FnDecl{
    String name;
    int arg_cnt;
    Cb handler;

    FnDecl(String name, int ac, Cb handle){
      this.name = name;
      this.arg_cnt = ac;
      this.handler = handle;
    }
  }

  /**
   *
   * Declarations of built-in functions in the mathematical evaluator
   *
   * funcname, arg_count, hanlder(span, arguments[arg_count]) 
   */
  final FnDecl[] funcs = {
    new FnDecl("sin", 1, (s, a) -> Math.sin(a[0])),
    new FnDecl("cos", 1, (s, a) -> Math.cos(a[0])),
    new FnDecl("tan", 1, (s, a) -> Math.tan(a[0])),
    new FnDecl("log", 2, (s, a) -> Math.log(a[1]) / Math.log(a[0])),
    new FnDecl("root", 2, (s, a) -> Math.pow(a[1], 1/a[0])),
    new FnDecl("sqrt", 1, (s, a) -> Math.sqrt(a[0])),
    // Convert degrees to radians
    new FnDecl("deg", 1, (s, a) -> a[0]/360 * 2 * Math.PI),
    // Convert radians to degrees 
    new FnDecl("rad", 1, (s, a) -> a[0] / (2*Math.PI) * 360),

  };

  public FunctionNode(VariableNode name, ArrayList<ArithmeticNode> args, Span span){
    super(span);
    this.name = name;
    this.arguments = args;
  }

  public VariableNode name;
  public ArrayList<ArithmeticNode> arguments;


  public String into_rpn(){
    String rep = "( ";
    for(var a : arguments){
      rep += a.into_rpn() + " ";
    }
    rep += ">| " + this.name.var_name + " )";
    return rep;
  }

  public Double resolve(VariableStore vars) throws CalculatorError{
    for(var d : funcs){
      if(d.name.equals(this.name.var_name)){
        if(this.arguments.size() != d.arg_cnt){
          throw new CalculatorError(this.name.span.from_end_to_end(this.span), "Bad call to '"+d.name+"', expected " + d.arg_cnt + " arguments but got " + this.arguments.size());
        }
        Double[] values = new Double[d.arg_cnt];
        for(int i = 0; i < d.arg_cnt; i++){
          Double value = this.arguments.get(i).resolve(vars);
          values[i] = value;
        }
        Double resolved = d.handler.call(this.span, values);
        return resolved;
      }
    }
    throw new CalculatorError(this.name.span, "Call to non-existent function '"+this.name.var_name+"'");
  }
}
