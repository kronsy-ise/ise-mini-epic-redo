use crate::expression::{ArithmeticNode, ParserError as ArithmeticParserError};
use crate::lexer::Lexeme;
use crate::tokof;

#[derive(Debug)]
pub enum Statement{
    Expr(ArithmeticNode),
    Assignment{
        var_name : String,
        expr : ArithmeticNode
    }
}


#[derive(Debug)]
pub enum ParserError{
    Arithmetic(ArithmeticParserError)
}




impl Statement{
    pub fn into_str(&self) -> String{
        match self{
            Self::Expr(e) => e.into_rpn(),
            Self::Assignment { var_name, expr } => format!("{} <- {}", var_name, expr.into_rpn())
        }
    }


    pub fn parse(stmnt : &[Lexeme]) -> Result<Self, ParserError>{
        match stmnt{
            // Assignment 
            [tokof!(Word(var_name)), tokof!(Equals), ..] => {
                // Assigning expression to var_name 

                let expr = &stmnt[2..];

                let expression = ArithmeticNode::parse(expr).map_err(ParserError::Arithmetic)?;

                Ok(Statement::Assignment{
                    var_name: var_name.to_string(),
                    expr : expression
                })
            },

            // Pure expression
            _ => {
                let expression = ArithmeticNode::parse(stmnt).map_err(ParserError::Arithmetic)?;
                Ok(Statement::Expr(expression))
            }
        }
    }
}
