use crate::codify::Codify;
use crate::error::Error;
use crate::types::TermKey;
use rustc_hash::FxHasher;
use std::fmt;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Term {
    pub(crate) key: TermKey,
    pub(crate) content: String,
    pub(crate) kind: TermKind,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TermKind {
    Terminal,
    Nonterminal,
}

impl Term {
    fn precompute_hash(&mut self) {
        let mut hasher = FxHasher::default();
        self.content.hash(&mut hasher);
        self.kind.hash(&mut hasher);
        self.key = hasher.finish();
    }

    pub fn terminal(content: &str) -> Self {
        let mut term = Self {
            key: Default::default(),
            content: content.to_owned(),
            kind: TermKind::Terminal,
        };
        term.precompute_hash();
        term
    }

    pub fn nonterminal(content: &str) -> Self {
        let mut term = Self {
            key: Default::default(),
            content: content.to_owned(),
            kind: TermKind::Nonterminal,
        };
        term.precompute_hash();
        term
    }

    pub fn epsilon() -> Self {
        Self::terminal("")
    }

    pub fn atomic_terminal_content(&self) -> char {
        self.content.chars().next().unwrap()
    }

    pub fn terminal_content(&self) -> Result<&String, Error> {
        match self.kind {
            TermKind::Terminal => Ok(&self.content),
            _ => Err(Error::NotATerminalError(
                "extected terminal but got nonterminal".to_owned(),
            )),
        }
    }

    pub fn is_epsilon(&self) -> bool {
        matches!(self.kind, TermKind::Terminal if self.content.is_empty())
    }

    pub fn is_atomic_terminal(&self) -> bool {
        matches!(self.kind, TermKind::Terminal if self.content.len() == 1)
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self.kind, TermKind::Terminal)
    }

    pub fn is_nonterminal(&self) -> bool {
        matches!(self.kind, TermKind::Nonterminal)
    }

    pub fn atomize(&self) -> Result<Vec<Self>, Error> {
        match self.kind {
            TermKind::Terminal => Ok(self
                .content
                .chars()
                .map(|c| Self::terminal(&c.to_string()))
                .collect()),
            TermKind::Nonterminal => Err(Error::NotATerminalError(
                "expected a terminal for atomization".to_owned(),
            )),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            TermKind::Terminal => write!(f, "\"{}\"", self.content.escape_default()),
            TermKind::Nonterminal => write!(f, "<{}>", self.content),
        }
    }
}

impl Codify for Term {
    fn codify(&self, prefix: &str) -> String {
        match self.kind {
            TermKind::Terminal => {
                format!("{prefix}Term::terminal(\"{}\")", self.content)
            }
            TermKind::Nonterminal => {
                format!("{prefix}Term::nonterminal(\"{}\")", self.content)
            }
        }
    }
}
