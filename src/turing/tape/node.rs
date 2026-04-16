use std::{
    cell::{Cell, OnceCell},
    rc::{Rc, Weak},
};

pub type Link<T> = Rc<Node<T>>;
pub type WeakLink<T> = Weak<Node<T>>;

pub struct Node<S: Default + Copy> {
    left: OnceCell<WeakLink<S>>,
    right: OnceCell<Link<S>>,
    symbol: Cell<S>,
}

impl<S: Default + Copy> Node<S> {
    pub fn new() -> Self {
        Self {
            left: OnceCell::new(),
            right: OnceCell::new(),
            symbol: Cell::default(),
        }
    }

    pub fn get_symbol(&self) -> S {
        self.symbol.get()
    }

    pub fn set_symbol(&self, symbol: S) {
        self.symbol.set(symbol);
    }

    pub fn get_left(&self) -> Option<Link<S>> {
        self.left.get().map(|link| link.upgrade()).flatten()
    }

    pub fn set_left(&self, back: Link<S>) {
        self.left
            .set(Rc::downgrade(&back))
            .map_err(|_| back)
            .ok()
            .unwrap()
    }

    pub fn get_right(&self) -> Option<Link<S>> {
        self.right.get().cloned()
    }

    pub fn set_right(&self, next: Link<S>) {
        self.right.set(next).ok().unwrap()
    }
}

impl<S: Default + Copy> From<S> for Node<S> {
    fn from(symbol: S) -> Self {
        Self {
            left: OnceCell::new(),
            right: OnceCell::new(),
            symbol: Cell::new(symbol),
        }
    }
}
