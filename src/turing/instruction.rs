use super::*;
use std::cmp::Ordering;

pub struct Instruction(pub State, pub Symbol, pub State, pub Symbol, pub Direction);

impl Instruction {
    pub fn new(sst: State, ssym: Symbol, nst: State, nsym: Symbol, dir: Direction) -> Self {
        Self(sst, ssym, nst, nsym, dir)
    }
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Instruction {}

impl PartialOrd for Instruction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Ordering::*;
        Some(match self.0.cmp(&other.0) {
            Less => Less,
            Greater => Greater,
            Equal => self.1.cmp(&other.1),
        })
    }
}

impl Ord for Instruction {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
