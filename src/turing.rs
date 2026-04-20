mod builder;
mod instruction;
mod state;
mod tape;

use instruction::SimpleInstruction;
use state::AliasMgr;
pub use state::State;
pub use state::{Alias, alias};
pub use tape::Direction;
use tape::Tape;

pub use instruction::Instruction;

use builder::TuringMachineBuilder;

use crate::common::has_unique_elements;

pub type SimpleState = u64;

pub struct TuringMachine {
    alias_mgr: AliasMgr,
    instructions: Vec<SimpleInstruction>,
    symbols: Vec<char>,
    tape: Tape,
}

impl TuringMachine {
    pub fn new(default_symbol: char, mut symbols: Vec<char>) -> Self {
        symbols.sort();

        if !has_unique_elements(&symbols) {
            panic!("Symbols are not unique!");
        }

        Self {
            alias_mgr: AliasMgr::new(Vec::new()),
            instructions: Vec::new(),
            symbols,
            tape: Tape::new(default_symbol),
        }
    }

    pub fn builder<'a>(default_symbol: char, symbols: Vec<char>) -> TuringMachineBuilder<'a> {
        TuringMachineBuilder::new(default_symbol, symbols)
    }

    // It computes a single instruction. It returns the next state that
    // the Turing machine needs to be to compute the next instruction
    // (None if no instruction was found (it has terminated)).
    fn compute_single(&mut self, current_state: SimpleState) -> Option<SimpleState> {
        let index_instr = self
            .instructions
            .binary_search(&SimpleInstruction::new(
                current_state,
                self.tape.get_symbol(),
                0,
                char::default(),
                Direction::Left,
            ))
            .ok()?;

        let instr = &self.instructions[index_instr];

        // check that the symbols of the instruction are in the array of symbols
        match (
            self.symbols.binary_search(&instr.1),
            self.symbols.binary_search(&instr.3),
        ) {
            (Ok(_), Ok(_)) => (),
            _ => {
                // TODO: handle error properly
                panic!("Symbols in an instruction are not valid!");
            }
        }

        self.tape.set_symbol(instr.3);

        match instr.4 {
            Direction::Left => self.tape.move_left(),
            Direction::Right => self.tape.move_right(),
        }

        Some(instr.2)
    }

    // Computes the instructions until it terminates.
    pub fn compute(&mut self) {
        let mut current_state = 0;

        while let Some(state) = self.compute_single(current_state) {
            current_state = state;
        }
    }

    // Get the current tape.
    pub fn get_tape(&self) -> String {
        self.tape.to_string()
    }
}
