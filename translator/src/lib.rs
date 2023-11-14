use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("Int")]
    Int,
    #[token("Float")]
    Float,
    #[token("String")]
    String,
    #[token("Boolean")]
    Boolean,

    #[token("Begin")]
    Begin,
    #[token("End")]
    End,

    #[token("Print")]
    Print,

    #[regex("[a-zA-Z$_][a-zA-Z0-9$_]*", |lex| lex.slice().parse().ok())]
    Identifier(String),
    #[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#, |lex| lex.slice().parse().ok())]
    LiteralString(String),
    #[regex("-?[0-9]+", |lex| lex.slice().parse().ok())]
    LiteralInteger(i32),
    #[regex("[0-9]*\\.[0-9]+([eE][+-]?[0-9]+)?|[0-9]+[eE][+-]?[0-9]+", |lex| lex.slice().parse().ok())]
    LiteralFloat(f32),

    #[token(":=")]
    OpAssign,
    #[token("+")]
    OpAddition,
    #[token("-")]
    OpSubtraction,
    #[token("*")]
    OpMultiplication,
    #[token("/")]
    OpDivision,
    #[token("&&")]
    OpAnd,
    #[token("||")]
    OpOr,
    #[token("^")]
    OpXor,
    #[token("!")]
    OpNot,
    #[token("=")]
    OpEqual,

    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("True")]
    True,
    #[token("False")]
    False,
    #[token(";")]
    Semicolon,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum BuiltIn {
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
    Not,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetics() {
        let source = "1 + 2 * (2.12 / 5)";
        let mut lex = Token::lexer(source);
        assert_eq!(lex.next(), Some(Ok(Token::LiteralInteger(1))));
        assert_eq!(lex.next(), Some(Ok(Token::OpAddition)));
        assert_eq!(lex.next(), Some(Ok(Token::LiteralInteger(2))));
        assert_eq!(lex.next(), Some(Ok(Token::OpMultiplication)));
        assert_eq!(lex.next(), Some(Ok(Token::LeftParen)));
        assert_eq!(lex.next(), Some(Ok(Token::LiteralFloat(2.12))));
        assert_eq!(lex.next(), Some(Ok(Token::OpDivision)));
        assert_eq!(lex.next(), Some(Ok(Token::LiteralInteger(5))));
        assert_eq!(lex.next(), Some(Ok(Token::RightParen)));
        assert_eq!(lex.next(), None);
    }
}
