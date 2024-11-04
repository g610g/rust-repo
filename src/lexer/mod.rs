use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::fsm::string::StringFSM;

#[derive(Debug, PartialEq)]
pub struct Lexeme {
    value: String,
    line: u32,
}
impl Lexeme {
    pub fn create_lexeme(value: String, line: u32) -> Self {
        return Lexeme { value, line };
    }
}
#[derive(Debug, PartialEq)]
pub enum Token {
    LCurly(Lexeme),
    RCurly(Lexeme),
    Comma(Lexeme),
    Colon(Lexeme),
    Key(Lexeme),
    Value(),
    EOF,
}
//#[derive(Debug, PartialEq)]
//enum ValueTok {
//    Array,
//    Null,
//    String,
//    Int,
//    Boolean,
//    Object,
//}
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
    pub fn mock_init(mock_string: String) -> Self {
        Self {
            input: mock_string.into_bytes(),
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
            //get the string character first
            b'"' => {
                let lexeme = self.get_lexeme();
                let mut string_fsm = StringFSM::init(lexeme);
                return Ok(string_fsm.generate_token(self).unwrap());
            }
            _ => return Err("something went wrong"),
        }
    }
    fn get_lexeme(&mut self) -> Vec<u8> {
        let mut chars = Vec::new();
        while !self.ch.is_ascii_whitespace() {
            chars.push(self.input[self.current_position]);
            self.read_char();
        }
        return chars;
    }
    //clears whitespace before getting the next token
    fn skip_whitespaces(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        self.current_position += 1;
        self.ch = self.input[self.current_position];
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn assert_get_token() {
    //    //test fails because ch is initally zero
    //    let mock_string = " Just testing".to_string();
    //    let mut lexer = Lexer::mock_init(mock_string.clone());
    //
    //    assert_eq!(
    //        Token::Key(Lexeme::create_lexeme(mock_string.clone(), 8)),
    //        lexer.get_next_token().unwrap()
    //    );
    //}

    #[test]
    fn assert_get_lexeme() {
        let mock_string = " Just testing".to_string();
        let mut lexer = Lexer::mock_init(mock_string.clone());
        let character_bytes = lexer.get_lexeme();
        let mock_string_slice = &mock_string.as_bytes()[..5];
        assert_eq!(character_bytes, mock_string_slice);
    }
}
