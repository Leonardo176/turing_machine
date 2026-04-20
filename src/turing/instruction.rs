mod simple;

pub use simple::SimpleInstruction;

use super::{Direction, state::State};

pub struct Instruction {
    start_state: State,
    start_symbol: char,
    end_state: State,
    end_symbol: char,
    direction: Direction,
}

impl Instruction {
    pub fn new(
        start_state: State,
        start_symbol: char,
        end_state: State,
        end_symbol: char,
        direction: Direction,
    ) -> Self {
        Self {
            start_state,
            start_symbol,
            end_state,
            end_symbol,
            direction,
        }
    }

    pub fn start_state(&self) -> &State {
        &self.start_state
    }

    pub fn start_symbol(&self) -> char {
        self.start_symbol
    }

    pub fn end_state(&self) -> &State {
        &self.end_state
    }

    pub fn end_symbol(&self) -> char {
        self.end_symbol
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }
}
