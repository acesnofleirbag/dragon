use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
pub struct Cursor {
    line: i32,
    column: i32,
}

pub struct Lexer {
    cursor: Cursor,
}

#[derive(Debug)]
enum Symbol {
    Identifier,
    Number,
    Ponctuation,
    Operator,
    Assign,
}

#[derive(Debug)]
struct Token {
    key: Symbol,
    value: String,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            cursor: Cursor { line: 0, column: 0 },
        }
    }

    fn is_digit(ch: u8) -> bool {
        matches!(ch, 0x30..=0x39)
    }

    fn is_letter(ch: u8) -> bool {
        matches!(ch, 0x41..=0x5a | 0x61..=0x7a)
    }

    fn is_operator(ch: u8) -> bool {
        matches!(ch, 0x21 | 0x3c..=0x3e)
    }

    fn is_new_line(ch: u8) -> bool {
        ch == 0x0a
    }

    fn is_whitespace_like(ch: u8) -> bool {
        matches!(ch, 0x09 | 0x0c | 0x0d | 0x20)
    }

    fn refresh_cursor(&mut self, token_value: &mut String, ch: u8) {
        if !Lexer::is_whitespace_like(ch) && !Lexer::is_new_line(ch) {
            self.cursor = Cursor {
                column: self.cursor.column + 1,
                ..self.cursor
            };

            token_value.push(char::from_u32(ch as u32).unwrap());
        } else if Lexer::is_new_line(ch) {
            self.cursor = Cursor {
                line: self.cursor.line + 1,
                column: 0,
            };
        }
    }

    // 47:14
    fn nchar(mut self, buffer: Vec<u8>) -> Result<Vec<Token>, &'static str> {
        let mut state = 0;
        let mut tokens = vec![];
        let mut value = String::new();

        for &ch in buffer.iter() {
            Lexer::refresh_cursor(&mut self, &mut value, ch);

            match state {
                0 => {
                    if Lexer::is_whitespace_like(ch) || Lexer::is_new_line(ch) {
                        state = 0;
                    } else if Lexer::is_letter(ch) {
                        state = 1;
                    } else if Lexer::is_digit(ch) {
                        state = 3;
                    } else {
                        return Err("Lexer: Unrecognized symbol");
                    }
                }
                1 => {
                    if Lexer::is_letter(ch) || Lexer::is_digit(ch) {
                        state = 1;
                    } else {
                        state = 2;
                    }
                }
                2 => {
                    tokens.push(Token {
                        key: Symbol::Identifier,
                        value: value.clone(),
                    });

                    state = 0;
                    value = String::new();
                }
                _ => {
                    return Err("Lexer: Invalid state");
                }
            }
        }

        Ok(tokens)
    }

    pub fn scanner(self, filepath: &str) -> Result<(), Box<dyn Error>> {
        let f = File::open(filepath)?;
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer)?;

        let tokens = self.nchar(buffer)?;

        // @@@
        dbg!(tokens);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_nchar() {}
}
