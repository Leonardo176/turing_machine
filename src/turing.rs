mod instruction;
mod state;
mod symbol;
mod tape;

use instruction::SimpleInstruction;
use state::AliasMgr;
pub use state::State;
pub use state::{Alias, alias};
pub use symbol::Symbol;
pub use tape::Direction;
use tape::Tape;

pub use instruction::Instruction;

pub type SimpleState = u64;

pub struct TuringMachine {
    instructions: Vec<SimpleInstruction>,
    tape: Tape<Symbol>,
}

// Checks that the instructions are unique (for all i1, i2 in instructions,
// i1.start_state == i2.start_state && i1.start_sym == i2.start_sym -> i1 = i2).
// Because the instructions are sorted by start_state and start_sym,
// we can just do a linear search and if two instructions are equal,
// the check fails.
fn check_instructions(instructions: &Vec<SimpleInstruction>) -> bool {
    let len = instructions.len();

    if len == 0 {
        return true;
    }

    for i in 0..(len - 1) {
        if instructions[i] == instructions[i + 1] {
            return false;
        }
    }

    true
}

impl TuringMachine {
    // Creates a Turing machine with an empty tape
    // and the given instructions.
    pub fn new(aliases: Vec<Alias>, instructions: Vec<Instruction<Symbol>>) -> Self {
        let aliases = AliasMgr::new(aliases);

        let mut simple_instructions = Vec::new();

        for instr in instructions.iter() {
            let instr = aliases.translate_instruction(instr);
            if let Some(instr) = instr {
                simple_instructions.push(instr);
            } else {
                panic!("Invalid instructions!");
            }
        }

        simple_instructions.sort();

        if !check_instructions(&simple_instructions) {
            // TODO: handle error properly
            panic!("Instructions are not unique!");
        }

        Self {
            instructions: simple_instructions,
            tape: Tape::new(),
        }
    }

    // Creates a Turing machine with the instructions given and
    // the given tape.
    pub fn from_tape(
        tape_index: usize,
        tape_data: &[Symbol],
        aliases: Vec<Alias>,
        instructions: Vec<Instruction<Symbol>>,
    ) -> Self {
        let mut tm = Self::new(aliases, instructions);
        tm.tape = Tape::from(tape_index, tape_data);

        tm
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
                Symbol::default(),
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
