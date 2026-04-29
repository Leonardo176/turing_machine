use super::{duplicate::DuplicateError, instruction::InstructionError, not_found::NotFoundError};
use crate::{
    Alias,
    turing::{State, instruction::SimpleInstruction},
};

#[derive(Debug)]
pub enum BuilderError {
    Instruction(InstructionError),
    DupInstruction(DuplicateError<SimpleInstruction>),
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
