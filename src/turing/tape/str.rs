use std::fmt::Display;

use super::Tape;

impl Display for Tape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        let mut node = self.start.clone();

        str.push(node.get_symbol());

        while let Some(right) = node.get_right() {
            str.push(right.get_symbol());
            node = right;
        }

        write!(f, "{str}")
    }
}
