pub mod lexer;
pub mod statement;
pub mod expression;
pub mod evaluator;


fn main() {

    let input1 = "foo = INF / 20";
    let input2 = "bar = foz * 4 - 31";
    let input3 = "baz = bar / 3";
    let input4 = "baz ^ 2";

    let t1 = lexer::Lexer::new(input1.to_string()).all().unwrap();
    let t2 = lexer::Lexer::new(input2.to_string()).all().unwrap();
    let t3 = lexer::Lexer::new(input3.to_string()).all().unwrap();
    let t4 = lexer::Lexer::new(input4.to_string()).all().unwrap();


    let s1 = statement::Statement::parse(&t1).unwrap();
    let s2 = statement::Statement::parse(&t2).unwrap();
    let s3 = statement::Statement::parse(&t3).unwrap();
    let s4 = statement::Statement::parse(&t4).unwrap();
    

    let mut eval = evaluator::Evaluator::new();

    if let Err(e) = eval.eval_statement(&s1){
        println!("{}", e.contextualize(input1));
        return;
    }
    if let Err(e) = eval.eval_statement(&s2){
        println!("{}", e.contextualize(input2));
        return;
    }
    if let Err(e) = eval.eval_statement(&s3){
        println!("{}", e.contextualize(input3));
        return;
    }
    if let Err(e) = eval.eval_statement(&s4){
        println!("{}", e.contextualize(input4));
        return;
    }
}
