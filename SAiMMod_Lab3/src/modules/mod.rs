
pub mod source;
pub mod consumer;
pub mod channel;

pub trait Module {
    fn set_state(&mut self, state: State);
    fn state(&self) -> State;
}

#[derive(Clone, PartialEq)]
pub enum State {
    Closed(Message),
    Open,
}

#[derive(Clone, PartialEq)]
pub struct Message {
    live_time: u8,
}

impl Message {
    pub fn new() -> Message {
        Message { live_time: 0 }
    }
}
