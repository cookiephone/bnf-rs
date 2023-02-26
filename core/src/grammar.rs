use crate::codify::Codify;
use crate::error::Error;
use crate::generator::GenerationStrategy;
use crate::generator::Generator;
use crate::rule::Rule;
use crate::term::Term;
use itertools::Itertools;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Grammar {
    pub(crate) start: Term,
    rules: Vec<Rule>,
    pub(crate) rule_lut: HashMap<Term, Rule>,
}

impl Grammar {
    pub fn builder() -> GrammarBuilder {
        GrammarBuilder::default()
    }

    pub fn generate(&self) -> Result<String, Error> {
        self.generation_interface(None, None)
    }

    pub fn generate_parameterized(
        &self,
        strategy: GenerationStrategy,
        seed: u64,
    ) -> Result<String, Error> {
        self.generation_interface(Some(strategy), Some(seed))
    }

    fn generation_interface(
        &self,
        strategy: Option<GenerationStrategy>,
        seed: Option<u64>,
    ) -> Result<String, Error> {
        let mut generator = Generator {
            grammar: self,
            stack: Vec::new(),
            sample: String::new(),
            rng: match seed {
                Some(seed) => StdRng::seed_from_u64(seed),
                None => StdRng::from_entropy(),
            },
        };
        generator.generate(strategy.unwrap_or(GenerationStrategy::UniformRHSSampling))
    }
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.rules
                .iter()
                .map(|rule| rule.to_string())
                .intersperse("\n".into())
                .collect::<String>()
        )
    }
}

impl Codify for Grammar {
    fn codify(&self, prefix: &str) -> String {
        let mut s = format!("{prefix}Grammar::builder().rules(&vec![\n");
        for rule in self.rules.iter() {
            s.push_str(&format!("{},\n", rule.codify(prefix)));
        }
        s.push_str("]).build()");
        s
    }
}

#[derive(Default)]
pub struct GrammarBuilder {
    rules: Vec<Rule>,
}

impl GrammarBuilder {
    pub fn rule(mut self, rule: Rule) -> Self {
        self.rules.push(rule);
        self
    }

    pub fn rules(mut self, rules: &[Rule]) -> Self {
        self.rules.extend_from_slice(rules);
        self
    }

    pub fn build(mut self) -> Grammar {
        self.collapse();
        Grammar {
            start: self
                .rules
                .first()
                .expect(
                    "cannot find starting symbol, a grammar requires at least one production rule",
                )
                .lhs
                .clone(),
            rule_lut: build_rule_lut(&self.rules),
            rules: self.rules,
        }
    }

    fn collapse(&mut self) {
        let start = match self.rules.first() {
            Some(rule) => rule.clone(),
            None => return,
        };
        self.rules.sort_unstable_by_key(|rule| rule.lhs.to_string());
        self.rules = self.rules.iter().fold(
            Vec::<Rule>::with_capacity(self.rules.len()),
            |mut acc, curr| {
                match acc.last() {
                    Some(prev) if prev.lhs == curr.lhs => {
                        let merged = prev.merge(curr);
                        acc.pop();
                        acc.push(merged);
                    }
                    Some(_) => acc.push(curr.clone()),
                    None => acc.push(curr.clone()),
                }
                acc
            },
        );
        let idx = self
            .rules
            .binary_search_by_key(&start.lhs.to_string(), |rule| rule.lhs.to_string())
            .unwrap();
        self.rules.swap(0, idx);
    }
}

fn build_rule_lut(rules: &[Rule]) -> HashMap<Term, Rule> {
    rules
        .iter()
        .map(|rule| (rule.lhs.clone(), rule.clone()))
        .collect()
}
