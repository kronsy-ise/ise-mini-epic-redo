package dev.kronsy.ise.epic2.text_processing;

import java.lang.reflect.Array;
import java.util.ArrayList;
import java.util.List;

import dev.kronsy.ise.epic2.errors.CalculatorError;
import dev.kronsy.ise.epic2.nodes.ArithmeticNode;
import dev.kronsy.ise.epic2.nodes.Assignment;
import dev.kronsy.ise.epic2.nodes.BinaryNode;
import dev.kronsy.ise.epic2.nodes.BinaryOperator;
import dev.kronsy.ise.epic2.nodes.FunctionNode;
import dev.kronsy.ise.epic2.nodes.LiteralNode;
import dev.kronsy.ise.epic2.nodes.Statement;
import dev.kronsy.ise.epic2.nodes.VariableNode;

public class Parser{
  
  public Parser(ArrayList<Token> toks){
    this.tokens = toks;
  }


  public ArrayList<Token> tokens;

  enum BinOp{
    Exp,
    Mul,
    Div,
    Add,
    Sub,
  }

  class Splitpoint{
    int index;
    BinOp operator;
    Token tok;

    Splitpoint(int index, BinOp op, Token tok){
      this.index = index;
      this.operator = op;
      this.tok = tok;
    }
  }
  private static BinOp get_binop(TokenKind tk){
    return switch(tk){
      case Plus -> Parser.BinOp.Add;
      case Minus -> Parser.BinOp.Sub;
      case Star -> Parser.BinOp.Mul;
      case Slash -> Parser.BinOp.Div;
      case Karat -> Parser.BinOp.Exp;
      default -> null;
    };
  }

  private static int get_precedence(BinOp op){
   return switch(op){
      case Add -> 0;
      case Sub -> 0;
      case Mul -> 1;
      case Div -> 1;
      case Exp -> 2;
    };
  }

  private ArrayList<ArrayList<Token>> split_at(List<Token> tokens, TokenKind tk) throws CalculatorError{
    int indentation = 0;
    ArrayList<ArrayList<Token>> segments = new ArrayList<>();
    ArrayList<Token> current = new ArrayList<>();
    for(int i = 0; i < tokens.size(); i++){
      Token t = tokens.get(i);


      switch(t.kind){
        case OpenParen:
          current.add(t);
          indentation++;
          break;
        case CloseParen:
          current.add(t);
          if(indentation == 0){
            throw new CalculatorError(t.location, "Redundant Closing Parenthesis");
          }
          indentation--;
          break;
        default:
          if(t.kind == tk && indentation == 0){
            // We have a segment
            segments.add(current);
            current = new ArrayList<>();
          }
          else{
            current.add(t);
          }
          break;
      }
    }
    segments.add(current);
    return segments;
  }

  private ArithmeticNode parse_expr(List<Token> toks) throws CalculatorError{
    if(toks.isEmpty()) throw new RuntimeException("Empty token list to parse");
    Span subexpr_span = toks.getFirst().location.up_to_end(toks.getLast().location);

    if(toks.size() == 1){
      // single token 
      Token t = toks.getFirst();

      switch(t.kind){
        case Number:
          return new LiteralNode(Double.parseDouble(t.value), t.location);
        case Word:
          return new VariableNode(t.value, t.location);
        default:
          throw new RuntimeException();
      }
    }


    Parser.Splitpoint best_split = null;
    int indentation = 0;
    int toplevel_count = 0;
    boolean prev_is_op = false;
    ArrayList<Token> toplevel_tokens = new ArrayList<>();
    for(int index = 0; index < toks.size(); index++){
      Token tok = toks.get(index);
      if(indentation == 0){
        toplevel_count++;
        toplevel_tokens.add(tok);
      }
      switch(tok.kind){
        case OpenParen:
          indentation++;
          break;
        case CloseParen:
          if(indentation == 0){
            // TODO: Redundant closing parenthesis 
            throw new CalculatorError(tok.location, "Redundant Closing Parenthesis");
          }
          indentation--;
          break;
        default:
          // We only process binary operators at the top level
          if(indentation == 0){
            BinOp op = Parser.get_binop(tok.kind);
            if(op == null){
              // We encountered a non binary operator token
              prev_is_op = false;
              continue;
            }
            if(prev_is_op){
              continue;
          }
            prev_is_op = true;
            if(best_split == null){
              best_split = new Splitpoint(index, op, tok);
            }
            else if(Parser.get_precedence(op) <= Parser.get_precedence(best_split.operator)){
              best_split = new Splitpoint(index, op, tok);
            }
          }
      }
    }

    if(indentation > 0){
      throw new CalculatorError(subexpr_span.immediately_after(), "Missing Closing Parenthesis");
    }

    if(best_split == null){
      // We havent a split, this happens when:
      // - Our expression is fully parenthesized (only when toplevel_count = 1 [first open paren])
      // - Our expression is the invocation of a function (i.e sqrt) (toplevel_count = 2 [first open paren + function name])
      // - An invalid expression is trying to be passed in as valid
      Token first = toks.getFirst();
      Token second = toks.get(1);
      Token last = toks.getLast();

      if(first.kind == TokenKind.OpenParen && last.kind == TokenKind.CloseParen){
        if(toplevel_count != 1){
          Token interrupter = toplevel_tokens.get(1);
          // TODO: Something interrupting the brackets
          throw new CalculatorError(interrupter.location, "Bracket interrupt");
        }
        var center = toks.subList(1, toks.size() - 1);
        return parse_expr(center); 
      }
      else if(first.kind == TokenKind.Word && second.kind == TokenKind.OpenParen && last.kind == TokenKind.CloseParen){
        if(toplevel_count != 2){
          Token interrupter = toplevel_tokens.get(2);
          throw new CalculatorError(interrupter.location, "Function Call Bracket Interrupt");
        }
        String fn_name = first.value;
        var arguments = toks.subList(2, toks.size()-1);
        var split_arguments = split_at(arguments, TokenKind.Comma);
        ArrayList<ArithmeticNode> processed_arguments = new ArrayList<>();

        for(var a : split_arguments){
          processed_arguments.add(parse_expr(a));
        }
        return new FunctionNode(new VariableNode(fn_name, first.location), processed_arguments, subexpr_span);
      }
      else{
        throw new CalculatorError(subexpr_span, "Malformed Expression");
      }
      
    }
    else{
      // We have a split
      var lhs = toks.subList(0, best_split.index);
      var rhs = toks.subList(best_split.index + 1, toks.size());

      if(rhs.isEmpty()){
        throw new CalculatorError(best_split.tok.location.immediately_after(), "Expected Right Hand Expression");
      }

      if(lhs.isEmpty()){
        // Check if its a prefix operator (such as -X or +X)
        switch(best_split.operator){
          case Add:
            return this.parse_expr(rhs); 
          case Sub:
            return new BinaryNode(BinaryOperator.Sub, new LiteralNode(0d, best_split.tok.location), this.parse_expr(rhs));
          default:
            System.out.println(best_split.tok.location);
            throw new CalculatorError(best_split.tok.location, "Unsupported prefix operator");
        }
      }

      var left_subtree = this.parse_expr(lhs);
      var right_subtree = this.parse_expr(rhs);

      BinaryOperator op = switch(best_split.operator){
        case Add -> BinaryOperator.Add;
        case Sub -> BinaryOperator.Sub;
        case Mul -> BinaryOperator.Mul;
        case Div -> BinaryOperator.Div;
        case Exp -> BinaryOperator.Pow;
      };

      return new BinaryNode(op, left_subtree, right_subtree);
    }

    
  

  }

  public Statement parse() throws CalculatorError{

    if(this.tokens.size() > 2){
      var first = tokens.get(0);
      var second = tokens.get(1);

      // We have an assignment statement
      if(first.kind == TokenKind.Word && second.kind == TokenKind.Equals){
        String var_name = first.value;
        var expr = tokens.subList(2, tokens.size());
        var assignment_value = parse_expr(expr);

        return new Assignment(new VariableNode(var_name, first.location), assignment_value);
      }
    }

    return this.parse_expr(this.tokens);
  }
}
