pub mod ast;

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::ast::lexer::TokenKind;

    #[test]
    fn arithmetics() {
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
}
