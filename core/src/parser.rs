use crate::error::Error;
use crate::grammar::Grammar;
use crate::term::Term;
use crate::term::TermKind;
use crate::types::NoHashMap;
use crate::types::NoHashSet;
use crate::types::StateKey;
use crate::types::TermKey;
use rustc_hash::FxHasher;
use std::fmt;
use std::hash::BuildHasher;
use std::hash::BuildHasherDefault;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

#[derive(Eq, PartialEq, Clone)]
struct EarleyState {
    lhs: TermKey,
    expression: Rc<Vec<Term>>,
    dot: usize,
    start: usize,
}

impl EarleyState {
    fn new(lhs: TermKey, expression: Rc<Vec<Term>>, dot: usize, start: usize) -> Self {
        Self {
            lhs,
            expression,
            dot,
            start,
        }
    }

    fn advance(&self) -> Self {
        Self {
            lhs: self.lhs,
            expression: self.expression.clone(),
            dot: self.dot + 1,
            start: self.start,
        }
    }

    fn at_dot(&self) -> Option<&Term> {
        self.expression.get(self.dot)
    }
}

impl Hash for EarleyState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lhs.hash(state);
        self.expression.as_ptr().hash(state);
        self.dot.hash(state);
        self.start.hash(state);
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
    unique: NoHashSet<StateKey>,
    transitive: NoHashMap<StateKey, EarleyState>,
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
        self.transitive
            .entry(state.lhs)
            .or_insert_with(|| state.clone());
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

struct ParsingContext {
    grammar: Grammar,
    input: Vec<char>,
    nullable: NoHashSet<TermKey>,
}

impl From<&Grammar> for ParsingContext {
    fn from(grammar: &Grammar) -> Self {
        let mut instance = Self {
            grammar: grammar.clone(),
            input: Default::default(),
            nullable: Default::default(),
        };
        instance.init();
        instance
    }
}

impl ParsingContext {
    fn init(&mut self) {
        self.grammar.atomize_terminals();
        self.compute_nullable_nonterminals();
    }

    fn compute_nullable_nonterminals(&mut self) {
        let mut was_updated = true;
        while was_updated {
            was_updated = false;
            for rule in self.grammar.rules.iter() {
                if !self.nullable.contains(&rule.lhs.key)
                    && rule.rhs.alternatives.iter().any(|terms| {
                        terms
                            .iter()
                            .all(|term| term.is_epsilon() || self.nullable.contains(&term.key))
                    })
                {
                    self.nullable.insert(rule.lhs.key);
                    was_updated = true;
                }
            }
        }
    }
}

#[derive(Default)]
struct ParsingState {
    state_table: Vec<Column>,
}

impl ParsingState {
    fn insert(&mut self, col: usize, state: EarleyState) {
        self.state_table[col].add(state);
    }

    fn get(&self, col: usize, state_index: usize) -> &EarleyState {
        &self.state_table[col].states[state_index]
    }

    fn predict(
        &mut self,
        context: &ParsingContext,
        col: usize,
        state_index: usize,
        nonterminal: TermKey,
    ) {
        for alternative in &context.grammar.rule(nonterminal).rhs.alternatives {
            let new_state = EarleyState::new(nonterminal, alternative.clone(), 0, col);
            self.insert(col, new_state);
        }
        if context.nullable.contains(&nonterminal) {
            let new_state = self.get(col, state_index).advance();
            self.insert(col, new_state);
        }
    }

    fn scan(&mut self, col: usize, state_index: usize, symbol_opt: Option<char>) {
        if symbol_opt.is_none() || self.state_table[col].symbol == symbol_opt.unwrap() {
            let new_state = self.get(col - 1, state_index).advance();
            self.insert(col, new_state);
        }
    }

    fn leo_complete(&mut self, col: usize, state_index: usize) {
        let state = self.get(col, state_index).clone();
        if let Some(topmost) = self.deterministic_reduction(&state) {
            self.insert(col, topmost);
        } else {
            self.earley_complete(col, state_index);
        }
    }

    fn earley_complete(&mut self, col: usize, state_index: usize) {
        let state = self.get(col, state_index);
        let completions = self.state_table[state.start]
            .states
            .iter()
            .filter(|s| s.at_dot().is_some() && s.at_dot().unwrap().key == state.lhs)
            .map(|parent| parent.advance())
            .collect::<Vec<EarleyState>>();
        for new_state in completions {
            self.insert(col, new_state);
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

    fn unique_postdot(&mut self, state: &EarleyState) -> Option<EarleyState> {
        let parents = self.state_table[state.start]
            .states
            .iter()
            .filter(|s| s.at_dot().is_some() && s.at_dot().unwrap().key == state.lhs)
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

    fn chart_parse(&mut self, context: &ParsingContext) {
        let n_columns = self.state_table.len();
        for col in 0..n_columns {
            self.chart_parse_column_step(context, col);
        }
    }

    fn chart_parse_column_step(&mut self, context: &ParsingContext, col: usize) {
        let mut state_index = 0;
        let mut n_states = self.state_table[col].len();
        while state_index < n_states {
            self.chart_parse_state_step(context, &mut state_index, &mut n_states, col);
        }
    }

    fn chart_parse_state_step(
        &mut self,
        context: &ParsingContext,
        state_index: &mut usize,
        n_states: &mut usize,
        col: usize,
    ) {
        let state = &self.state_table[col].states[*state_index];
        let symbol = state.at_dot();
        match symbol {
            None => {
                self.leo_complete(col, *state_index);
            }
            Some(term) => match term.kind {
                TermKind::Nonterminal => {
                    let term = term.clone();
                    self.predict(context, col, *state_index, term.key);
                }
                TermKind::Terminal => {
                    let next_col = col + 1;
                    if next_col < self.state_table.len() {
                        let symbol = term.content.chars().next();
                        self.scan(next_col, *state_index, symbol);
                    }
                }
            },
        }
        *state_index += 1;
        *n_states = self.state_table[col].len();
    }
}

pub struct ExtendedEarleyParser {
    context: ParsingContext,
    state: ParsingState,
}

impl ExtendedEarleyParser {
    fn init_input(&mut self, input: &str) {
        self.context.input = input.chars().collect();
        self.init_state_table();
        self.seed_state_table();
    }

    fn init_state_table(&mut self) {
        self.state.state_table = Vec::with_capacity(self.context.input.len() + 1);
        self.state.state_table.push(Column::new('_'));
        for symbol in &self.context.input {
            self.state.state_table.push(Column::new(*symbol));
        }
    }

    fn seed_state_table(&mut self) {
        let initial_rule = self
            .context
            .grammar
            .rule_lut
            .get(&self.context.grammar.start)
            .ok_or(Error::EmptyGrammarError(
                "cannot seed earley state table without initial rule".to_owned(),
            ))
            .unwrap();
        for alternative in &initial_rule.rhs.alternatives {
            self.state.state_table[0].add(EarleyState::new(
                initial_rule.lhs.key,
                alternative.clone(),
                0,
                0,
            ))
        }
    }

    pub fn dump_state(&self) -> String {
        let mut s = String::new();
        s.push_str("============================================\n");
        s.push_str("state table\n");
        s.push_str("============================================\n");
        for (start, column) in self.state.state_table.iter().enumerate() {
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

    fn chart_parse(&mut self) {
        self.state.chart_parse(&self.context);
    }

    pub fn recognize(&mut self, input: &str) -> bool {
        self.init_input(input);
        self.chart_parse();
        self.state
            .state_table
            .last()
            .unwrap()
            .states
            .iter()
            .any(|state| {
                state.start == 0
                    && state.at_dot().is_none()
                    && state.lhs == self.context.grammar.start
            })
    }
}

impl From<&Grammar> for ExtendedEarleyParser {
    fn from(grammar: &Grammar) -> Self {
        Self {
            context: grammar.into(),
            state: Default::default(),
        }
    }
}
