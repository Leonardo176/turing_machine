use crate::turing::{Direction, SimpleState};
use std::cmp::Ordering;

pub struct SimpleInstruction(
    pub SimpleState,
    pub char,
    pub SimpleState,
    pub char,
    pub Direction,
);

impl SimpleInstruction {
    pub fn new(sst: SimpleState, ssym: char, nst: SimpleState, nsym: char, dir: Direction) -> Self {
        Self(sst, ssym, nst, nsym, dir)
    }
}

impl PartialEq for SimpleInstruction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for SimpleInstruction {}

impl PartialOrd for SimpleInstruction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Ordering::*;
        Some(match self.0.cmp(&other.0) {
            Less => Less,
            Greater => Greater,
            Equal => self.1.cmp(&other.1),
        })
    }
}

impl Ord for SimpleInstruction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
