use std::cell::Cell;

use logos::Logos;

use crate::{diagnostics::DiagnosticBagCell, text::span::Span};

use super::{
    lexer::{Token, TokenKind},
    AstBinaryOperator, AstBinaryOperatorKind, AstExpression, AstStatement,
};

#[derive(Debug)]
pub struct Counter {
    value: Cell<usize>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            value: Cell::new(0),
        }
    }

    pub fn increment(&self) {
        let current_value = self.value.get();
        self.value.set(current_value + 1);
    }

    pub fn get_value(&self) -> usize {
        self.value.get()
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: Counter,
    diagnostics_bag: DiagnosticBagCell,
}

impl Parser {
    pub fn new(diagnostics_bag: DiagnosticBagCell) -> Self {
        Self {
            tokens: Vec::new(),
            current: Counter::new(),
            diagnostics_bag,
        }
    }

    pub fn from_input(source: &str, diagnostics_bag: DiagnosticBagCell) -> Self {
        let mut lex = TokenKind::lexer(source);
        let mut tokens = Vec::new();
        while let Some(token) = lex.next() {
            let lexeme = source[lex.span()].to_string();
            if let Ok(token) = token {
                let span = lex.span();
                tokens.push(Token::new(
                    token,
                    Span::new(span.start, span.end, lexeme.clone()),
                    lexeme,
                ))
            } else {
                let span = lex.span();
                diagnostics_bag
                    .borrow_mut()
                    .report_unknown_token(&token.unwrap(), Span::new(span.start, span.end, lexeme));
            }
        }
        let lexeme = source[lex.span()].to_string();
        let span = lex.span();
        tokens.push(Token::new(
            TokenKind::EOF,
            Span::new(span.start, span.end, lexeme.clone()),
            lexeme,
        ));
        Self {
            tokens,
            current: Counter::new(),
            diagnostics_bag,
        }
    }

    pub fn next_statement(&mut self) -> Option<AstStatement> {
        if self.is_at_end() {
            return None;
        }
        Some(self.parse_statement())
    }

    fn is_at_end(&self) -> bool {
        self.current().kind == TokenKind::EOF
    }

    fn parse_statement(&mut self) -> AstStatement {
        match &self.current().kind {
            TokenKind::Identifier(name) => self.parse_assign_statement(name.clone()),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_assign_statement(&mut self, name: String) -> AstStatement {
        let identifier = self.consume_and_check(TokenKind::Identifier(name)).clone();
        self.consume_and_check(TokenKind::OpAssign);
        let expr = self.parse_expression();
        self.consume_and_check(TokenKind::Semicolon);
        AstStatement::assign_statement(identifier, expr)
    }

    fn parse_expression_statement(&mut self) -> AstStatement {
        let expr = self.parse_expression();
        AstStatement::expression(expr)
    }

    fn parse_expression(&mut self) -> AstExpression {
        return self.parse_binary_expression(0);
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> AstExpression {
        let mut left = self.parse_primary_expression();

        while let Some(operator) = self.parse_binary_operator() {
            let operator_precedence = operator.precedence();
            if operator_precedence < precedence {
                break;
            }
            self.consume();
            let right = self.parse_binary_expression(operator_precedence);
            left = AstExpression::binary(operator, left, right);
        }

        return left;
    }

    fn parse_binary_operator(&mut self) -> Option<AstBinaryOperator> {
        let token = self.current();
        let kind = match token.kind {
            TokenKind::OpAddition => Some(AstBinaryOperatorKind::Plus),
            TokenKind::OpSubtraction => Some(AstBinaryOperatorKind::Minus),
            TokenKind::OpMultiplication => Some(AstBinaryOperatorKind::Multiply),
            TokenKind::OpDivision => Some(AstBinaryOperatorKind::Divide),
            _ => None,
        }?;
        Some(AstBinaryOperator::new(kind, token.clone()))
    }

    fn parse_primary_expression(&mut self) -> AstExpression {
        let token = self.consume();
        match &token.kind {
            TokenKind::LiteralInteger(number) => AstExpression::number(*number),
            TokenKind::LeftParen => {
                let expr = self.parse_expression();
                self.consume_and_check(TokenKind::RightParen);
                AstExpression::parenthesized(expr)
            }
            // TokenKind::Identifier(name) => todo!(),
            _ => {
                self.diagnostics_bag
                    .borrow_mut()
                    .report_expected_expression(token);
                AstExpression::error(token.span.clone())
            }
        }
    }

    fn peek(&self, offset: isize) -> &Token {
        let mut index = (self.current.get_value() as isize + offset) as usize;
        if index >= self.tokens.len() {
            index = self.tokens.len() - 1;
        }
        self.tokens.get(index).unwrap()
    }

    fn current(&self) -> &Token {
        self.peek(0)
    }

    fn consume(&self) -> &Token {
        self.current.increment();
        self.peek(-1)
    }

    fn consume_and_check(&self, kind: TokenKind) -> &Token {
        let token = self.consume();
        if token.kind != kind {
            self.diagnostics_bag
                .borrow_mut()
                .report_unexpected_token(&kind, token);
        }
        token
    }
}
