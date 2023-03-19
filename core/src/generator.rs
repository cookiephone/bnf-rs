use crate::error::Error;
use crate::grammar::Grammar;
use crate::term::Term;
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;

pub(crate) struct Generator<'a> {
    pub(crate) grammar: &'a Grammar,
    pub(crate) stack: Vec<Term>,
    pub(crate) sample: String,
    pub(crate) rng: StdRng,
}

impl Generator<'_> {
    fn init(&mut self) {
        self.stack.clear();
        self.stack.push(self.grammar.start.clone());
    }

    fn process_terminals(&mut self) {
        while self
            .stack
            .last()
            .map(|term| term.terminal().is_ok())
            .unwrap_or(false)
        {
            self.sample
                .extend(self.stack.pop().unwrap().terminal().unwrap().chars().rev());
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
        let rule = context
            .grammar
            .rule_for(&nonterminal)
            .ok_or(Error::UnknownNonterminalError(format!(
                "no production rule for nonterminal {} found",
                nonterminal
            )))?;
        match self {
            GenerationStrategy::UniformRHSSampling => {
                let terms = rule.rhs.alternatives.choose(&mut context.rng).unwrap();
                context.stack.extend_from_slice(terms)
            }
            GenerationStrategy::RecursionAvoidance => {
                let terms = match rule
                    .rhs
                    .alternatives
                    .iter()
                    .filter(|terms| !terms.contains(&rule.lhs))
                    .choose(&mut context.rng)
                {
                    Some(terms) => terms,
                    None => {
                        return Err(Error::InfinitelyRecursiveProductionError(
                            "cannot generate from infinitely recursive production rule".to_owned(),
                        ))
                    }
                };
                context.stack.extend_from_slice(terms)
            }
            GenerationStrategy::GreedyTerminals => {
                let terms = match rule
                    .rhs
                    .alternatives
                    .iter()
                    .filter(|terms| terms.iter().all(|term| term.terminal().is_ok()))
                    .choose(&mut context.rng)
                {
                    Some(terms) => terms,
                    None => rule.rhs.alternatives.choose(&mut context.rng).unwrap(),
                };
                context.stack.extend_from_slice(terms)
            }
        }
        Ok(())
    }
}
