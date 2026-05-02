#[derive(Debug)]
pub struct AliasFormatError {
    alias: String,
    // the alias[index] character is not valid
    index: usize,
}

impl AliasFormatError {
    pub fn new(alias: String, index: usize) -> Self {
        Self { alias, index }
    }

    pub fn alias(&self) -> &str {
        self.alias.as_str()
    }

    pub fn index(&self) -> usize {
        self.index
    }
}

impl std::fmt::Display for AliasFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The character {} on index {} of the alias {} is not allowed",
            self.alias.chars().nth(self.index).unwrap(),
            self.index,
            self.alias
        )
    }
}

impl std::error::Error for AliasFormatError {}
