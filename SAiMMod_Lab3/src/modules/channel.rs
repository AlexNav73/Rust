
use super::*;

pub struct Channel<'a, T> {
    state: State,
    next: Option<T>,
    addition_action: Box<FnMut(&mut State) + 'a>,
}

impl<'a, T> Channel<'a, T> where T: Module {

    pub fn new<A: FnMut(&mut State) + 'a>(action: Box<A>) -> Channel<'a, T> {
        Channel {
            state: State::Open,
            next: None,
            addition_action: action
        }
    }

    pub fn set_next(&mut self, module: T) {
        self.next = Some(module);
    }

    pub fn reject(&mut self, state: &mut State) {
        self.state = State::Open;
    }

    pub fn block(&mut self, state: &mut State) { }

}

impl<'a, T> Module for Channel<'a, T> where T: Module {

    fn set_state(&mut self, state: State) {
        self.state = state;
        if let Some(ref mut next) = self.next {
            if let State::Closed(m) = next.state() {
                if self.state == State::Open {
                    self.state = State::Closed(m);
                    next.set_state(State::Open);
                }
            }
        }
        (*self.addition_action)(&mut self.state);
    }

    fn state(&self) -> State {
        self.state.clone()
    }

}
