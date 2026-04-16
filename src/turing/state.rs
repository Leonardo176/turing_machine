use crate::turing::{SimpleInstruction, Symbol, instruction::Instruction};

pub enum State {
    Str(String),
    Int(u64),
}

pub fn alias(name: &str) -> State {
    State::Str(String::from(name))
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
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

pub struct AliasMgr {
    aliases: Vec<Alias>,
}

// aliases must be sorted
fn check_aliases(aliases: &[Alias]) -> bool {
    let len = aliases.len();

    if len == 0 {
        return true;
    }

    for i in 0..(len - 1) {
        if aliases[i].name == aliases[i + 1].name {
            return false;
        }
    }

    true
}

impl AliasMgr {
    pub fn new(mut aliases: Vec<Alias>) -> Self {
        aliases.sort();

        if !check_aliases(&aliases) {
            // TODO: properly handle errors
            panic!("Aliases are not unique!");
        }

        Self { aliases }
    }

    fn translate_state(&self, state: &State) -> Option<u64> {
        match state {
            State::Int(num) => Some(*num),
            State::Str(string) => self
                .aliases
                .binary_search_by(|alias| alias.name.cmp(string))
                .ok()
                .map(|num| self.aliases[num].state),
        }
    }

    pub fn translate_instruction(
        &self,
        instruction: &Instruction<Symbol>,
    ) -> Option<SimpleInstruction> {
        if let (Some(start_state), Some(end_state)) = (
            self.translate_state(instruction.start_state()),
            self.translate_state(instruction.end_state()),
        ) {
            Some(SimpleInstruction(
                start_state,
                instruction.start_symbol(),
                end_state,
                instruction.end_symbol(),
                instruction.direction(),
            ))
        } else {
            None
        }
    }
}
