use crate::lexer::{Lexeme, LexemeKind, Span};
use crate::tokof;

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

#[derive(Debug)]
pub enum ArithmeticNodeKind {
    Add(NodeRef, NodeRef),
    Sub(NodeRef, NodeRef),
    Mul(NodeRef, NodeRef),
    Div(NodeRef, NodeRef),
    Pow(NodeRef, NodeRef),
    Root(NodeRef, NodeRef),
    // Log(base, value)
    Log(NodeRef, NodeRef),
    Sin(NodeRef),
    Cos(NodeRef),
    Tan(NodeRef),
    /// Convert degrees to rads
    Deg(NodeRef),
    Neg(NodeRef),
    Literal(f64),
    Variable(String),
}

#[derive(Debug)]
pub struct ArithmeticNode {
    pub span: Span,
    pub kind: ArithmeticNodeKind,
}

type NodeRef = Box<ArithmeticNode>;

impl ArithmeticNode {
    pub fn new(kind: ArithmeticNodeKind, span: Span) -> Self {
        Self { kind, span }
    }

    ///
    /// Convert this node into a Reverse Polish Notation String
    ///
    pub fn into_rpn(&self) -> String {
        match &self.kind {
            ArithmeticNodeKind::Add(a, b) => format!("({} {} +)", a.into_rpn(), b.into_rpn()),
            ArithmeticNodeKind::Sub(a, b) => format!("({} {} -)", a.into_rpn(), b.into_rpn()),
            ArithmeticNodeKind::Mul(a, b) => format!("({} {} *)", a.into_rpn(), b.into_rpn()),
            ArithmeticNodeKind::Div(a, b) => format!("({} {} /)", a.into_rpn(), b.into_rpn()),
            ArithmeticNodeKind::Pow(a, b) => format!("({} {} ^)", a.into_rpn(), b.into_rpn()),
            ArithmeticNodeKind::Neg(a) => format!("({} -)", a.into_rpn()),
            ArithmeticNodeKind::Sin(a) => format!("({} sin)", a.into_rpn()),
            ArithmeticNodeKind::Cos(a) => format!("({} cos)", a.into_rpn()),
            ArithmeticNodeKind::Tan(a) => format!("({} tan)", a.into_rpn()),
            ArithmeticNodeKind::Deg(a) => format!("({} deg)", a.into_rpn()),
            ArithmeticNodeKind::Root(a, b) => format!("({} {} nrt)", a.into_rpn(), b.into_rpn()),
            ArithmeticNodeKind::Log(a, b) => format!("({} {} log)", a.into_rpn(), b.into_rpn()),
            ArithmeticNodeKind::Literal(l) => format!("{}", l),
            ArithmeticNodeKind::Variable(v) => v.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    InvalidUnaryOperator(BinOp),
    InvalidArgumentCount { expected: usize, got: usize },
    UnexpectedToken(Lexeme),
    UnrecognizedFunction(String, Span),
}

impl ArithmeticNode {
    ///
    /// Split the given expression at the separator
    ///
    /// takes indentation into account
    ///
    fn split_at<'a>(expr: &'a [Lexeme], sep: &LexemeKind) -> Vec<&'a [Lexeme]> {
        let mut ind = 0;
        let mut ret = Vec::new();

        let mut item_start = 0;

        for (i, tok) in expr.iter().enumerate() {
            match &tok.kind {
                LexemeKind::OpenParen => ind += 1,
                LexemeKind::CloseParen => ind -= 1,
                l if l == sep && ind == 0 => {
                    // We encounter the separator at the root level
                    let slice = &expr[item_start..i];
                    ret.push(slice);
                    item_start = i + 1;
                }
                // Just another content token, do nothing
                _ => (),
            }
        }

        // Account for final element

        if item_start < expr.len() {
            ret.push(&expr[item_start..]);
        }

        ret
    }

    pub fn parse(expr: &[Lexeme]) -> Result<ArithmeticNode, ParserError> {
        let span = expr[0].span.up_to(&expr[expr.len() - 1].span);
        // Handle a one-element expr
        if let [lex] = expr {
            return match &lex.kind {
                LexemeKind::Number(val) => {
                    Ok(ArithmeticNode::new(ArithmeticNodeKind::Literal(*val), span))
                }
                LexemeKind::Word(val) => Ok(ArithmeticNode::new(
                    ArithmeticNodeKind::Variable(val.clone()),
                    span,
                )),
                _ => Err(ParserError::UnexpectedToken(lex.clone())),
            };
        }

        // We use the following algorithm in order to parse the expression
        //
        // 1. Find "split points"
        // 2. Find the final "split point" with the lowest priority, (construciton happens in
        //    reverse)
        // 3. Subparse both sides of the split point

        #[derive(Debug)]
        struct SplitPoint {
            idx: usize,
            op: BinOp,
        }

        impl SplitPoint {
            fn new(idx: usize, op: BinOp) -> Self {
                Self { op, idx }
            }

            fn priority(&self) -> u8 {
                match self.op {
                    BinOp::Exp => 2,
                    BinOp::Mul => 1,
                    BinOp::Div => 1,
                    BinOp::Add => 0,
                    BinOp::Sub => 0,
                }
            }
        }

        let mut split_points = Vec::<SplitPoint>::new();

        let mut ind: usize = 0;

        // if the previous encountered token is an operator
        // used for the prefix op hack
        let mut prev_enc_op = false;

        for (idx, l) in expr.iter().enumerate() {
            if ind == 0 {
                match &l.kind {
                    LexemeKind::Plus => {
                        if !prev_enc_op {
                            split_points.push(SplitPoint::new(idx, BinOp::Add));
                        }
                        prev_enc_op = true;
                    }
                    LexemeKind::Minus => {
                        if !prev_enc_op {
                            split_points.push(SplitPoint::new(idx, BinOp::Sub));
                        }
                        prev_enc_op = true;
                    }
                    LexemeKind::Star => {
                        if !prev_enc_op {
                            split_points.push(SplitPoint::new(idx, BinOp::Mul))
                        }
                        prev_enc_op = true;
                    }
                    LexemeKind::Slash => {
                        if !prev_enc_op {
                            split_points.push(SplitPoint::new(idx, BinOp::Div))
                        }
                        prev_enc_op = true;
                    }
                    LexemeKind::Karat => {
                        if !prev_enc_op {
                            split_points.push(SplitPoint::new(idx, BinOp::Exp))
                        }
                        prev_enc_op = true;
                    }
                    LexemeKind::OpenParen => {
                        ind += 1;
                        prev_enc_op = false;
                    }
                    LexemeKind::CloseParen => return Err(ParserError::UnexpectedToken(l.clone())),
                    _ => {
                        prev_enc_op = false;
                    }
                }
            } else {
                match &l.kind {
                    LexemeKind::OpenParen => {
                        ind += 1;
                    }
                    LexemeKind::CloseParen => {
                        ind -= 1;
                    }
                    _ => (),
                }
            }
        }

        let target_split = split_points.iter().enumerate().max_by(|x, y| {
            let px = x.1.priority();
            let py = y.1.priority();

            let cmp = px.cmp(&py);

            match cmp {
                std::cmp::Ordering::Equal => x.0.cmp(&y.0),
                std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
                std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            }
        });

        match target_split {
            Some((_, split)) => {
                if split.idx == 0 {
                    // We are applying a unary operation to the value on the left

                    match split.op {
                        BinOp::Exp => Err(ParserError::InvalidUnaryOperator(BinOp::Exp)),
                        BinOp::Mul => Err(ParserError::InvalidUnaryOperator(BinOp::Mul)),
                        BinOp::Div => Err(ParserError::InvalidUnaryOperator(BinOp::Div)),
                        BinOp::Add => ArithmeticNode::parse(&expr[1..]),
                        BinOp::Sub => Ok(ArithmeticNode::new(
                            ArithmeticNodeKind::Neg(Box::new(ArithmeticNode::parse(&expr[1..])?)),
                            span,
                        )),
                    }
                } else {
                    let (lhs, mut rhs) = expr.split_at(split.idx);
                    rhs = &rhs[1..];

                    let left_subtree = ArithmeticNode::parse(lhs)?;
                    let right_subtree = ArithmeticNode::parse(rhs)?;

                    match split.op {
                        BinOp::Mul => Ok(ArithmeticNode::new(
                            ArithmeticNodeKind::Mul(
                                Box::new(left_subtree),
                                Box::new(right_subtree),
                            ),
                            span,
                        )),
                        BinOp::Div => Ok(ArithmeticNode::new(
                            ArithmeticNodeKind::Div(
                                Box::new(left_subtree),
                                Box::new(right_subtree),
                            ),
                            span,
                        )),
                        BinOp::Sub => Ok(ArithmeticNode::new(
                            ArithmeticNodeKind::Sub(
                                Box::new(left_subtree),
                                Box::new(right_subtree),
                            ),
                            span,
                        )),
                        BinOp::Add => Ok(ArithmeticNode::new(
                            ArithmeticNodeKind::Add(
                                Box::new(left_subtree),
                                Box::new(right_subtree),
                            ),
                            span,
                        )),
                        BinOp::Exp => Ok(ArithmeticNode::new(
                            ArithmeticNodeKind::Pow(
                                Box::new(left_subtree),
                                Box::new(right_subtree),
                            ),
                            span,
                        )),
                    }
                }
            }
            None => {
                // There is no split point, this occurs in the following cases:
                // - Our expression is fully parenthesized
                // - Our expression is the invocation of a function (i.e sqrt)

                match expr {
                    // Fully parenthesized expression
                    [tokof!(OpenParen), .., tokof!(CloseParen)] => {
                        let inner_expr = &expr[1..expr.len() - 1];

                        ArithmeticNode::parse(inner_expr)
                    }

                    // Function invocation
                    [tokof!(Word(fn_name)), tokof!(OpenParen), .., tokof!(CloseParen)] => {
                        let raw_args = &expr[2..expr.len() - 1];

                        let args = ArithmeticNode::split_at(raw_args, &LexemeKind::Comma);
                        let mut args = args
                            .into_iter()
                            .map(ArithmeticNode::parse)
                            .collect::<Result<Vec<_>, _>>()?;

                        match fn_name.as_str() {
                            // Nth root, arg 1 is the root number and arg 2 is the target
                            "nrt" => {
                                if args.len() != 2 {
                                    return Err(ParserError::InvalidArgumentCount {
                                        expected: 2,
                                        got: args.len(),
                                    });
                                }

                                let (nth_root, target) = (args.remove(0), args.remove(0));

                                Ok(ArithmeticNode::new(
                                    ArithmeticNodeKind::Root(Box::new(nth_root), Box::new(target)),
                                    span,
                                ))
                            }
                            "log" => {
                                if args.len() != 2 {
                                    return Err(ParserError::InvalidArgumentCount {
                                        expected: 2,
                                        got: args.len(),
                                    });
                                }

                                let (base, target) = (args.remove(0), args.remove(0));

                                Ok(ArithmeticNode::new(
                                    ArithmeticNodeKind::Log(Box::new(base), Box::new(target)),
                                    span,
                                ))
                            }
                            "sin" => {
                                if args.len() != 1 {
                                    return Err(ParserError::InvalidArgumentCount {
                                        expected: 1,
                                        got: args.len(),
                                    });
                                }

                                let val = args.remove(0);
                                Ok(ArithmeticNode::new(
                                    ArithmeticNodeKind::Sin(Box::new(val)),
                                    span,
                                ))
                            }
                            "cos" => {
                                if args.len() != 1 {
                                    return Err(ParserError::InvalidArgumentCount {
                                        expected: 1,
                                        got: args.len(),
                                    });
                                }

                                let val = args.remove(0);
                                Ok(ArithmeticNode::new(
                                    ArithmeticNodeKind::Cos(Box::new(val)),
                                    span,
                                ))
                            }
                            "tan" => {
                                if args.len() != 1 {
                                    return Err(ParserError::InvalidArgumentCount {
                                        expected: 1,
                                        got: args.len(),
                                    });
                                }

                                let val = args.remove(0);
                                Ok(ArithmeticNode::new(
                                    ArithmeticNodeKind::Tan(Box::new(val)),
                                    span,
                                ))
                            }
                            "deg" => {
                                if args.len() != 1 {
                                    return Err(ParserError::InvalidArgumentCount {
                                        expected: 1,
                                        got: args.len(),
                                    });
                                }

                                let val = args.remove(0);
                                Ok(ArithmeticNode::new(
                                    ArithmeticNodeKind::Deg(Box::new(val)),
                                    span,
                                ))
                            }
                            _ => panic!("Unrecognized function: {fn_name:?}"),
                        }
                    }

                    // Literal
                    [tokof!(Number(lit))] => {
                        Ok(ArithmeticNode::new(ArithmeticNodeKind::Literal(*lit), span))
                    }
                    _ => panic!("Unrecognized Expression: {expr:?}"),
                }
            }
        }
    }
}
