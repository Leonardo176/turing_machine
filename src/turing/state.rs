mod alias;
use crate::turing::error::AliasFormatError;
pub use alias::{Alias, AliasMgr};

fn is_normal_alias(alias: &str) -> Result<(), usize> {
    let mut i = 0;
    for ch in alias.chars() {
        match ch {
            'a'..='z' | 'A'..='Z' | '_' => (),
            _ => return Err(i),
        }
        i += 1;
    }

    Ok(())
}

#[derive(Clone, Debug)]
pub enum State {
    Str(String),
    Int(u64),
}

impl State {
    pub fn new(alias: &str, offset: u64) -> Self {
        if is_normal_alias(alias).is_err() {
            Self::from(alias)
        } else {
            Self::Str(alias.to_owned() + "+" + offset.to_string().as_str())
        }
    }

    pub fn scompose(&self) -> Result<(Option<String>, u64), AliasFormatError> {
        match self {
            State::Int(st) => Ok((None, *st)),
            State::Str(st) => {
                let (state, offset) = match st.split_once('+') {
                    Some(st) => st,
                    None => (st.as_str(), "0"),
                };

                if let Err(err) = is_normal_alias(state) {
                    return Err(AliasFormatError::new(st.to_owned(), err));
                }

                let offset = match u64::from_str_radix(offset, 10) {
                    Ok(offset) => offset,
                    Err(_) => {
                        return Err(AliasFormatError::new(
                            st.to_owned(),
                            state.len()
                                + offset
                                    .chars()
                                    .enumerate()
                                    .find(|(_, ch)| '0' <= *ch && *ch <= '9')
                                    .unwrap()
                                    .0,
                        ));
                    }
                };

                Ok((Some(state.to_owned()), offset))
            }
        }
    }
}

impl From<u64> for State {
    fn from(value: u64) -> Self {
        Self::Int(value)
    }
}

impl From<&str> for State {
    fn from(value: &str) -> Self {
        Self::Str(value.to_owned())
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                State::Str(str) => str.to_owned(),
                State::Int(int) => int.to_string(),
            }
        )
    }
}
