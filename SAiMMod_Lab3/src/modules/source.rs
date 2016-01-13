
use super::*;

pub struct Source<T: Module> {
    state: State,
    next: Option<T>,
}

impl<T: Module> Source<T> {
    pub fn new() -> Source<T> {
        Source {
            state: State::Open,
            next: None,
        }
    }

    pub fn set_next(&mut self, module: T) {
        self.next = Some(module);
    }
}

impl<T: Module> Module for Source<T> {

    fn set_state(&mut self, _: State) {
        if let Some(ref mut next) = self.next {
            if let State::Closed(ref m) = next.state() {
            }
        }
    }

    fn state(&self) -> State {
        self.state.clone()
    }

}
