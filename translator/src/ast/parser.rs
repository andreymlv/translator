use logos::Logos;

use super::{
    lexer::{Token, TokenKind},
    AstBinaryOperator, AstBinaryOperatorKind, AstExpression, AstStatement,
};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
        }
    }

    pub fn from_input(source: &str) -> Self {
        let mut lex = TokenKind::lexer(source);
        let mut tokens = Vec::new();
        while let Some(token) = lex.next() {
            let lexeme = source[lex.span()].to_string();
            tokens.push(Token::new(token.unwrap(), lex.span(), lexeme))
        }
        Self { tokens, current: 0 }
    }

    pub fn next_statement(&mut self) -> Option<AstStatement> {
        return self.parse_statement();
    }

    fn parse_statement(&mut self) -> Option<AstStatement> {
        let token = self.current();
        if token.is_none() {
            return None;
        }
        let expr = self.parse_expression()?;
        return Some(AstStatement::expression(expr));
    }

    fn parse_expression(&mut self) -> Option<AstExpression> {
        return self.parse_binary_expression(0);
    }

    fn parse_binary_expression(&mut self, precedence: u8) -> Option<AstExpression> {
        let mut left = self.parse_primary_expression()?;

        while let Some(operator) = self.parse_binary_operator() {
            let operator_precedence = operator.precedence();
            if operator_precedence < precedence {
                break;
            }
            self.consume();
            let right = self.parse_binary_expression(operator_precedence)?;
            left = AstExpression::binary(operator, left, right);
        }

        return Some(left);
    }

    fn parse_binary_operator(&mut self) -> Option<AstBinaryOperator> {
        let token = self.current()?;
        let kind = match token.kind {
            TokenKind::OpAddition => Some(AstBinaryOperatorKind::Plus),
            TokenKind::OpSubtraction => Some(AstBinaryOperatorKind::Minus),
            TokenKind::OpMultiplication => Some(AstBinaryOperatorKind::Multiply),
            TokenKind::OpDivision => Some(AstBinaryOperatorKind::Divide),
            _ => None,
        }?;
        Some(AstBinaryOperator::new(kind, token.clone()))
    }

    fn parse_primary_expression(&mut self) -> Option<AstExpression> {
        let token = self.consume()?;
        return match token.kind {
            TokenKind::LiteralInteger(number) => Some(AstExpression::number(number)),
            TokenKind::LeftParen => {
                let expr = self.parse_expression()?;
                let token = self.consume()?;
                if token.kind != TokenKind::RightParen {
                    panic!("Expected right paren");
                }
                Some(AstExpression::parenthesized(expr))
            }
            _ => None,
        };
    }

    fn peek(&self, offset: isize) -> Option<&Token> {
        self.tokens.get((self.current as isize + offset) as usize)
    }

    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }

    fn consume(&mut self) -> Option<&Token> {
        self.current += 1;
        let token = self.peek(-1)?;
        Some(token)
    }
}
