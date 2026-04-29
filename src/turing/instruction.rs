mod simple;

pub use simple::SimpleInstruction;

use super::{Direction, state::State};

#[derive(Debug)]
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

impl From<&SimpleInstruction> for Instruction {
    fn from(instr: &SimpleInstruction) -> Self {
        Self {
            start_state: State::Int(instr.0),
            start_symbol: instr.1,
            end_state: State::Int(instr.2),
            end_symbol: instr.3,
            direction: instr.4,
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Instruction {{ start_state: {}, start_symbol: {}, end_state: {}, end_symbol: {}, direction: {} }}",
            self.start_state(),
            self.start_symbol(),
            self.end_state(),
            self.end_symbol(),
            self.direction()
        )
    }
}
