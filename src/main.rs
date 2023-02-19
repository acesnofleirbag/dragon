pub mod lexer;
pub mod parser;

use std::error::Error;

use lexer::Lexer;
use parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let mut lexer = Lexer::new();

    if let Err(err) = lexer.scanner("_input") {
        eprintln!("{}", err);
    };

    let mut parser = Parser::new(&mut lexer);

    // call to the init token of the syntax
    match parser.e() {
        Err(err) => eprintln!(
            "Parser: syntatic analysis error\n\n== DETAILS ==\n\n{}",
            err
        ),
        Ok(_) => println!("Parsing Success"),
    }

    Ok(())
}
