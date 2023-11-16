use logos::{Logos, Span};

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum TokenKind {
    #[token("Int")]
    Int,
    #[token("Float")]
    Float,
    #[token("String")]
    String,
    #[token("Logical")]
    Logical,

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
    #[token("%")]
    OpPercent,
    #[token("/")]
    OpDivision,
    #[token("&&")]
    OpLogicalAnd,
    #[token("&")]
    OpBitwiseAnd,
    #[token("||")]
    OpLogicalOr,
    #[token("|")]
    OpBitwiseOr,
    #[token("^")]
    OpBitwiseXor,
    #[token("!")]
    OpLogicalNot,
    #[token("~")]
    OpBitwiseNot,
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

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span, lexeme: String) -> Self {
        Self { kind, span, lexeme }
    }
}
