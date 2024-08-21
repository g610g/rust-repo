use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::fsm::string::StringFSM;

#[derive(Debug, PartialEq)]

pub enum Token {
    LCurly,
    RCurly,
    Comma,
    Colon,
    Key,
    Value(ValueTok),
    EOF,
}
#[derive(Debug, PartialEq)]
enum ValueTok {
    Array,
    Null,
    String,
    Int,
    Boolean,
    Object,
}
impl Token {}
pub struct Lexer {
    input: Vec<u8>,
    tokens: Vec<Token>,
    current_position: usize,
    pub ch: u8,
}
impl Lexer {
    //this must be a json extension
    pub fn init(json_path: &str) -> Self {
        let file = File::open(json_path).unwrap();
        let mut reader = BufReader::new(file);
        let mut input_str = String::new();
        let _ = reader.read_to_string(&mut input_str);
        Self {
            input: input_str.into_bytes(),
            tokens: vec![],
            current_position: 0,
            ch: 0,
        }
    }
    pub fn mock_init() -> Self {
        Self {
            input: String::from("\"\"").into_bytes(),
            tokens: vec![],
            current_position: 0,
            ch: 0,
        }
    }
    fn walk_input(&self) {
        self.input.iter().for_each(|byte| {
            println!("{}", *byte as char);
        })
    }

    pub fn get_next_token(&mut self) -> Result<Token, &str> {
        self.skip_whitespaces();
        match self.ch {
            b'"' => {
                let mut string_fsm = StringFSM::init();
                return Ok(string_fsm.generate_token(self).unwrap());
            }
            _ => return Err("something went wrong"),
        }
    }

    //clears whitespace before getting the next token
    fn skip_whitespaces(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
    fn read_char(&mut self) {
        self.ch = self.input[self.current_position];
        self.current_position += 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_get_token() {
        //test fails because ch is initally zero
        let mut lexer = Lexer::mock_init();

        assert_eq!(Token::Key, lexer.get_next_token().unwrap());
    }
}
