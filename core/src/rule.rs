use crate::alternatives::Alternatives;
use crate::codify::Codify;
use crate::term::Term;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub lhs: Rc<Term>,
    pub rhs: Alternatives,
}

impl Rule {
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            lhs: self.lhs.clone(),
            rhs: self.rhs.merge(&other.rhs),
        }
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ::= {}", self.lhs, self.rhs)
    }
}

impl Codify for Rule {
    fn codify(&self, prefix: &str) -> String {
        format!(
            "    {prefix}Rule {{\n        lhs: std::rc::Rc::new({}),\n        rhs: {}\n    }}",
            self.lhs.codify(prefix),
            self.rhs.codify(prefix)
        )
    }
}
