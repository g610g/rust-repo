use crate::lexer::{Lexeme, Lexer, Token};

pub struct StringFSM {
    current_state: State,
    chars: Vec<u8>,
    chars_length: usize,
}
impl StringFSM {
    pub fn init(chars: Vec<u8>) -> Self {
        StringFSM {
            current_state: State::StartState,
            chars_length: chars.len(),
            chars,
        }
    }

    //I should return the Token with a lexem
    pub fn generate_token(&mut self, lexer: &mut Lexer) -> Result<Token, &str> {
        let transition: Transition = Transition::create_transition(lexer.ch);

        while let Ok(state) = self.current_state.transition(&transition) {
            self.current_state = state;
        }
        let lexeme = Lexeme::create_lexeme(String::new(), 2);
        return Ok(Token::Key(lexeme));
    }
}
enum State {
    StartState,
    FinalState,
    StartQuote,
    EndQuote,
    AsciiChar,
    DeadendState,
}

impl State {
    pub fn transition(&self, input: &Transition) -> Result<State, &str> {
        match (self, input) {
            (Self::StartState, Transition::Quote) => Ok(Self::StartQuote),
            (Self::StartState, _) => Err("State transition error"),
            (Self::StartQuote, Transition::Quote) => Err("State transition error"),
            (Self::StartQuote, _) => Ok(Self::AsciiChar),
            (Self::AsciiChar, Transition::Quote) => Ok(Self::FinalState),
            (Self::AsciiChar, _) => Ok(Self::AsciiChar),
            (Self::EndQuote, _) => Err("State transition error"),
            (Self::FinalState, _) => Ok(Self::DeadendState),
            (Self::DeadendState, _) => Ok(Self::DeadendState),
        }
    }
}
enum Transition {
    Numeric,
    SpecialChar,
    Alphabet,
    Quote,
}
impl Transition {
    //brb
    pub fn create_transition(item: u8) -> Self {
        if item.is_ascii_alphabetic() {
            Self::Alphabet
        } else if item == b'"' {
            Self::Quote
        } else if item.is_ascii_digit() {
            Self::Numeric
        } else {
            Self::SpecialChar
        }
    }
}
