use std::{
    fs::{self, File},
    io::{BufReader, Read},
};

enum Token {
    LCurly,
    RCurly,
    Comma,
    Colon,
    Key(String),
    Value(ValueTok),
    EOF,
}
enum ValueTok {
    Array,
    Null,
    String,
    Int,
    Boolean,
    Object,
}
impl Token {}
struct Lexer {
    input_str: String,
    tokens: Vec<Token>,
    position: usize,
}
impl Lexer {
    //this must be a json extension
    fn init(json_path: &str) -> Self {
        let file = File::open(json_path).unwrap();
        let mut reader = BufReader::new(file);
        let mut input_str = String::new();
        let _ = reader.read_to_string(&mut input_str);
        Self {
            input_str,
            tokens: vec![],
            position: 0,
        }
    }
    fn walk_input(&self) {
        self.input_str.chars().for_each(|v| {
            println!("char:{v}");
        });
    }

    fn get_next_token(&mut self) {}
}
fn main() {
    let lex = Lexer::init("assets/test.json");
    lex.walk_input();
}

//todos:
//define grammar for the language
//impliment the lexical phase or tokenization of the input string
//fsm instead of using regex?
