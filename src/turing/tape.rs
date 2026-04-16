mod node;
mod str;

use node::{Link, Node};
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

pub struct Tape<S: Default + Copy> {
    start: Link<S>,
    pos: Link<S>,
}

impl<S: Default + Copy> Tape<S> {
    pub fn new() -> Self {
        let new_node = Rc::new(Node::new());
        Self {
            start: new_node.clone(),
            pos: new_node,
        }
    }

    pub fn from(index: usize, data: &[S]) -> Self {
        let mut tape = Self::new();
        let mut pos = tape.pos.clone();
        let len = data.len();

        for i in 0..len {
            if i == index {
                pos = tape.pos.clone();
            }

            tape.set_symbol(data[i]);
            tape.move_right();
        }

        tape.pos = pos;

        tape
    }

    pub fn get_symbol(&self) -> S {
        self.pos.get_symbol()
    }

    pub fn set_symbol(&mut self, symbol: S) {
        self.pos.set_symbol(symbol);
    }

    pub fn move_left(&mut self) {
        if let Some(left) = self.pos.get_left() {
            self.pos = left;
        } else {
            let new_node = Rc::new(Node::<S>::new());
            new_node.set_right(self.pos.clone());
            self.pos.set_left(new_node.clone());

            self.start = new_node.clone();
            self.pos = new_node;
        }
    }

    pub fn move_right(&mut self) {
        if let Some(right) = self.pos.get_right() {
            self.pos = right;
        } else {
            let new_node = Rc::new(Node::<S>::new());
            new_node.set_left(self.pos.clone());
            self.pos.set_right(new_node.clone());

            self.pos = new_node;
        }
    }
}
