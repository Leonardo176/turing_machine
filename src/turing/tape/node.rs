use std::{
    cell::{Cell, OnceCell},
    rc::{Rc, Weak},
};

pub type Link = Rc<Node>;
pub type WeakLink = Weak<Node>;

pub struct Node {
    left: OnceCell<WeakLink>,
    right: OnceCell<Link>,
    symbol: Cell<char>,
}

impl Node {
    pub fn get_symbol(&self) -> char {
        self.symbol.get()
    }

    pub fn set_symbol(&self, symbol: char) {
        self.symbol.set(symbol);
    }

    pub fn get_left(&self) -> Option<Link> {
        self.left.get().map(|link| link.upgrade()).flatten()
    }

    pub fn set_left(&self, back: Link) {
        self.left
            .set(Rc::downgrade(&back))
            .map_err(|_| back)
            .ok()
            .unwrap()
    }

    pub fn get_right(&self) -> Option<Link> {
        self.right.get().cloned()
    }

    pub fn set_right(&self, next: Link) {
        self.right.set(next).ok().unwrap()
    }
}

impl From<char> for Node {
    fn from(symbol: char) -> Self {
        Self {
            left: OnceCell::new(),
            right: OnceCell::new(),
            symbol: Cell::new(symbol),
        }
    }
}
