use crate::error::Error;
use crate::grammar::Grammar;
use crate::term::Term;
use nohash_hasher::NoHashHasher;
use rustc_hash::FxHasher;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::BuildHasher;
use std::hash::BuildHasherDefault;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Hash, Eq, PartialEq, Clone)]
struct EarleyState {
    lhs: Term,
    expression: Vec<Term>,
    dot: usize,
    start: usize,
}

impl EarleyState {
    fn new(lhs: Term, expression: Vec<Term>, dot: usize, start: usize) -> Self {
        Self {
            lhs,
            expression,
            dot,
            start,
        }
    }

    fn advance(&self) -> Self {
        Self {
            lhs: self.lhs.clone(),
            expression: self.expression.clone(),
            dot: self.dot + 1,
            start: self.start,
        }
    }

    fn at_dot(&self) -> Option<&Term> {
        self.expression.get(self.dot)
    }
}

impl fmt::Display for EarleyState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{} ::= ", self.lhs);
        for i in 0..self.dot {
            s.push_str(format!("{} ", self.expression[i]).as_str());
        }
        s.push_str(". ");
        for i in self.dot..self.expression.len() {
            s.push_str(format!("{} ", self.expression[i]).as_str());
        }
        s.push_str(format!("({})", self.start).as_str());

        write!(f, "{}", s)
    }
}

struct Column {
    symbol: char,
    states: Vec<EarleyState>,
    unique: HashSet<u64, BuildHasherDefault<NoHashHasher<u64>>>,
    transitive: HashMap<Term, EarleyState, BuildHasherDefault<FxHasher>>,
    hash_builder: BuildHasherDefault<FxHasher>,
}

impl Column {
    fn new(symbol: char) -> Self {
        Self {
            symbol,
            states: Vec::new(),
            unique: Default::default(),
            transitive: Default::default(),
            hash_builder: Default::default(),
        }
    }

    fn add_transitive(&mut self, state: &EarleyState) {
        if !self.transitive.contains_key(&state.lhs) {
            self.transitive.insert(state.lhs.clone(), state.clone());
        }
    }

    fn add(&mut self, state: EarleyState) {
        let mut hasher = self.hash_builder.build_hasher();
        state.hash(&mut hasher);
        let hash = hasher.finish();
        if !self.unique.contains(&hash) {
            self.unique.insert(hash);
            self.states.push(state);
        }
    }

    fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    fn len(&self) -> usize {
        self.states.len()
    }
}

pub struct ExtendedEarleyParser {
    grammar: Grammar,
    input: Vec<char>,
    nullable: HashSet<Term, BuildHasherDefault<FxHasher>>,
    state_table: Vec<Column>,
}

impl ExtendedEarleyParser {
    fn init(&mut self, input: &str) {
        self.input = input.chars().collect();
        self.init_state_table();
        self.compute_nullable_nonterminals();
        self.seed();
    }

    fn init_state_table(&mut self) {
        self.state_table = Vec::with_capacity(self.input.len() + 1);
        self.state_table.push(Column::new('_'));
        for symbol in &self.input {
            self.state_table.push(Column::new(*symbol));
        }
    }

    fn compute_nullable_nonterminals(&mut self) {
        let mut was_updated = true;
        while was_updated {
            was_updated = false;
            for rule in self.grammar.rules.iter() {
                if !self.nullable.contains(&rule.lhs)
                    && rule.rhs.alternatives.iter().any(|terms| {
                        terms
                            .iter()
                            .all(|term| term.is_epsilon() || self.nullable.contains(term))
                    })
                {
                    self.nullable.insert(rule.lhs.clone());
                    was_updated = true;
                }
            }
        }
    }

    fn seed(&mut self) {
        let initial_rule = self
            .grammar
            .rule_lut
            .get(&self.grammar.start)
            .ok_or(Error::EmptyGrammarError(
                "cannot seed earley state table without initial rule".to_owned(),
            ))
            .unwrap();
        for expression in &initial_rule.rhs.alternatives {
            self.state_table[0].add(EarleyState::new(
                initial_rule.lhs.clone(),
                expression.to_vec(),
                0,
                0,
            ))
        }
    }

    pub fn dump(&self) -> String {
        let mut s = String::new();
        s.push_str("============================================\n");
        s.push_str("state table\n");
        s.push_str("============================================\n");
        for (start, column) in self.state_table.iter().enumerate() {
            if !column.is_empty() {
                s.push_str(
                    format!(
                        "[column: {} | symbol: '{}']\n",
                        start,
                        column.symbol.escape_default()
                    )
                    .as_str(),
                );
                for state in column.states.iter() {
                    s.push_str(format!("    {}\n", state).as_str());
                }
            }
        }
        s.push_str("============================================\n");
        s
    }

    fn scan(&mut self, col: usize, state_index: usize, symbol_opt: Option<char>) {
        if symbol_opt.is_none() || self.state_table[col].symbol == symbol_opt.unwrap() {
            let new_state = self.state_table[col - 1].states[state_index].advance();
            self.state_table[col].add(new_state);
        }
    }

    fn predict(&mut self, col: usize, state_index: usize, nonterminal: &Term) {
        for alternative in &self.grammar.rule_for(nonterminal).unwrap().rhs.alternatives {
            self.state_table[col].add(EarleyState::new(
                nonterminal.clone(),
                alternative.clone(),
                0,
                col,
            ));
        }
        if self.nullable.contains(nonterminal) {
            let new_state = self.state_table[col].states[state_index].advance();
            self.state_table[col].add(new_state);
        }
    }

    fn deterministic_reduction(&mut self, state: &EarleyState) -> Option<EarleyState> {
        if let Some(matching_parent) = self.unique_postdot(state) {
            if let Some(transitive_state) = self.state_table[state.start]
                .transitive
                .get(&matching_parent.lhs)
            {
                return Some(transitive_state.clone());
            }
            let candidate = matching_parent.advance();
            let topmost = self.deterministic_reduction(&candidate).or(Some(candidate));
            self.state_table[matching_parent.start].add_transitive(topmost.as_ref().unwrap());
            return topmost;
        }
        None
    }

    fn leo_complete(&mut self, col: usize, state_index: usize) {
        let state = self.state_table[col].states[state_index].clone();
        if let Some(topmost) = self.deterministic_reduction(&state) {
            self.state_table[col].add(topmost);
        } else {
            self.earley_complete(col, state_index);
        }
    }

    fn unique_postdot(&mut self, state: &EarleyState) -> Option<EarleyState> {
        let parents = self.state_table[state.start]
            .states
            .iter()
            .filter(|s| s.at_dot().is_some() && *s.at_dot().unwrap() == state.lhs)
            .take(2)
            .collect::<Vec<&EarleyState>>();
        if parents.len() != 1 {
            return None;
        }
        let parent = *parents.first().unwrap();
        if parent.dot == parent.expression.len() - 1 {
            return Some(parent.clone());
        }
        None
    }

    fn earley_complete(&mut self, col: usize, state_index: usize) {
        let state = &self.state_table[col].states[state_index];
        let completions = self.state_table[state.start]
            .states
            .iter()
            .filter(|s| s.at_dot().is_some() && *s.at_dot().unwrap() == state.lhs)
            .map(|parent| parent.advance())
            .collect::<Vec<EarleyState>>();
        for completion in completions {
            self.state_table[col].add(completion);
        }
    }

    fn chart_parse(&mut self) {
        let n_columns = self.state_table.len();
        for col in 0..n_columns {
            let mut state_index = 0;
            let mut n_states = self.state_table[col].len();
            while state_index < n_states {
                let state = &self.state_table[col].states[state_index];
                match state.at_dot() {
                    Some(term) if self.grammar.rule_for(term).is_some() => {
                        let term = term.clone();
                        self.predict(col, state_index, &term);
                    }
                    Some(Term::Terminal(s)) => {
                        let next_col = col + 1;
                        if next_col < n_columns {
                            let symbol = s.chars().next();
                            self.scan(next_col, state_index, symbol);
                        }
                    }
                    None => {
                        self.leo_complete(col, state_index);
                    }
                    other => panic!("encountered nonterminal which does not appear in the left-hand side of any production rule: {:?}", other),
                }
                state_index += 1;
                n_states = self.state_table[col].len();
            }
        }
    }

    pub fn recognize(grammar: &Grammar, input: &str) -> bool {
        let mut parser = Self::from(grammar);
        parser.init(input);
        parser.chart_parse();
        parser
            .state_table
            .last()
            .unwrap()
            .states
            .iter()
            .any(|state| state.at_dot().is_none() && state.lhs == parser.grammar.start)
    }
}

impl From<&Grammar> for ExtendedEarleyParser {
    fn from(grammar: &Grammar) -> Self {
        let mut grammar_clone = grammar.clone();
        grammar_clone.atomize_terminals();
        Self {
            grammar: grammar_clone,
            input: Default::default(),
            nullable: Default::default(),
            state_table: Default::default(),
        }
    }
}
