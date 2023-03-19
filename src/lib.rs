#![doc = include_str!("../README.md")]

pub mod playground;
pub use bnf_core::{
    Alternatives, Error, ExtendedEarleyParser, GenerationStrategy, Grammar, GrammarBuilder, Rule,
    Term,
};
pub use bnf_macros::*;
