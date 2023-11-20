use crate::ast::AstBinaryExpression;
use crate::ast::AstBinaryOperatorKind;
use crate::ast::AstNumberExpression;
use crate::ast::AstVisitor;
use crate::text::span::Span;

pub struct AstEvaluator {
    pub last_value: Option<i32>,
}

impl AstEvaluator {
    pub fn new() -> Self {
        Self { last_value: None }
    }
}

impl AstVisitor for AstEvaluator {
    fn visit_number(&mut self, number: &AstNumberExpression) {
        self.last_value = Some(number.number);
    }

    fn visit_binary_expression(&mut self, expr: &AstBinaryExpression) {
        self.visit_expression(&expr.left);
        let left = self.last_value.unwrap();
        self.visit_expression(&expr.right);
        let right = self.last_value.unwrap();
        self.last_value = Some(match expr.operator.kind {
            AstBinaryOperatorKind::Plus => left + right,
            AstBinaryOperatorKind::Minus => left - right,
            AstBinaryOperatorKind::Multiply => left * right,
            AstBinaryOperatorKind::Divide => left / right,
            AstBinaryOperatorKind::Mod => left % right,
        });
    }

    fn visit_error(&mut self, _span: &Span) {
        todo!()
    }

    fn visit_assign_statement(&mut self, _statement: &super::AstAssignStatement) {
        todo!()
    }
}
