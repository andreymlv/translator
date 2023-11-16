use anyhow::Result;
use translator::ast::{evaluator::AstEvaluator, parser::Parser, Ast};

fn main() -> Result<()> {
    let input = "(1 +2)+3 * 9 + 1";
    let mut ast = Ast::new();
    let mut parser = Parser::from_input(input);
    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }
    ast.visualize();
    let mut eval = AstEvaluator::new();
    ast.visit(&mut eval);
    println!("Result {input} = {:?}", eval.last_value);
    Ok(())
}
