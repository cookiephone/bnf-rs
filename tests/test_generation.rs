use bnf::*;

#[test]
fn test_generation_generate() {
    let grammar = corpus::grammar_bnf();
    for _ in 0..100 {
        let word = grammar.generate().unwrap();
        assert!(grammar.recognize(word.as_str()));
    }
}

#[test]
fn test_generation_generate_parameterized() {
    let grammar = corpus::grammar_bnf();
    for seed in 0..100 {
        for strategy in &[
            GenerationStrategy::UniformRHSSampling,
            GenerationStrategy::RecursionAvoidance,
            GenerationStrategy::GreedyTerminals,
        ] {
            let word = grammar.generate_parameterized(*strategy, seed).unwrap();
            assert!(grammar.recognize(word.as_str()));
        }
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
