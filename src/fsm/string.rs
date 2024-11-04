use core::{panic, str};

use crate::lexer::{Lexeme, Lexer, Token};

pub struct StringFSM {
    current_state: StringState,
    chars: Vec<u8>,
}
impl StringFSM {
    pub fn init() -> Self {
        StringFSM {
            current_state: StringState::StartState,
            chars: Vec::new(),
        }
    }

    //This attribute will disable the lint warning for fn's that are unused
    #[allow(dead_code)]
    fn give_current_state(&self) -> &StringState {
        &self.current_state
    }

    //returns a token variant on success. Otherwise, err
    pub fn generate_token(&mut self, lexer: &mut Lexer) -> Result<Token, &str> {
        let mut transition = Transition::create_transition(lexer.give_current_ch());
        loop {
            match self.current_state.transition(&transition) {
                Ok(state) => match state {
                    StringState::AcceptState => {
                        self.current_state = state;
                        self.chars.push(lexer.give_current_ch());
                        break;
                    }
                    _ => {
                        self.current_state = state;
                        self.chars.push(lexer.give_current_ch());
                        lexer.read_char();
                        transition = Transition::create_transition(lexer.give_current_ch());
                    }
                },
                Err(_) => panic!("Err at generating token in string fsm"),
            }
        }

        match self.current_state {
            StringState::AcceptState => {
                //we should only create a lexeme if this is okay
                let lexeme_string = str::from_utf8(&self.chars).unwrap();
                let lexeme = Lexeme::create_lexeme(lexeme_string.to_string(), 21);
                return Ok(Token::String(lexeme));
            }
            _ => {
                return Err("Cannot create token based on the given lexeme");
            }
        }
    }
}
#[derive(Debug)]
pub enum StringState {
    StartState,
    SecondState,
    ThirdState,
    AcceptState,
    DeadState,
}

impl StringState {
    pub fn transition(&self, transition: &Transition) -> Result<StringState, StringState> {
        match (self, transition) {
            (Self::StartState, Transition::Quote) => Ok(Self::SecondState),
            (Self::StartState, _) => Err(Self::DeadState),
            (Self::SecondState, Transition::Quote) => Ok(Self::AcceptState),
            (Self::SecondState, _) => Ok(Self::ThirdState),
            (Self::ThirdState, Transition::Quote) => Ok(Self::AcceptState),
            (Self::ThirdState, _) => Ok(Self::ThirdState),
            (Self::AcceptState, _) => Err(Self::DeadState),
            (Self::DeadState, _) => Err(Self::DeadState),
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
    pub fn create_transition(ch: u8) -> Self {
        if ch.is_ascii_alphabetic() {
            Self::Alphabet
        } else if ch == b'"' {
            Self::Quote
        } else if ch.is_ascii_digit() {
            Self::Numeric
        } else {
            Self::SpecialChar
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;

    use super::StringFSM;

    #[test]
    fn assert_generating_token() -> Result<(), String> {
        let mut lexer = Lexer::mock_init("\"Hello World \"\"".to_string());
        let mut fsm = StringFSM::init();
        let token = fsm.generate_token(&mut lexer)?;
        println!("State is: {:?}", fsm.give_current_state());
        println!("Token is: {:?}", token);
        Ok(())
    }
}
