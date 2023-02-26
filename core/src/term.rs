use crate::codify::Codify;
use crate::error::Error;
use std::fmt;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Term {
    Terminal(String),
    Nonterminal(String),
}

impl Term {
    pub fn terminal(&self) -> Result<String, Error> {
        match self {
            Term::Terminal(s) => Ok(s.to_string()),
            _ => Err(Error::NotATerminalError(
                "extected terminal but got nonterminal".to_owned(),
            )),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Terminal(s) => write!(f, "\"{}\"", s.escape_default()),
            Term::Nonterminal(s) => write!(f, "<{s}>"),
        }
    }
}

impl Codify for Term {
    fn codify(&self, prefix: &str) -> String {
        match self {
            Self::Terminal(s) => format!("{prefix}Term::Terminal(\"{s}\".to_owned())"),
            Self::Nonterminal(s) => format!("{prefix}Term::Nonterminal(\"{s}\".to_owned())"),
        }
    }
}
