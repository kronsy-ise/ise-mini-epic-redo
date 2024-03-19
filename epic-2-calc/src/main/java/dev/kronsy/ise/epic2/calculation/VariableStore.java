package dev.kronsy.ise.epic2.calculation;

import java.util.HashMap;


public class VariableStore{
  HashMap<String, Double> variables;


  public VariableStore(){
    this.variables = new HashMap<>();
  }

  public void inject_constants(){
    this.set("pi", Math.PI);
    this.set("e", Math.E);
    this.set("inf", Double.POSITIVE_INFINITY);
    this.set("nan", Double.NaN);
    this.set("tau", Math.PI * 2);
  }



  public void set(String name, Double value){
    this.variables.put(name, value);
  }

  public Double get(String var){
    return this.variables.get(var);
  }
}
