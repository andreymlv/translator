use std::ops::Range;

use crate::text::span::Span;

use self::lexer::Token;
use colored::*;

pub mod evaluator;
pub mod lexer;
pub mod parser;

pub struct Ast {
    pub statements: Vec<AstStatement>,
}

impl Ast {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: AstStatement) {
        self.statements.push(statement);
    }

    pub fn visit(&self, visitor: &mut dyn AstVisitor) {
        for statement in &self.statements {
            visitor.visit_statement(statement);
        }
    }

    pub fn visualize(&self) -> () {
        let mut printer = AstPrinter {
            result: String::new(),
        };
        self.visit(&mut printer);
        println!("{}", printer.result)
    }
}

pub trait AstVisitor {
    fn do_visit_statement(&mut self, statement: &AstStatement) {
        match &statement.kind {
            AstStatementKind::Expression(expr) => {
                self.visit_expression(expr);
            }
            AstStatementKind::AssignStatement(statement) => self.visit_assign_statement(statement),
        }
    }
    fn visit_statement(&mut self, statement: &AstStatement) {
        self.do_visit_statement(statement);
    }
    fn do_visit_expression(&mut self, expression: &AstExpression) {
        match &expression.kind {
            AstExpressionKind::Number(number) => {
                self.visit_number(number);
            }
            AstExpressionKind::Binary(expr) => {
                self.visit_binary_expression(expr);
            }
            AstExpressionKind::Parenthesized(expr) => {
                self.visit_parenthesized_expression(expr);
            }
            AstExpressionKind::Error(span) => self.visit_error(span),
            AstExpressionKind::Variable(expr) => todo!(),
        }
    }
    fn visit_expression(&mut self, expression: &AstExpression) {
        self.do_visit_expression(expression);
    }

    fn visit_assign_statement(&mut self, statement: &AstAssignStatement);

    fn visit_number(&mut self, number: &AstNumberExpression);

    fn visit_error(&mut self, span: &Span);

    fn visit_binary_expression(&mut self, binary_expression: &AstBinaryExpression) {
        self.visit_expression(&binary_expression.left);
        self.visit_expression(&binary_expression.right);
    }

    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &AstParenthesizedExpression,
    ) {
        self.visit_expression(&parenthesized_expression.expression);
    }
}

pub struct AstPrinter {
    result: String,
}

impl AstPrinter {
    fn add_whitespace(&mut self) {
        self.result.push_str(" ")
    }
    fn add_newline(&mut self) {
        self.result.push_str("\n")
    }
}

impl AstVisitor for AstPrinter {
    fn visit_statement(&mut self, statement: &AstStatement) {
        AstVisitor::do_visit_statement(self, statement);
        self.add_newline();
    }

    fn visit_expression(&mut self, expression: &AstExpression) {
        AstVisitor::do_visit_expression(self, expression);
    }

    fn visit_number(&mut self, number: &AstNumberExpression) {
        self.result
            .push_str(&format!("{}", number.number.to_string().cyan()));
    }

    fn visit_binary_expression(&mut self, binary_expression: &AstBinaryExpression) {
        self.visit_expression(&binary_expression.left);
        self.add_whitespace();
        self.result.push_str(&format!(
            "{}",
            binary_expression.operator.token.lexeme.white()
        ));
        self.add_whitespace();
        self.visit_expression(&binary_expression.right);
    }

    fn visit_parenthesized_expression(
        &mut self,
        parenthesized_expression: &AstParenthesizedExpression,
    ) {
        self.result.push_str("(");
        self.visit_expression(&parenthesized_expression.expression);
        self.result.push_str(")");
    }

    fn visit_error(&mut self, span: &Span) {
        self.result.push_str(&format!("{}", span.literal.red()));
    }

    fn visit_assign_statement(&mut self, statement: &AstAssignStatement) {
        self.result
            .push_str(&format!("{}", statement.identifier.lexeme.green()));
        self.add_whitespace();
        self.result.push_str(":=");
        self.add_whitespace();
        self.visit_expression(&statement.initializer);
        self.result.push_str(";");
    }
}

pub enum AstStatementKind {
    Expression(AstExpression),
    AssignStatement(AstAssignStatement),
}

pub struct AstAssignStatement {
    identifier: Token,
    initializer: AstExpression,
}

pub struct AstStatement {
    kind: AstStatementKind,
}

impl AstStatement {
    pub fn new(kind: AstStatementKind) -> Self {
        AstStatement { kind }
    }

    pub fn expression(expr: AstExpression) -> Self {
        AstStatement::new(AstStatementKind::Expression(expr))
    }

    pub fn assign_statement(identifier: Token, initializer: AstExpression) -> Self {
        AstStatement::new(AstStatementKind::AssignStatement(AstAssignStatement {
            identifier,
            initializer,
        }))
    }
}

pub enum AstExpressionKind {
    Number(AstNumberExpression),
    Binary(AstBinaryExpression),
    Parenthesized(AstParenthesizedExpression),
    Variable(AstVariableExpression),
    Error(Span),
}

#[derive(Debug)]
pub enum AstBinaryOperatorKind {
    Plus,
    Minus,
    Multiply,
    Divide,
    Mod,
}

pub struct AstBinaryOperator {
    kind: AstBinaryOperatorKind,
    token: Token,
}

impl AstBinaryOperator {
    pub fn new(kind: AstBinaryOperatorKind, token: Token) -> Self {
        AstBinaryOperator { kind, token }
    }

    pub fn precedence(&self) -> u8 {
        match self.kind {
            AstBinaryOperatorKind::Plus => 3,
            AstBinaryOperatorKind::Minus => 3,
            AstBinaryOperatorKind::Multiply => 4,
            AstBinaryOperatorKind::Divide => 4,
            AstBinaryOperatorKind::Mod => 4,
        }
    }
}

pub struct AstBinaryExpression {
    left: Box<AstExpression>,
    operator: AstBinaryOperator,
    right: Box<AstExpression>,
}

pub struct AstNumberExpression {
    number: i32,
}

pub struct AstParenthesizedExpression {
    expression: Box<AstExpression>,
}

pub struct AstVariableExpression {
    identifier: Token,
}

impl AstVariableExpression {
    pub fn identifier(&self) -> &str {
        &self.identifier.lexeme
    }
}

pub struct AstExpression {
    kind: AstExpressionKind,
}

impl AstExpression {
    pub fn new(kind: AstExpressionKind) -> Self {
        AstExpression { kind }
    }

    pub fn number(number: i32) -> Self {
        AstExpression::new(AstExpressionKind::Number(AstNumberExpression { number }))
    }

    pub fn binary(operator: AstBinaryOperator, left: AstExpression, right: AstExpression) -> Self {
        AstExpression::new(AstExpressionKind::Binary(AstBinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }))
    }

    pub fn parenthesized(expression: AstExpression) -> Self {
        AstExpression::new(AstExpressionKind::Parenthesized(
            AstParenthesizedExpression {
                expression: Box::new(expression),
            },
        ))
    }

    fn error(span: Span) -> AstExpression {
        AstExpression::new(AstExpressionKind::Error(span))
    }
}
