package dev.kronsy.ise.epic2.text_processing;


public class Span{
  public int begin;
  public int end;


  public Span(int begin, int end){
    assert begin <= end;

    this.begin = begin;
    this.end = end;
  }


  /**
   * Create a new span from the beginning of this span to the 
   * end of the other span
   */ 
  public Span up_to_end(Span other){
    return new Span(this.begin, other.end);
  }

  public Span from_end_to_end(Span other){
    return new Span(this.end, other.end);
  }

  public Span immediately_after(){
    return new Span(this.end-1, this.end+1);
  }

  public String toString(){
    return ""+this.begin+"->"+this.end;
  }

  public int width(){
    return this.end - this.begin;
  }
}
