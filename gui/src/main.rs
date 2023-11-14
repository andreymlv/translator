use anyhow::Result;
use logos::Logos;
use translator::Token;

fn main() -> Result<()> {
    let source = "
        Int a;
        Begin
            a := 12;
        End
        Print \"asd\";
        Print a;
        ";
    println!("{}", source);
    let mut lex = Token::lexer(source);
    while let Some(token) = lex.next() {
        let span = lex.span();
        let lexeme = &source[span];
        println!("{:?} {}", token, lexeme);
    }
    Ok(())
}
