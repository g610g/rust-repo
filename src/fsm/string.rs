use core::str;

use crate::lexer::{Lexeme, Token};

pub struct StringFSM {
    current_state: State,
    chars: Vec<u8>,
    current_lexeme_ch: u8,
}
impl StringFSM {
    pub fn init(chars: Vec<u8>) -> Self {
        StringFSM {
            current_state: State::StartState,
            current_lexeme_ch: chars[0],
            chars,
        }
    }
    fn give_current_state(&self) -> &State {
        &self.current_state
    }

    //returns a token variant on success. Otherwise, err
    pub fn generate_token(&mut self) -> Result<Token, &str> {
        for ch in self.chars.clone() {
            let transition = Transition::create_transition(ch);
            match self.current_state.transition(&transition) {
                Ok(state) => {
                    self.current_state = state;
                }
                Err(_) => {
                    return Err("We have an error during creating token with the given lexeme")
                }
            }
        }

        match self.current_state {
            State::AcceptState => {
                //we should only create a lexeme if this is okay
                let lexeme_string = str::from_utf8(&self.chars).unwrap();
                let lexeme = Lexeme::create_lexeme(lexeme_string.to_string(), 21);
                return Ok(Token::Key(lexeme));
            }
            _ => {
                return Err("Cannot create token based on the given lexeme");
            }
        }
    }
}
#[derive(Debug)]
enum State {
    StartState,
    SecondState,
    ThirdState,
    AcceptState,
    DeadState,
}

impl State {
    pub fn transition(&self, transition: &Transition) -> Result<State, &str> {
        match (self, transition) {
            (Self::StartState, Transition::Quote) => Ok(Self::SecondState),
            (Self::StartState, _) => Ok(Self::DeadState),
            //prolly dont return an error but return a dead_end state?
            (Self::SecondState, Transition::Quote) => Ok(Self::AcceptState),
            (Self::SecondState, _) => Ok(Self::ThirdState),
            (Self::ThirdState, Transition::Quote) => Ok(Self::AcceptState),
            (Self::ThirdState, _) => Ok(Self::ThirdState),
            (Self::AcceptState, _) => Err("Already at accept state"),
            (Self::DeadState, _) => Err("We are at dead state"),
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
    use super::StringFSM;

    #[test]
    fn assert_generating_token() -> Result<(), String> {
        let lexeme_bytes_character = String::from("\"@\"").into_bytes();
        let mut fsm = StringFSM::init(lexeme_bytes_character);
        let token = fsm.generate_token()?;
        println!("State is: {:?}", fsm.give_current_state());
        println!("Token is: {:?}", token);
        Ok(())
    }
}
