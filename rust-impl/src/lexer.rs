#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Span{
    begin : usize,
    end : usize
}

///
/// For use in pattern matching
///
#[macro_export]
macro_rules! tokof {
    ($kind : ident) => (
        $crate::lexer::Lexeme{ kind: $crate::lexer::LexemeKind::$kind, .. }
    );
    ($kind : ident ( $bind : ident )) => (
        $crate::lexer::Lexeme{ kind: $crate::lexer::LexemeKind::$kind($bind), .. }
    )
}

impl Span{
    pub fn new(begin : usize, end : usize) -> Self{
        Span{begin, end}
    }

    pub fn up_to(&self, other : &Span) -> Span{
        Span::new(self.begin, other.end)
    }

    pub fn underline(&self, s : &str, message : Option<&str>) -> String{
        let mut output = String::from(s);
        output += "\n";
        output += &" ".repeat(self.begin);
        output += &"-".repeat(self.end - self.begin);

        if let Some(msg) = message{
            output += " << ";
            output += msg;
        }
        output
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LexemeKind{
    Plus,
    Minus,
    Star,
    Slash,
    Karat,
    Comma,

    Word(String),
    Equals,

    OpenParen,
    CloseParen,

    Number(f64)
}

#[derive(Debug, Clone)]
pub struct Lexeme{
    pub kind : LexemeKind,
    pub span : Span
}

impl Lexeme{
    pub fn new(kind : LexemeKind, span : Span) -> Self{
        Self{kind, span}
    }
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

    pub fn all(mut self) -> Result<Vec<Lexeme>, ()>{
        let mut toks = Vec::new();

        while let Ok(Some(tok)) = self.next(){
            toks.push(tok);
        }

        Ok(toks)
    }

    fn next_span(&mut self, n : usize) -> Span{
        let p = self.cursor;
        self.cursor += n;
        Span::new(p, self.cursor)
    }

    pub fn next(&mut self) -> Result<Option<Lexeme>, ()>{

        // We have already consumed the whole input, return None
        if self.cursor == self.input.len(){
            return Ok(None)
        }

        let all_content = &self.input[self.cursor..];
        let content = all_content.trim_start();

        let ws_count = all_content.len() - content.len();

        self.cursor += ws_count;

        if content.starts_with('('){
            Ok(Some(Lexeme::new(LexemeKind::OpenParen, self.next_span(1))))
        }
        else if content.starts_with(')'){
            Ok(Some(Lexeme::new(LexemeKind::CloseParen, self.next_span(1))))
        }
        else if content.starts_with('*'){
            Ok(Some(Lexeme::new(LexemeKind::Star, self.next_span(1))))
        }
        else if content.starts_with('+'){
            Ok(Some(Lexeme::new(LexemeKind::Plus, self.next_span(1))))
        }
        else if content.starts_with('-'){
            Ok(Some(Lexeme::new(LexemeKind::Minus, self.next_span(1))))
        }
        else if content.starts_with('^'){
            Ok(Some(Lexeme::new(LexemeKind::Karat, self.next_span(1))))
        }
        else if content.starts_with('/'){
            Ok(Some(Lexeme::new(LexemeKind::Slash, self.next_span(1))))
        }
        else if content.starts_with(','){
            Ok(Some(Lexeme::new(LexemeKind::Comma, self.next_span(1))))
        }
        else if content.starts_with('='){
            Ok(Some(Lexeme::new(LexemeKind::Equals, self.next_span(1))))
        }
        else{
            // Try to recognize a literal number or word 
            if content.starts_with(|c : char| c.is_ascii_digit()){
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
                                return Err(())
                            }
                            else{
                                break;
                            }
                        },
                    }

                }

                let num = &self.input[start..self.cursor];
                let num : f64 = num.parse().unwrap();
              
                Ok(Some(Lexeme::new(LexemeKind::Number(num), Span::new(start, self.cursor))))

            }
            else if content.starts_with(|c : char| c.is_alphabetic()){
                // We read in a word
                let start = self.cursor;

                for el in content.chars(){
                    if !el.is_alphabetic(){
                        break;
                    }
                    self.cursor+=1;
                }

                let end = self.cursor;

                let kw = &self.input[start..end];
                
                Ok(Some(Lexeme::new(LexemeKind::Word(kw.to_string()), Span::new(start, self.cursor))))
            }
            else{
                // Unrecognized input
                Err(())
            }
        }

    }
} 
