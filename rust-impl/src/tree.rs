pub enum ArithmeticNode{
    Add(NodeRef, NodeRef),
    Sub(NodeRef, NodeRef),
    Mul(NodeRef, NodeRef),
    Div(NodeRef, NodeRef),
    Pow(NodeRef, NodeRef),
    Root(NodeRef, NodeRef),
    Neg(NodeRef),
    Literal(f64),
}


#[derive(Clone, Debug)]
pub enum Lexeme{
    Plus,
    Minus,
    Star,
    Slash,
    Karat,

    Word(String),

    OpenParen,
    CloseParen,

    Number(f64)
}



pub struct Lexer{
    input : String,
    cursor : usize
}

impl Lexer{
    pub fn new(input : String) -> Self{
        Self{
            input,
            cursor: 0
        }
    }

    pub fn next(&mut self) -> Result<Lexeme, ()>{
        let all_content = &self.input[self.cursor..];
        let content = all_content.trim_start();

        let ws_count = all_content.len() - content.len();

        self.cursor += ws_count;

        if content.starts_with("("){
            self.cursor += 1;
            Ok(Lexeme::OpenParen)
        }
        else if content.starts_with(")"){
            self.cursor += 1;
            Ok(Lexeme::CloseParen)
        }
        else if content.starts_with("*"){
            self.cursor += 1;
            Ok(Lexeme::Star)
        }
        else if content.starts_with("+"){
            self.cursor += 1;
            Ok(Lexeme::Plus)
        }
        else if content.starts_with("-"){
            self.cursor += 1;
            Ok(Lexeme::Minus)
        }
        else if content.starts_with("^"){
            self.cursor += 1;
            Ok(Lexeme::Karat)
        }
        else if content.starts_with("/"){
            self.cursor += 1;
            Ok(Lexeme::Slash)
        }
        else{
            // Try to recognize a literal number or word 

            if content.starts_with(|c| c.is_digit()){
                // Parse a digit

                let mut has_enc_dot = false;
                let start = self.cursor;
                for el in content.chars(){

                    // We have just encountered a dot
                    let needs_digit = has_enc_dot && &self.input[self.cursor..=self.cursor] == ".";

                    match el{
                        '.' => {
                            if has_enc_dot{ 
                                break
                            }
                            has_enc_dot = true;
                            self.cursor+=1;
                        },
                        '0'..='9' => {
                            self.cursor+=1;
                        },
                        _ => {
                            if needs_digit{
                                self.cursor++;
                                return Err(())
                            }
                            else{
                                
                            }
                        },
                    }

                }

                let num = &self.input[start..self.cursor];
               
                f64::try_from
                todo!()

            }
            else if content.starts_with(|c| c.is_alphabetic()){
                todo!()
            }
            else{
                Err(())
            }
        }

    }
} 



// Binary Operator precedence
// + => 1
// - => 1 
// * => 2
// / => 2
// 
// ^ => 3
//
// sqrt() is a function, so precedence does not matter 
//
// negation is evaluated before anything
//



type NodeRef = Box<ArithmeticNode>;


pub enum ArithmeticError{
    DivideByZero,
    InfinityDivision
}

pub enum ParserError{

}


impl ArithmeticNode{
    pub fn eval(&self) -> Result<f64, ArithmeticError>{
        match self{
            Self::Add(a, b) => Ok(a.eval()? + b.eval()?),
            Self::Sub(a, b) => Ok(a.eval()? - b.eval()?),
            Self::Mul(a, b) => Ok(a.eval()? * b.eval()?),
            Self::Div(a, b) => {
                let num = a.eval()?;
                let denom = b.eval()?;

                if denom == 0.0{
                    Err(ArithmeticError::DivideByZero)
                }
                else if num.is_infinite() && denom.is_infinite(){
                    Err(ArithmeticError::InfinityDivision)
                }
                else{
                    Ok(num / denom)
                }
            },
            Self::Pow(a, b) => Ok(a.eval()?.powf(b.eval()?)),
            Self::Root(a, b) => Ok( b.eval()?.powf(1.0/a.eval()?) ),
            Self::Neg(a) => Ok(-a.eval()?),
            Self::Literal(l) => Ok(*l)
        }
    }



    pub fn parse(expr : &[Lexeme]) -> Result<ArithmeticNode, ParserError>{

        // We use the following algorithm in order to parse the expression
        //
        // 1. Find "split points"
        // 2. Find the final "split point" with the lowest priority, (construciton happens in
        //    reverse)
        // 3. Subparse both sides of the split point

        struct SplitPoint{
            idx : usize,
            tok : Lexeme
        }

        impl SplitPoint{
            fn new(idx : usize, tok : Lexeme) -> Self{
                Self{
                    tok, idx
                }
            }

            fn priority(&self) -> u8{
                match self.tok{
                    Lexeme::Karat => 2,
                    Lexeme::Star => 1,
                    Lexeme::Slash => 1,
                    Lexeme::Plus => 0,
                    Lexeme::Minus => 0
                }
            }
        }

        let mut split_points = Vec::<SplitPoint>::new();


        let mut ind : usize = 0;

        for (idx, l) in expr.iter().enumerate(){
          
            
            if ind == 0{
                match l{
                    Lexeme::Plus => {
                        split_points.push(SplitPoint::new(idx, Lexeme::Plus));
                    },
                    Lexeme::Minus => {
                        split_points.push(SplitPoint::new(idx, Lexeme::Minus));
                    },
                    Lexeme::Star => {
                        split_points.push(SplitPoint::new(idx, Lexeme::Star))
                    },
                    Lexeme::Slash => {
                        split_points.push(SplitPoint::new(idx, Lexeme::Slash))
                    },
                    Lexeme::OpenParen => {
                        ind += 1;
                    },
                    Lexeme::CloseParen => {
                        todo!("Handle Me!")
                    },
                    _ => ()
                }
            }
            else{
                match l{
                    Lexeme::OpenParen => {
                        ind += 1;
                    },
                    Lexeme::CloseParen => {
                        ind -= 1;
                    },
                    _ => ()
                }
            }

        }

        let target_split = split_points.iter().enumerate().max_by(|x, y|{
            let px = x.1.priority();
            let py = y.1.priority();

            let cmp = px.cmp(&py);

            match cmp{
                std::cmp::Ordering::Equal => {
                    x.0.cmp(&y.0)
                },
                _ => cmp
            }
        });

        
        match target_split{
            Some((_, split)) => {
                let (lhs, mut rhs) = expr.split_at(split.idx);
                rhs = &rhs[1..];
            },
            None => {
                // There is no split point, this occurs in the following cases:
                // - Our expression is fully parenthesized
                // - Our expression is a literal 
                // - Our expression is the invocation of a function (i.e sqrt)
            }
        }

        todo!()
    }
}
