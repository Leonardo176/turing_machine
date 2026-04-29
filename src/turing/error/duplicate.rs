use std::fmt;

#[derive(Debug)]
pub struct DuplicateError<T: fmt::Debug + fmt::Display> {
    first: T,
    second: T,
    type_name: String,
}

impl<T: fmt::Debug + fmt::Display> DuplicateError<T> {
    pub(crate) fn new(first: T, second: T, type_name: &str) -> Self {
        Self {
            first,
            second,
            type_name: String::from(type_name),
        }
    }

    pub fn first(&self) -> &T {
        &self.first
    }

    pub fn second(&self) -> &T {
        &self.second
    }

    pub fn type_name(&self) -> &str {
        self.type_name.as_str()
    }
}

impl<T: fmt::Debug + fmt::Display> fmt::Display for DuplicateError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} and {} are duplicate {}",
            self.first, self.second, self.type_name
        )
    }
}

impl<T: fmt::Debug + fmt::Display> std::error::Error for DuplicateError<T> {}
