use super::{Alias, Instruction, TuringMachine, state::AliasMgr, tape::Tape};
use crate::{
    common::has_unique_elements,
    turing::{
        State,
        error::{
            BuilderError, DuplicateError, InstructionError, InstructionFieldError, NotFoundError,
        },
    },
};

pub struct TuringMachineBuilder<'a> {
    default_symbol: char,
    symbols: Vec<char>,
    initial_state: State,
    tape_index: usize,
    tape_data: &'a str,
    aliases: Vec<Alias>,
    instructions: Vec<Instruction>,
}

impl<'a> TuringMachineBuilder<'a> {
    pub fn new(default_symbol: char, symbols: Vec<char>) -> Self {
        Self {
            default_symbol,
            symbols,
            tape_index: 0,
            tape_data: "",
            initial_state: State::Int(0),
            aliases: Vec::new(),
            instructions: Vec::new(),
        }
    }

    pub fn instructions(mut self, instructions: Vec<Instruction>) -> Self {
        self.instructions = instructions;
        self
    }

    pub fn tape(mut self, tape_index: usize, tape_data: &'a str) -> Self {
        self.tape_index = tape_index;
        self.tape_data = tape_data;
        self
    }

    pub fn aliases(mut self, aliases: Vec<Alias>) -> Self {
        self.aliases = aliases;
        self
    }

    pub fn initial_state(mut self, initial_state: impl Into<State>) -> Self {
        self.initial_state = initial_state.into();
        self
    }

    pub fn build(self) -> Result<TuringMachine, BuilderError> {
        // build basic TM
        let mut tm = TuringMachine::new(self.default_symbol, self.symbols)?;

        // Build aliases
        tm.alias_mgr = AliasMgr::new(self.aliases)?;

        // Build initial state
        tm.current_state = tm.alias_mgr.translate_state(&self.initial_state)?;

        // Build instructions
        let mut simple_instructions = Vec::new();

        for instr in self.instructions.iter() {
            let instr = tm.alias_mgr.translate_instruction(instr)?;
            // check that symbols of instr are in tm.symbols

            if tm.symbols.binary_search(&instr.1).is_err() {
                return Err(InstructionError::new(
                    InstructionFieldError::StartSymbol(NotFoundError::new(
                        instr.1,
                        "list of symbols",
                    )),
                    Instruction::from(&instr),
                )
                .into());
            }

            if tm.symbols.binary_search(&instr.3).is_err() {
                return Err(InstructionError::new(
                    InstructionFieldError::EndSymbol(NotFoundError::new(
                        instr.3,
                        "list of symbols",
                    )),
                    Instruction::from(&instr),
                )
                .into());
            }

            simple_instructions.push(instr);
        }

        simple_instructions.sort();
        if let Err(err) = has_unique_elements(&simple_instructions, "instruction") {
            return Err(DuplicateError::new(
                tm.alias_mgr.translate_instruction_back(err.first()),
                tm.alias_mgr.translate_instruction_back(err.second()),
                err.type_name(),
            )
            .into());
        }

        tm.instructions = simple_instructions;

        // Build tape
        tm.tape = Tape::from(tm.tape.default_symbol(), self.tape_index, self.tape_data);

        Ok(tm)
    }
}
