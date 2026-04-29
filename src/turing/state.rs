use crate::{
    common::has_unique_elements,
    turing::{SimpleInstruction, SimpleState, instruction::Instruction},
};

#[derive(Debug)]
pub enum State {
    Str(String),
    Int(u64),
}

pub fn alias(name: &str) -> State {
    State::Str(String::from(name))
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Str(str) => str.to_owned(),
                State::Int(int) => int.to_string(),
            }
        )
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Alias {
    name: String,
    state: u64,
}

impl Alias {
    pub fn new(name: &str, state: u64) -> Self {
        Self {
            name: String::from(name),
            state,
        }
    }
}

impl std::fmt::Display for Alias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct AliasMgr {
    aliases: Vec<Alias>,
}

impl AliasMgr {
    pub fn new(mut aliases: Vec<Alias>) -> Self {
        aliases.sort();

        if !has_unique_elements(&aliases) {
            // TODO: properly handle errors
            panic!("Aliases are not unique!");
        }

        Self { aliases }
    }

    pub fn translate_state(&self, state: &State) -> Option<SimpleState> {
        match state {
            State::Int(num) => Some(*num),
            State::Str(string) => self
                .aliases
                .binary_search_by(|alias| alias.name.cmp(string))
                .ok()
                .map(|num| self.aliases[num].state),
        }
    }

    pub fn translate_instruction(&self, instruction: &Instruction) -> Option<SimpleInstruction> {
        let start_state = self.translate_state(instruction.start_state())?;
        let end_state = self.translate_state(instruction.end_state())?;

        Some(SimpleInstruction(
            start_state,
            instruction.start_symbol(),
            end_state,
            instruction.end_symbol(),
            instruction.direction(),
        ))
    }
}
