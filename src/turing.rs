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

pub type SimpleState = u64;

pub struct TuringMachine {
    alias_mgr: AliasMgr,
    instructions: Vec<SimpleInstruction>,
    tape: Tape,
}

impl TuringMachine {
    pub fn builder() -> TuringMachineBuilder {
        TuringMachineBuilder::new()
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
            .ok();

        if let Some(index_instr) = index_instr {
            let instr = &self.instructions[index_instr];

            self.tape.set_symbol(instr.3);

            match instr.4 {
                Direction::Left => self.tape.move_left(),
                Direction::Right => self.tape.move_right(),
            }

            Some(instr.2)
        } else {
            None
        }
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
