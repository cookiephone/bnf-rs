use crate::codify::Codify;
use crate::term::Term;
use itertools::Itertools;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Alternatives {
    pub alternatives: Vec<Rc<Vec<Term>>>,
}

impl Alternatives {
    pub fn new() -> Self {
        Self {
            alternatives: Vec::new(),
        }
    }

    pub fn add_alternative(&mut self, alternative: Vec<Term>) {
        self.alternatives.push(Rc::new(alternative));
    }

    pub fn merge(&self, other: &Self) -> Self {
        return Self {
            alternatives: self
                .alternatives
                .iter()
                .chain(other.alternatives.iter())
                .cloned()
                .unique()
                .collect(),
        };
    }
}

impl Default for Alternatives {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<Vec<Term>>> for Alternatives {
    fn from(alternatives: Vec<Vec<Term>>) -> Self {
        Self {
            alternatives: alternatives.into_iter().map(Rc::new).collect(),
        }
    }
}

impl fmt::Display for Alternatives {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.alternatives
                .iter()
                .map(|alternative| alternative
                    .iter()
                    .map(|term| term.to_string())
                    .intersperse(" ".into())
                    .collect::<String>())
                .intersperse(" | ".into())
                .collect::<String>()
        )
    }
}

impl Codify for Alternatives {
    fn codify(&self, prefix: &str) -> String {
        let mut s = format!("{prefix}Alternatives::from(vec![\n");
        for alternative in self.alternatives.iter() {
            s.push_str("                vec![");
            for term in alternative.iter() {
                s.push_str(&format!("{},", term.codify(prefix)));
            }
            s.push_str("],\n");
        }
        s.push_str("            ])");
        s
    }
}
