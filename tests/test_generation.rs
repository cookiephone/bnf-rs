use bnf::*;

#[test]
fn test_generation_generate() {
    let grammar = playground::grammar_bnf();
    for _ in 0..100 {
        let _word = grammar.generate().unwrap();
        // TODO: parse word as grammar to check validity
    }
}

#[test]
fn test_generation_generate_parameterized() {
    let grammar = playground::grammar_bnf();
    for seed in 0..100 {
        for strategy in &[
            GenerationStrategy::UniformRHSSampling,
            GenerationStrategy::RecursionAvoidance,
            GenerationStrategy::GreedyTerminals,
        ] {
            let _word1 = grammar.generate_parameterized(*strategy, seed).unwrap();
        }
        // TODO: parse words as grammar to check validity
    }
}

#[test]
#[allow(clippy::useless_vec)]
fn test_generation_generate_parameterized_infinitely_recursive_production() {
    let grammar = grammar! {
       s = a | "a"
       a = a | a
    }
    .unwrap();
    assert!(matches!(
        grammar.generate_parameterized(GenerationStrategy::RecursionAvoidance, 0),
        Err(Error::InfinitelyRecursiveProductionError { .. })
    ))
}
