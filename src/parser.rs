//! Syntax Rules
//! ------------
//!
//! ```
//! E -> T E'
//! E' -> OP T E' | &
//! T -> id | num
//! OP -> + | - | * | /
//! ```

use crate::{
    lexer::{Symbol, Token},
    Lexer,
};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: &'a Lexer,
    curr_token: Option<&'a Token>,
    curr_token_pos: usize,
}

impl<'a> Parser<'a> {
    // the parser need to have the lexer to make an on-demand usage
    pub fn new(lexer: &'a Lexer) -> Parser<'a> {
        Parser {
            lexer,
            curr_token: None,
            curr_token_pos: 0,
        }
    }

    pub fn e(&mut self) -> Result<(), String> {
        self.t()?;
        self.eline()?;

        Ok(())
    }

    fn eline(&mut self) -> Result<(), String> {
        self.curr_token = self.lexer.advance(self.curr_token_pos);
        self.curr_token_pos += 1;

        if let Some(token) = self.curr_token {
            let token_t = token.get_type();

            if token_t != Symbol::Operator {
                self.op(token_t)?;
                self.t()?;
                self.eline()?;
            }
        }

        Ok(())
    }

    fn t(&mut self) -> Result<(), String> {
        self.curr_token = self.lexer.advance(self.curr_token_pos);
        self.curr_token_pos += 1;

        if let Some(token) = self.curr_token {
            let token_t = token.get_type();

            if token_t != Symbol::Identifier && token_t != Symbol::Number {
                return Err(format!(
                    "Parser: token Identifier or Number expected, received: \n\n```\n{:#?}\n```",
                    self.curr_token.unwrap()
                ));
            }
        }

        Ok(())
    }

    fn op(&self, token_t: Symbol) -> Result<(), String> {
        if token_t != Symbol::Operator {
            return Err(format!(
                "Parser: token Operator expected, received: \n\n```\n{:#?}\n```",
                self.curr_token.unwrap()
            ));
        }

        Ok(())
    }
}
