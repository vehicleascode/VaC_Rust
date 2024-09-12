mod lexer;
mod parser;

use lexer::{Lexer, Token};
use parser::Parser;

fn main() {
    let code = "
        function startEngine() {
            speed = 100;
            if (speed > 60) {
                applyBrakes();
            }
        }
    ";

    let lexer = Lexer::new(code);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();

    println!("{:#?}", ast);
}
