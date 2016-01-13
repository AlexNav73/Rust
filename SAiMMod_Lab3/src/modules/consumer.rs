
use super::*;

pub struct Consumer {
    messages: Vec<Message>,
}

impl Consumer {
    pub fn new() -> Consumer {
        Consumer {
            messages: Vec::new()
        }
    }
}

impl Module for Consumer {
    fn set_state(&mut self, _: State) {
    }

    fn state(&self) -> State {
        State::Open
    }
}
