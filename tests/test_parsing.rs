use bnf::*;

#[test]
fn test_parsing_recognize() {
    let grammar = playground::grammar_bnf();
    for seed in 0..1000 {
        let word = grammar
            .generate_parameterized(GenerationStrategy::UniformRHSSampling, seed)
            .unwrap();
        assert!(grammar.recognize(word.as_str()));
    }
}
