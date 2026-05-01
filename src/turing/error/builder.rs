use super::{duplicate::DuplicateError, instruction::InstructionError, not_found::NotFoundError};
use crate::{Alias, Instruction, turing::State};

#[derive(Debug)]
pub enum BuilderError {
    Instruction(InstructionError),
    DupInstruction(DuplicateError<Instruction>),
    Alias(DuplicateError<Alias>),
    Symbol(DuplicateError<char>),
    InitialState(NotFoundError<State>),
    DefaultSymbol(NotFoundError<char>),
}

impl std::fmt::Display for BuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use BuilderError::*;
        write!(
            f,
            "{}",
            match self {
                DupInstruction(err) => err.to_string(),
                Instruction(err) => err.to_string(),
                Alias(err) => err.to_string(),
                Symbol(err) => err.to_string(),
                InitialState(err) => err.to_string(),
                DefaultSymbol(err) => err.to_string(),
            }
        )
    }
}

impl std::error::Error for BuilderError {}

impl From<InstructionError> for BuilderError {
    fn from(value: InstructionError) -> Self {
        Self::Instruction(value)
    }
}

impl From<DuplicateError<Instruction>> for BuilderError {
    fn from(value: DuplicateError<Instruction>) -> Self {
        Self::DupInstruction(value)
    }
}

impl From<DuplicateError<Alias>> for BuilderError {
    fn from(value: DuplicateError<Alias>) -> Self {
        Self::Alias(value)
    }
}

impl From<DuplicateError<char>> for BuilderError {
    fn from(value: DuplicateError<char>) -> Self {
        Self::Symbol(value)
    }
}

impl From<NotFoundError<char>> for BuilderError {
    fn from(value: NotFoundError<char>) -> Self {
        Self::DefaultSymbol(value)
    }
}

impl From<NotFoundError<State>> for BuilderError {
    fn from(value: NotFoundError<State>) -> Self {
        Self::InitialState(value)
    }
}
