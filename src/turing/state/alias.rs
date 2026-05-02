use super::State;
use crate::{
    common::has_unique_elements_by,
    turing::{Instruction, SimpleInstruction, SimpleState, error::*},
};

#[derive(Clone, Debug)]
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

// This is the alias state manager.
// It allows to store the aliases of the states and
// to translate back and forth between the aliases and the numbers.
// Every alias string is associated with exactly one number and
// every number (in the aliases) is associated with exactly one alias string.
// (forall a1, a2 in aliases)(a1.name == a2.name <--> a1.value == a2.value)
pub struct AliasMgr {
    aliases: Vec<Alias>,
}

impl AliasMgr {
    fn are_aliases_one_to_one(aliases: &mut [Alias]) -> Result<(), DuplicateError<Alias>> {
        aliases.sort_unstable_by_key(|alias| alias.state);

        has_unique_elements_by(aliases, |a1, a2| a1.state == a2.state, "StateAlias")?;

        aliases.sort_unstable_by_key(|alias| alias.name.clone());

        has_unique_elements_by(aliases, |a1, a2| a1.name == a2.name, "StateAlias")
    }

    pub fn new(mut aliases: Vec<Alias>) -> Result<Self, DuplicateError<Alias>> {
        AliasMgr::are_aliases_one_to_one(&mut aliases)?;

        Ok(Self { aliases })
    }

    pub fn translate_state(&self, state: &State) -> Result<SimpleState, StateError> {
        let (base, mut simple_state) = state.scompose()?;
        if let Some(base) = base {
            simple_state += self
                .aliases
                .binary_search_by(|alias| alias.name.cmp(&base))
                .map(|num| self.aliases[num].state)
                .map_err(|_| StateError::from(NotFoundError::new(base, "list of aliases")))?;
        }

        Ok(simple_state)
    }

    pub fn translate_state_back(&self, state: SimpleState) -> State {
        for st in self.aliases.iter() {
            if st.state == state {
                return st.name.as_str().into();
            }
        }
        State::Int(state)
    }

    pub fn translate_instruction(
        &self,
        instruction: &Instruction,
    ) -> Result<SimpleInstruction, InstructionError> {
        let start_state = self
            .translate_state(instruction.start_state())
            .map_err(|err| {
                InstructionError::new(
                    InstructionFieldError::StartState(err.into()),
                    instruction.clone(),
                )
            })?;
        let end_state = self
            .translate_state(instruction.end_state())
            .map_err(|err| {
                InstructionError::new(
                    InstructionFieldError::EndState(err.into()),
                    instruction.clone(),
                )
            })?;

        Ok(SimpleInstruction(
            start_state,
            instruction.start_symbol(),
            end_state,
            instruction.end_symbol(),
            instruction.direction(),
        ))
    }

    pub fn translate_instruction_back(&self, instruction: &SimpleInstruction) -> Instruction {
        let start_state = self.translate_state_back(instruction.0);
        let end_state = self.translate_state_back(instruction.2);
        Instruction::new(
            start_state,
            instruction.1,
            end_state,
            instruction.3,
            instruction.4,
        )
    }
}
