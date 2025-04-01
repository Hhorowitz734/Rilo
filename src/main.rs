mod token;
mod lexer;

use lexer::Lexer;
use token::Token;


fn main() {
    let input = r#"
        fn main() -> int {
            let mut x = 42;
            let y = x + 3;
            return y;
        }
    "#;

    let mut lexer = Lexer::new(input);
    loop {
   
        let tok = lexer.next_token();
        println!("{:?}", tok);

        if tok == Token::Eof {
            break;
        }

    }
}
