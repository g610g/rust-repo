enum Token {
    LCurly,
    RCurly,
    Comma,
    Colon,
    Key(String),
    Value(ValueTok),
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
    input: String,
}
fn main() {
    let _tokens: Vec<Token> = vec![];
    //a character but the datatype of the variable is u8 or bytes
    let my_str = "Здравствуйте".to_string();
    my_str.chars().enumerate().for_each(|(idx, v)| {
        println!("value:{v}, idx:{idx}");
    });
    println!("Len of our string is: {}", my_str.len());
}

//todos:
//#define grammar for the language
//#impliment the lexical phase or tokenization of the input string
