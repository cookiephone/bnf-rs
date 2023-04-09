use crate::error::Error;
use crate::grammar::Grammar;
use crate::types::TermKey;
use itertools::Itertools;
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;

pub(crate) struct Generator<'a> {
    pub(crate) grammar: &'a Grammar,
    pub(crate) stack: Vec<TermKey>,
    pub(crate) sample: String,
    pub(crate) rng: StdRng,
}

impl Generator<'_> {
    fn init(&mut self) {
        self.stack.clear();
        self.stack.push(self.grammar.start);
    }

    fn process_terminals(&mut self) {
        while self
            .stack
            .last()
            .map(|term_key| self.grammar.term(term_key).terminal_content().is_ok())
            .unwrap_or(false)
        {
            let term_key = self.stack.pop().unwrap();
            self.sample.extend(
                self.grammar
                    .term(&term_key)
                    .terminal_content()
                    .unwrap()
                    .chars()
                    .rev(),
            );
        }
    }

    pub fn generate(&mut self, strategy: GenerationStrategy) -> Result<String, Error> {
        self.init();
        while !self.stack.is_empty() {
            strategy.step(self)?;
            self.process_terminals();
        }
        Ok(self.sample.chars().rev().collect())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum GenerationStrategy {
    UniformRHSSampling,
    RecursionAvoidance,
    GreedyTerminals,
}

impl GenerationStrategy {
    pub(crate) fn step(&self, context: &mut Generator) -> Result<(), Error> {
        let nonterminal = context.stack.pop().unwrap();
        let rule = context.grammar.rule(&nonterminal);
        match self {
            GenerationStrategy::UniformRHSSampling => {
                let terms = rule
                    .rhs
                    .alternatives
                    .choose(&mut context.rng)
                    .unwrap()
                    .iter()
                    .map(|term| term.key);
                context.stack.extend(terms);
            }
            GenerationStrategy::RecursionAvoidance => {
                let terms = match rule
                    .rhs
                    .alternatives
                    .iter()
                    .filter(|terms| !terms.iter().map(|term| term.key).contains(&rule.lhs.key))
                    .choose(&mut context.rng)
                {
                    Some(terms) => terms.iter().map(|term| term.key),
                    None => {
                        return Err(Error::InfinitelyRecursiveProductionError(
                            "cannot generate from infinitely recursive production rule".to_owned(),
                        ))
                    }
                };
                context.stack.extend(terms);
            }
            GenerationStrategy::GreedyTerminals => {
                let terms = match rule
                    .rhs
                    .alternatives
                    .iter()
                    .filter(|terms| terms.iter().all(|term| term.terminal_content().is_ok()))
                    .choose(&mut context.rng)
                {
                    Some(terms) => terms,
                    None => rule.rhs.alternatives.choose(&mut context.rng).unwrap(),
                };
                context.stack.extend(terms.iter().map(|term| term.key))
            }
        }
        Ok(())
    }
}
