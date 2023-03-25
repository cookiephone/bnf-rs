use crate::parser::EarleyState;
use crate::term::Term;
use crate::types::FxHashMap;
use crate::types::FxHashSet;
use crate::types::TermKey;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

#[derive(Clone)]
enum SPPFNodeItem {
    Symbol(TermKey),
    LR0Item {
        expression: Rc<Vec<Term>>,
        dot: usize,
    },
    Null,
}

impl Hash for SPPFNodeItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Symbol(term_key) => term_key.hash(state),
            Self::LR0Item { expression, dot } => {
                expression.as_ptr().hash(state);
                dot.hash(state);
            }
            Self::Null => (),
        }
    }
}

impl PartialEq for SPPFNodeItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::LR0Item {
                    expression: expression_1,
                    dot: dot_1,
                },
                Self::LR0Item {
                    expression: expression_2,
                    dot: dot_2,
                },
            ) => expression_1 == expression_2 && dot_1 == dot_2,
            (Self::Symbol(term_key_1), Self::Symbol(term_key_2)) => term_key_1 == term_key_2,
            (Self::Null, Self::Null) => true,
            _ => false,
        }
    }
}

impl Eq for SPPFNodeItem {}

#[derive(Clone)]
struct SPPFNodeLabel {
    item: SPPFNodeItem,
    start: usize,
    end: usize,
}

impl SPPFNodeLabel {
    fn from_state(state: &EarleyState, end: usize) -> SPPFNodeLabel {
        let start = state.start;
        let item = match state.dot {
            0 | 1 => SPPFNodeItem::Null,
            _ => match state.at_dot() {
                Some(_) => SPPFNodeItem::LR0Item {
                    expression: state.expression.clone(),
                    dot: state.dot,
                },
                None => SPPFNodeItem::Symbol(state.lhs),
            },
        };
        Self { item, start, end }
    }

    fn null() -> Self {
        Self {
            item: SPPFNodeItem::Null,
            start: Default::default(),
            end: Default::default(),
        }
    }
}

impl Hash for SPPFNodeLabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.item.hash(state);
        self.start.hash(state);
        self.end.hash(state);
    }
}

impl PartialEq for SPPFNodeLabel {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item && self.start == other.start && self.end == other.end
    }
}

impl Eq for SPPFNodeLabel {}

struct SPPFNode {
    label: SPPFNodeLabel,
    children: FxHashSet<SPPFNode>,
}

impl SPPFNode {
    fn null() -> Self {
        Self {
            label: SPPFNodeLabel::null(),
            children: Default::default(),
        }
    }
}

impl From<SPPFNodeLabel> for SPPFNode {
    fn from(label: SPPFNodeLabel) -> Self {
        Self {
            label,
            children: Default::default(),
        }
    }
}

impl Hash for SPPFNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label.hash(state);
    }
}

impl PartialEq for SPPFNode {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Eq for SPPFNode {}

struct SPPF {
    nodes: FxHashMap<SPPFNodeLabel, SPPFNode>,
}

impl SPPF {
    fn insert(&mut self, label: SPPFNodeLabel) -> &mut SPPFNode {
        self.nodes.entry(label.clone()).or_insert(label.into())
    }

    fn get_node(&self, label: &SPPFNodeLabel) -> Option<&SPPFNode> {
        self.nodes.get(label)
    }

    fn get_node_mut(&mut self, label: &SPPFNodeLabel) -> Option<&mut SPPFNode> {
        self.nodes.get_mut(label)
    }
}
