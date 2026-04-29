use std::fmt;

#[derive(Debug)]
pub struct NotFoundError<T: fmt::Debug + fmt::Display> {
    elem: T,
    container: String,
}

impl<T: fmt::Debug + fmt::Display> NotFoundError<T> {
    pub(crate) fn new(elem: T, container: &str) -> Self {
        Self {
            elem,
            container: String::from(container),
        }
    }

    pub fn elem(&self) -> &T {
        &self.elem
    }

    pub fn container(&self) -> &str {
        self.container.as_str()
    }
}

impl<T: fmt::Debug + fmt::Display> fmt::Display for NotFoundError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} has not been found on {}", self.elem, self.container)
    }
}

impl<T: fmt::Debug + fmt::Display> std::error::Error for NotFoundError<T> {}
