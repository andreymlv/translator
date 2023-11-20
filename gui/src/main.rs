use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use translator::{
    ast::{evaluator::AstEvaluator, parser::Parser, Ast},
    diagnostics::{printer::DiagnosticsPrinter, DiagnosticBag},
    text::SourceText,
};

fn main() -> Result<(), String> {
    let input = "
        a := 123;
        b := 321;
        c := a + b;
    ";
    let text = SourceText::new(input.to_string());
    let diagnostics_bag = Rc::new(RefCell::new(DiagnosticBag::new()));
    let mut ast = Ast::new();
    let mut parser = Parser::from_input(input, diagnostics_bag.clone());
    while let Some(statement) = parser.next_statement() {
        ast.add_statement(statement);
    }
    ast.visualize();
    let diagnostics_binding = diagnostics_bag.borrow();
    if !diagnostics_binding.diagnostics.is_empty() {
        let diagnostics_printer = DiagnosticsPrinter::new(&text, &diagnostics_binding.diagnostics);
        diagnostics_printer.print();
        return Err("Compilation failed".to_string());
    }
    let mut eval = AstEvaluator::new();
    ast.visit(&mut eval);
    println!("Result {input} = {:?}", eval.last_value);
    Ok(())
}
