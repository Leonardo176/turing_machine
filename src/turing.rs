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
    current_state: SimpleState,
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

        // if default_symbol is not in symbols
        if symbols.binary_search(&default_symbol).is_err() {
            panic!("Default symbol is not in symbols!");
        }

        Self {
            alias_mgr: AliasMgr::new(Vec::new()),
            current_state: 0,
            instructions: Vec::new(),
            symbols,
            tape: Tape::new(default_symbol),
        }
    }

    pub fn builder<'a>(default_symbol: char, symbols: Vec<char>) -> TuringMachineBuilder<'a> {
        TuringMachineBuilder::new(default_symbol, symbols)
    }

    // It computes a single instruction. It returns the executed instruction.
    // The instruction returned has always the state as integers.
    fn compute_single(&mut self) -> Option<Instruction> {
        let index_instr = self
            .instructions
            .binary_search(&SimpleInstruction::new(
                self.current_state,
                self.tape.get_symbol(),
                0,
                char::default(),
                Direction::Left,
            ))
            .ok()?;

        let instr = &self.instructions[index_instr];

        // check that the symbols of the instruction are in the array of symbols
        if self.symbols.binary_search(&instr.1).is_err()
            || self.symbols.binary_search(&instr.3).is_err()
        {
            // TODO: handle error properly
            panic!("One of the symbols in an instruction is not in the symbols!");
        }

        self.current_state = instr.2;
        self.tape.set_symbol(instr.3);

        match instr.4 {
            Direction::Left => self.tape.move_left(),
            Direction::Right => self.tape.move_right(),
        }

        Some(Instruction::from(instr))
    }

    // Computes the instructions until it terminates.
    pub fn compute(&mut self) {
        while self.compute_single().is_some() {}
    }

    // Get the current tape.
    pub fn get_tape(&self) -> String {
        self.tape.to_string()
    }
}
