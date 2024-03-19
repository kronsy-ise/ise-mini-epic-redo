package dev.kronsy.ise.epic2.calculation;


public class ExecutionEngine{


  public ExecutionEngine(){
    this.variables = new VariableStore();
    this.variables.inject_constants();
  }

  public VariableStore variables;

  public void display_result(Double result){
    String cyan = "\033[0;36m";
    String reset = "\033[0;0m";
    System.out.println(cyan+"Result  > "+reset + result + "\n");
  }

  public void feedback_confirm(){
    System.out.println();
  }
}
