use std::fmt::Display;

use super::Tape;

impl<S: Default + Copy + Display> Display for Tape<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        let mut node = self.start.clone();

        str.push_str(node.get_symbol().to_string().as_str());

        while let Some(right) = node.get_right() {
            str.push_str(right.get_symbol().to_string().as_str());
            node = right;
        }

        write!(f, "{str}")
    }
}
