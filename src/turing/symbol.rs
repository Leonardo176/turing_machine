use std::fmt::Display;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Symbol {
    Blank,
    Zero,
}

impl Default for Symbol {
    fn default() -> Self {
        Symbol::Blank
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Blank => write!(f, "b"),
            Symbol::Zero => write!(f, "0"),
        }
    }
}
