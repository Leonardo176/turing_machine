use super::{Alias, Instruction, TuringMachine, state::AliasMgr, tape::Tape};
use crate::{common::has_unique_elements, turing::State};

pub struct TuringMachineBuilder<'a> {
    tm: TuringMachine,
    initial_state: State,
    tape_index: usize,
    tape_data: &'a [char],
    aliases: Vec<Alias>,
    instructions: Vec<Instruction>,
}

impl<'a> TuringMachineBuilder<'a> {
    pub fn new(default_symbol: char, symbols: Vec<char>) -> Self {
        Self {
            tape_index: 0,
            tape_data: &[],
            initial_state: State::Int(0),
            aliases: Vec::new(),
            tm: TuringMachine::new(default_symbol, symbols),
            instructions: Vec::new(),
        }
    }

    pub fn instructions(mut self, instructions: Vec<Instruction>) -> Self {
        self.instructions = instructions;
        self
    }

    pub fn tape(mut self, tape_index: usize, tape_data: &'a [char]) -> Self {
        self.tape_index = tape_index;
        self.tape_data = tape_data;
        self
    }

    pub fn aliases(mut self, aliases: Vec<Alias>) -> Self {
        self.aliases = aliases;
        self
    }

    pub fn initial_state(mut self, initial_state: State) -> Self {
        self.initial_state = initial_state;
        self
    }

    pub fn build(self) -> TuringMachine {
        let mut tm = self.tm;

        // Build aliases

        tm.alias_mgr = AliasMgr::new(self.aliases);

        // Build initial state

        match tm.alias_mgr.translate_state(&self.initial_state) {
            Some(state) => tm.current_state = state,
            None => {
                panic!("Initial state not found in state aliases!");
            }
        }

        // Build instructions

        let mut simple_instructions = Vec::new();

        for instr in self.instructions.iter() {
            match tm.alias_mgr.translate_instruction(instr) {
                Some(instr) => simple_instructions.push(instr),
                None => (),
            }
        }

        simple_instructions.sort();

        if !has_unique_elements(&simple_instructions) {
            panic!("Error: instructions are not unique!");
        }

        // Build tape
        tm.tape = Tape::from(tm.tape.default_symbol(), self.tape_index, self.tape_data);

        tm.instructions = simple_instructions;
        tm
    }
}
