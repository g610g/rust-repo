use crate::lexer::{Lexer, Token};

pub struct StringFSM {
    current_state: State,
}
impl StringFSM {
    pub fn init() -> Self {
        StringFSM {
            current_state: State::StartState,
        }
    }

    //returns Result if error occured during running state machine
    pub fn generate_token(&mut self, lexer: &mut Lexer) -> Result<Token, &str> {
        let transition: Transition = Transition::create_transition(lexer.ch);

        while let Ok(state) = self.current_state.transition(&transition) {
            self.current_state = state;
        }
        return Ok(Token::Key);
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
