struct StringFSM {
    current_state: State,
    initial_state: State,
    final_state: State,
}
impl StringFSM {
    pub fn init() -> Self {
        StringFSM {
            current_state: State::StartState,
            initial_state: State::StartState,
            final_state: State::FinalState,
        }
    }
    pub fn run(&mut self, item: u8) -> Result<(), &str> {
        let mut transition: Transition = Transition::create_transition(item);
        while let State::DeadendState = self.current_state.transition(input, transition) {}
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
    pub fn transition(self, input: Transition) -> Result<State, &str> {
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
