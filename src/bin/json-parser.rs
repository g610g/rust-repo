use os_school::lexer::Lexer;

fn main() {
    let mut lex = Lexer::init("assets/test.json");
    lex.get_next_token();
}

//todos:
//define grammar for the language
//impliment the lexical phase or tokenization of the input string
//fsm instead of using regex?
