use crate::codify::Codify;
use crate::error::Error;
use crate::generator::GenerationStrategy;
use crate::generator::Generator;
use crate::parser::ExtendedEarleyParser;
use crate::rule::Rule;
use crate::term::Term;
use crate::types::NoHashMap;
use crate::types::TermKey;
use itertools::Itertools;
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Grammar {
    pub(crate) start: TermKey,
    pub(crate) rules: Vec<Rule>,
    pub(crate) rule_lut: NoHashMap<TermKey, Rule>,
    pub(crate) term_lut: NoHashMap<TermKey, Term>,
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

    pub fn parse(&self, input: &str) {
        // TODO
    }

    pub fn recognize(&self, input: &str) -> bool {
        ExtendedEarleyParser::recognize(self, input)
    }

    pub(crate) fn rule(&self, term_key: TermKey) -> &Rule {
        self.rule_lut.get(&term_key).unwrap()
    }

    pub(crate) fn term(&self, term_key: TermKey) -> &Term {
        self.term_lut.get(&term_key).unwrap()
    }

    pub(crate) fn atomize_terminals(&mut self) {
        for rule in self.rules.iter_mut() {
            for alternative in rule.rhs.alternatives.iter_mut() {
                let mut tmp = Vec::new();
                for term in alternative.iter() {
                    if term.is_nonterminal() || term.is_atomic_terminal() {
                        tmp.push(term.to_owned());
                    } else {
                        tmp.append(&mut term.atomize().unwrap());
                    }
                }
                *alternative = Rc::new(tmp);
            }
        }
        self.init();
    }

    fn build_rule_lut(&mut self) {
        self.rule_lut = self
            .rules
            .iter()
            .map(|rule| (rule.lhs.key, rule.clone()))
            .collect();
    }

    fn build_term_lut(&mut self) {
        self.term_lut =
            self.rules
                .iter()
                .flat_map(|rule| {
                    rule.rhs.alternatives.iter().flat_map(|alternative| {
                        alternative.iter().map(|term| (term.key, term.clone()))
                    })
                })
                .collect()
    }

    fn init(&mut self) {
        self.build_rule_lut();
        self.build_term_lut();
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
        let mut s = format!("{prefix}Grammar::builder().rules(&[\n");
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
        let mut grammar = Grammar {
            start: self
                .rules
                .first()
                .expect(
                    "cannot find starting symbol, a grammar requires at least one production rule",
                )
                .lhs
                .key,
            rule_lut: Default::default(),
            term_lut: Default::default(),
            rules: self.rules,
        };
        grammar.init();
        grammar
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
