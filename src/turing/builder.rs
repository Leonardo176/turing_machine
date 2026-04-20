use super::{Alias, Instruction, TuringMachine, state::AliasMgr, tape::Tape};
use crate::common::has_unique_elements;

pub struct TuringMachineBuilder {
    tm: TuringMachine,
    aliases: Vec<Alias>,
    instructions: Vec<Instruction>,
}

impl TuringMachineBuilder {
    pub fn new() -> Self {
        Self {
            aliases: Vec::new(),
            tm: TuringMachine {
                alias_mgr: AliasMgr::new(Vec::new()),
                instructions: Vec::new(),
                tape: Tape::new('b'),
            },
            instructions: Vec::new(),
        }
    }

    pub fn instructions(mut self, instructions: Vec<Instruction>) -> Self {
        self.instructions = instructions;
        self
    }

    pub fn tape(mut self, tape_index: usize, tape_data: &[char]) -> Self {
        self.tm.tape = Tape::from('b', tape_index, tape_data);
        self
    }

    pub fn aliases(mut self, aliases: Vec<Alias>) -> Self {
        self.aliases = aliases;
        self
    }

    pub fn build(self) -> TuringMachine {
        let mut tm = self.tm;
        let mut simple_instructions = Vec::new();

        tm.alias_mgr = AliasMgr::new(self.aliases);

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

        tm.instructions = simple_instructions;
        tm
    }
}
