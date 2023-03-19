use crate::codify::Codify;
use crate::error::Error;
use std::fmt;
use std::hash::Hash;

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

    pub fn is_epsilon(&self) -> bool {
        matches!(self, Term::Terminal(s) if s.is_empty())
    }

    pub fn is_atomic_terminal(&self) -> bool {
        matches!(self, Term::Terminal(s) if s.len() == 1)
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Term::Terminal(_))
    }

    pub fn is_nonterminal(&self) -> bool {
        matches!(self, Term::Nonterminal(_))
    }

    pub fn atomize(&self) -> Result<Vec<Self>, Error> {
        if let Term::Terminal(s) = self {
            Ok(s.chars().map(|c| Term::Terminal(c.to_string())).collect())
        } else {
            Err(Error::NotATerminalError(
                "expected a terminal for atomization".to_owned(),
            ))
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
