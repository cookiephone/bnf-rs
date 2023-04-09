use crate::parser::EarleyState;
use crate::parser::ParsingContext;
use crate::term::Term;
use crate::types::FxHashMap;
use crate::types::FxHashSet;
use crate::types::TermKey;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) enum SppfNodeItem {
    Symbol(TermKey),
    LR0Item {
        lhs: TermKey,
        rhs: Rc<Vec<Term>>,
        dot: usize,
    },
    Epsilon,
    Null,
}

impl SppfNodeItem {
    fn dump_str(&self, context: &ParsingContext) -> String {
        match self {
            Self::Symbol(term_key) => context.grammar.term(term_key).to_string().replace('"', "'"),
            Self::LR0Item { lhs, rhs, dot } => {
                let mut s = String::new();
                s.push_str(&context.grammar.term(lhs).to_string());
                s.push_str(" ::= ");
                for i in 0..*dot {
                    s.push_str(format!("{} ", rhs[i]).replace('"', "'").as_str());
                }
                s.push_str(". ");
                for i in *dot..rhs.len() {
                    s.push_str(format!("{} ", rhs[i]).replace('"', "'").as_str());
                }
                s
            },
            Self::Epsilon => "epsilon".to_owned(),
            Self::Null => "null".to_owned(),
        }
    }
}

impl Hash for SppfNodeItem {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Symbol(term_key) => term_key.hash(state),
            Self::LR0Item { lhs, rhs, dot } => {
                lhs.hash(state);
                rhs.as_ptr().hash(state);
                dot.hash(state);
            }
            Self::Epsilon => "epsilon".hash(state),
            Self::Null => (),
        }
    }
}

impl PartialEq for SppfNodeItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::LR0Item {
                    lhs: lhs_1,
                    rhs: rhs_1,
                    dot: dot_1,
                },
                Self::LR0Item {
                    lhs: lhs_2,
                    rhs: rhs_2,
                    dot: dot_2,
                },
            ) => *lhs_1 == *lhs_2 && rhs_1 == rhs_2 && *dot_1 == *dot_2,
            (Self::Symbol(term_key_1), Self::Symbol(term_key_2)) => *term_key_1 == *term_key_2,
            (Self::Null, Self::Null) => true,
            _ => false,
        }
    }
}

impl Eq for SppfNodeItem {}

#[derive(Clone)]
pub(crate) struct SppfNodeLabel {
    pub(crate) item: SppfNodeItem,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl SppfNodeLabel {
    fn from_state(state: &EarleyState, end: usize) -> Self {
        let start = state.start;
        let item = match state.dot {
            0 | 1 => SppfNodeItem::Null,
            _ => match state.at_dot() {
                Some(_) => SppfNodeItem::LR0Item {
                    lhs: state.lhs,
                    rhs: state.expression.clone(),
                    dot: state.dot,
                },
                None => SppfNodeItem::Symbol(state.lhs),
            },
        };
        Self { item, start, end }
    }

    pub(crate) fn epsilon() -> Self {
        Self {
            item: SppfNodeItem::Epsilon,
            start: Default::default(),
            end: Default::default(),
        }
    }

    pub(crate) fn null() -> Self {
        Self {
            item: SppfNodeItem::Null,
            start: Default::default(),
            end: Default::default(),
        }
    }

    pub(crate) fn is_null(&self) -> bool {
        matches!(self.item, SppfNodeItem::Null)
    }

    fn dump_str(&self, context: &ParsingContext) -> String {
        match self.item {
            SppfNodeItem::Null | SppfNodeItem::Epsilon => self.item.dump_str(context),
            _ => format!("{} [{}, {}]", self.item.dump_str(context), self.start, self.end),
        }
    }
}

impl Hash for SppfNodeLabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.item {
            SppfNodeItem::Epsilon | SppfNodeItem::Null => self.item.hash(state),
            _ => {
                self.item.hash(state);
                self.start.hash(state);
                self.end.hash(state);
            }
        }
        
    }
}

impl PartialEq for SppfNodeLabel {
    fn eq(&self, other: &Self) -> bool {
        match self.item {
            SppfNodeItem::Epsilon | SppfNodeItem::Null => self.item == other.item,
            _ => self.item == other.item && self.start == other.start && self.end == other.end,
        }
    }
}

impl Eq for SppfNodeLabel {}

pub(crate) struct SppfNode {
    pub(crate) label: SppfNodeLabel,
    children: FxHashSet<SppfNodeLabel>,
}

impl SppfNode {
    pub(crate) fn add_child(&mut self, child: SppfNodeLabel) {
        self.children.insert(child);
    }
}

impl From<SppfNodeLabel> for SppfNode {
    fn from(label: SppfNodeLabel) -> Self {
        Self {
            label,
            children: Default::default(),
        }
    }
}

impl Hash for SppfNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.label.hash(state);
    }
}

impl PartialEq for SppfNode {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Eq for SppfNode {}

pub(crate) struct Sppf {
    nodes: FxHashMap<SppfNodeLabel, SppfNode>,
}

impl Sppf {
    fn new() -> Self {
        let mut instance = Self { nodes: Default::default() };
        instance.init();
        instance
    }

    fn init(&mut self) {
        self.insert(SppfNodeLabel::null());
          self.insert(SppfNodeLabel::epsilon());
    }

    fn insert(&mut self, label: SppfNodeLabel) -> &mut SppfNode {
        self.nodes.entry(label.clone()).or_insert(label.into())
    }

    pub(crate) fn insert_from_symbol(&mut self, symbol: TermKey, start: usize, end: usize) -> &mut SppfNode {
        let item = SppfNodeItem::Symbol(symbol);
        let label = SppfNodeLabel { item, start, end };
        self.insert(label)
    }

    pub(crate) fn insert_from_state(&mut self, state: &EarleyState, end: usize) -> &mut SppfNode {
        let label = SppfNodeLabel::from_state(state, end);
        self.insert(label)
    }

    pub(crate) fn get_node(&self, label: &SppfNodeLabel) -> &SppfNode {
        self.nodes.get(label).unwrap()
    }

    pub(crate) fn dump_dot(&self, context: &ParsingContext) -> String {
        let mut dot = String::new();
        let mut edges = String::new();
        dot.push_str("digraph FPPS {\n");
        for parent in self.nodes.values() {
            dot.push_str(&format!("    \"{}\";\n", parent.label.dump_str(context)));
            for child_label in &parent.children {
                edges.push_str(&format!("    \"{}\" -> \"{}\";\n", parent.label.dump_str(context), child_label.dump_str(context)));
            }
        }
        dot.push_str(&edges);
        dot.push('}');
        dot
    }
}

impl Default for Sppf {
    fn default() -> Self {
        Self::new()
    }
}
