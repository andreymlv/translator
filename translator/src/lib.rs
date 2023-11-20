pub mod ast;
pub mod text;
pub mod diagnostics;

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use logos::Logos;

    use crate::{
        ast::{evaluator::AstEvaluator, lexer::TokenKind, parser::Parser, Ast},
        diagnostics::DiagnosticBag,
    };

    #[test]
    fn arithmetics_lexer() {
        let source = "1 + 2 * (2.12 / 5)";
        let mut lex = TokenKind::lexer(source);
        assert_eq!(lex.next(), Some(Ok(TokenKind::LiteralInteger(1))));
        assert_eq!(lex.next(), Some(Ok(TokenKind::OpAddition)));
        assert_eq!(lex.next(), Some(Ok(TokenKind::LiteralInteger(2))));
        assert_eq!(lex.next(), Some(Ok(TokenKind::OpMultiplication)));
        assert_eq!(lex.next(), Some(Ok(TokenKind::LeftParen)));
        assert_eq!(lex.next(), Some(Ok(TokenKind::LiteralFloat(2.12))));
        assert_eq!(lex.next(), Some(Ok(TokenKind::OpDivision)));
        assert_eq!(lex.next(), Some(Ok(TokenKind::LiteralInteger(5))));
        assert_eq!(lex.next(), Some(Ok(TokenKind::RightParen)));
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn arithmetics_parse_eval() {
        let input = "2*9+ 3 / 1 + (2 + 3)";
        let diagnostics_bag = Rc::new(RefCell::new(DiagnosticBag::new()));
        let mut ast = Ast::new();
        let mut parser = Parser::from_input(input, diagnostics_bag);
        while let Some(statement) = parser.next_statement() {
            ast.add_statement(statement);
        }
        let mut eval = AstEvaluator::new();
        ast.visit(&mut eval);
        println!("Result {input} = {:?}", eval.last_value);
        assert_eq!(eval.last_value, Some(26));
    }
}
