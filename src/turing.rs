mod builder;
pub mod error;
mod instruction;
mod state;
mod tape;

pub use instruction::Instruction;
pub use state::{Alias, State};
pub use tape::Direction;

use crate::common::has_unique_elements;
use builder::TuringMachineBuilder;
use error::{BuilderError, NotFoundError};
use instruction::SimpleInstruction;
use state::AliasMgr;
use tape::Tape;

type SimpleState = u64;

pub struct TuringMachine {
    alias_mgr: AliasMgr,
    current_state: SimpleState,
    instructions: Vec<SimpleInstruction>,
    symbols: Vec<char>,
    tape: Tape,
}

impl TuringMachine {
    pub fn new(default_symbol: char, mut symbols: Vec<char>) -> Result<Self, BuilderError> {
        symbols.sort();

        if let Err(err) = has_unique_elements(&symbols, "symbol") {
            return Err(BuilderError::Symbol(err));
        }

        // if default_symbol is not in symbols
        if symbols.binary_search(&default_symbol).is_err() {
            return Err(BuilderError::DefaultSymbol(NotFoundError::new(
                default_symbol,
                "list of symbols",
            )));
        }

        Ok(Self {
            alias_mgr: AliasMgr::new(Vec::new()).unwrap(),
            current_state: 0,
            instructions: Vec::new(),
            symbols,
            tape: Tape::new(default_symbol),
        })
    }

    pub fn builder<'a>(default_symbol: char, symbols: Vec<char>) -> TuringMachineBuilder<'a> {
        TuringMachineBuilder::new(default_symbol, symbols)
    }

    // It computes a single instruction. Returns the executed instruction.
    // The instruction returned has always its states as integers.
    pub fn compute_single(&mut self) -> Option<Instruction> {
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

        // all instructions have known symbols (check builder.rs)

        self.current_state = instr.2;
        self.tape.set_symbol(instr.3);

        match instr.4 {
            Direction::Left => self.tape.move_left(),
            Direction::Right => self.tape.move_right(),
        }

        Some(Instruction::from(instr))
    }

    // Computes instructions until it terminates.
    pub fn compute(&mut self) {
        while self.compute_single().is_some() {}
    }

    // Get the current tape.
    pub fn get_tape(&self) -> String {
        self.tape.to_string()
    }
}
