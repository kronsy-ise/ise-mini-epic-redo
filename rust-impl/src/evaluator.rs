use crate::lexer::Span;

pub struct Evaluator{
    variables : std::collections::HashMap<String, f64>
}

#[derive(Debug)]
pub enum ArithmeticError{
    DivideByZero(Span),
    InfinityDivision(Span),
    LogZero(Span),
    UndefinedVariable(String, Span)
}

impl ArithmeticError{
    pub fn span(&self) -> &Span{
        match self{
            Self::DivideByZero(span)
            | Self::InfinityDivision(span)
            | Self::LogZero(span)
            | Self::UndefinedVariable(_, span) => span
        }
    }

    pub fn contextualize(&self, source : &str) -> String{
        self.span().underline(source, Some(&self.message()))
    }
    pub fn message(&self) -> String{
        match self{
            Self::DivideByZero(_) => "Division by Zero".into(),
            Self::InfinityDivision(_) => "Undefined Infinite Division".into(),
            Self::LogZero(_) => "Logarithm of Zero".into(),
            Self::UndefinedVariable(name, _) => format!("Undefined variable: {name:?}")
        }
    }
}

impl Evaluator{
    pub fn new() -> Self{
        let mut vars = std::collections::HashMap::new();
        vars.insert("PI".to_string(), std::f64::consts::PI);
        vars.insert("E".to_string(), std::f64::consts::E);
        vars.insert("INF".to_string(), std::f64::INFINITY);
        Self{
            variables: vars
        }
    }

    pub fn get_variable(&self, name : &str) -> Option<f64>{
        self.variables.get(name).copied()
    }

    pub fn set_variable(&mut self, name : String, val : f64){
        self.variables.insert(name, val);
    }



    pub fn eval_expression(&self, expr : &crate::expression::ArithmeticNode) -> Result<f64, ArithmeticError>{
        use crate::expression::ArithmeticNodeKind;
        match &expr.kind{
            ArithmeticNodeKind::Add(a, b) => Ok(self.eval_expression(a)? + self.eval_expression(b)?),
            ArithmeticNodeKind::Sub(a, b) => Ok(self.eval_expression(a)? - self.eval_expression(b)?),
            ArithmeticNodeKind::Mul(a, b) => Ok(self.eval_expression(a)? * self.eval_expression(b)?),
            ArithmeticNodeKind::Sin(a) => Ok(self.eval_expression(a)?.sin()),
            ArithmeticNodeKind::Cos(a) => Ok(self.eval_expression(a)?.cos()),
            ArithmeticNodeKind::Tan(a) => Ok(self.eval_expression(a)?.tan()),
            ArithmeticNodeKind::Deg(a) => Ok(self.eval_expression(a)?.to_radians()),
            ArithmeticNodeKind::Div(a, b) => {
                let num = self.eval_expression(a)?;
                let denom = self.eval_expression(b)?;

                if denom == 0.0{
                    Err(ArithmeticError::DivideByZero(expr.span.clone()))
                }
                else if num.is_infinite() && denom.is_infinite(){
                    Err(ArithmeticError::InfinityDivision(expr.span.clone()))
                }
                else{
                    Ok(num / denom)
                }
            },
            ArithmeticNodeKind::Pow(a, b) => Ok(self.eval_expression(a)?.powf(self.eval_expression(b)?)),
            ArithmeticNodeKind::Root(a, b) => Ok( self.eval_expression(b)?.powf(1.0/self.eval_expression(a)?) ),
            ArithmeticNodeKind::Log(a, b) => {
                let base = self.eval_expression(a)?;
                let value = self.eval_expression(b)?;

                if value == 0.0{
                    Err(ArithmeticError::LogZero(expr.span.clone()))
                }
                else{
                    Ok(value.log(base))
                }
            },
            ArithmeticNodeKind::Neg(a) => Ok(-self.eval_expression(a)?),
            ArithmeticNodeKind::Literal(l) => Ok(*l),
            ArithmeticNodeKind::Variable(name) => {
                let v = self.get_variable(name);
                match v{
                    Some(v) => Ok(v),
                    None => Err(ArithmeticError::UndefinedVariable(name.clone(), expr.span.clone()))
                }
            }
        }
    }

    pub fn eval_statement(&mut self, stmnt : &crate::statement::Statement) -> Result<(), ArithmeticError>{
        use crate::statement::Statement;
        match stmnt{
            Statement::Assignment { var_name, expr } => {
                let val = self.eval_expression(expr)?;
                println!("EVAL: Setting '{var_name}' = {val}");
                self.set_variable(var_name.to_string(), val);
            },
            Statement::Expr(e) => {
                let res = self.eval_expression(e)?;
                println!("EVAL: Expression resolves to {res}");
            }
        };

        Ok(())
    }
}
