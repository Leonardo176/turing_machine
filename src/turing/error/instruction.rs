use crate::{
    Instruction,
    turing::error::{not_found::NotFoundError, state::StateError},
};

#[derive(Debug)]
pub enum InstructionFieldError {
    StartState(StateError),
    StartSymbol(NotFoundError<char>),
    EndState(StateError),
    EndSymbol(NotFoundError<char>),
}

impl std::fmt::Display for InstructionFieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use InstructionFieldError::*;
        write!(
            f,
            "{}",
            match self {
                StartState(err) => err.to_string(),
                StartSymbol(err) => err.to_string(),
                EndState(err) => err.to_string(),
                EndSymbol(err) => err.to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct InstructionError {
    field: InstructionFieldError,
    instruction: Instruction,
}

impl InstructionError {
    pub(crate) fn new(field: InstructionFieldError, instruction: Instruction) -> Self {
        Self { field, instruction }
    }

    pub fn field(&self) -> &InstructionFieldError {
        &self.field
    }

    pub fn instruction(&self) -> &Instruction {
        &self.instruction
    }
}

impl std::fmt::Display for InstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "instruction error on {}: {}",
            self.instruction, self.field
        )
    }
}

impl std::error::Error for InstructionError {}
