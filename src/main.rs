mod lexer;

use lexer::Lexer;

fn main() {
    let lexer = Lexer::new();

    if let Err(err) = lexer.scanner("_input") {
        eprintln!("{}", err);
    };
}
