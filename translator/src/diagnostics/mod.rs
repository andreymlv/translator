pub mod printer;

use std::{cell::RefCell, ops::Range, rc::Rc};

use crate::{
    ast::lexer::{Token, TokenKind},
    text::span::Span,
};

#[derive(Debug)]
pub enum DiagnosticKind {
    Error,
    Warning,
}

#[derive(Debug)]
pub struct Diagnostic {
    pub message: String,
    pub span: Span,
    pub kind: DiagnosticKind,
}

impl Diagnostic {
    pub fn new(message: String, span: Span, kind: DiagnosticKind) -> Self {
        Self {
            message,
            span,
            kind,
        }
    }
}

pub type DiagnosticBagCell = Rc<RefCell<DiagnosticBag>>;

#[derive(Debug)]
pub struct DiagnosticBag {
    pub diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBag {
    pub fn new() -> Self {
        Self {
            diagnostics: vec![],
        }
    }

    pub fn report_error(&mut self, message: String, span: Span) {
        let error = Diagnostic::new(message, span, DiagnosticKind::Error);
        self.diagnostics.push(error);
    }

    pub fn report_warning(&mut self, message: String, span: Span) {
        let error = Diagnostic::new(message, span, DiagnosticKind::Warning);
        self.diagnostics.push(error);
    }

    pub fn report_unexpected_token(&mut self, expected: &TokenKind, actual: &Token) {
        self.report_error(
            format!("Expected <{:?}>, found <{:?}>", expected, actual.kind),
            actual.span.clone(),
        )
    }

    pub fn report_expected_expression(&mut self, actual: &Token) {
        self.report_error(
            format!("Expected expression, found <{:?}>", actual.kind),
            actual.span.clone(),
        )
    }

    pub fn report_unknown_token(&mut self, actual: &TokenKind, span: Span) {
        self.report_error(format!("Unknown token finded <{:?}>", actual), span)
    }
}
