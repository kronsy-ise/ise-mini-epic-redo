package dev.kronsy.ise.epic2.util;




public class Color{
  public static String red(String text){
    return "\033[0;31m" + text + "\033[0;0m";
  }
  public static String blue(String text){
    return "\033[0;34m" + text + "\033[0;0m";
  }
  public static String yellow(String text){
    return "\033[0;33m" + text + "\033[0;0m";
  }

  public static String green(String text){
    return "\033[0;32m" + text + "\033[0;0m";
  }

  public static String cyan(String text){
    return "\033[0;36m" + text + "\033[0;0m";
  }
}
