use bnf::*;

#[test]
fn test_parsing_recognize() {
    let grammar = corpus::grammar_bnf();
    for seed in 0..1000 {
        let word = grammar
            .generate_parameterized(GenerationStrategy::UniformRHSSampling, seed)
            .unwrap();
        assert!(grammar.recognize(word.as_str()));
    }
}

#[test]
fn test_parsing_sppf() {
    // yes this grammar is nonsense but it's an example from the paper (doi:10.1016/j.entcs.2008.03.044)
    let grammar = grammar! {
        S = A T | "a" T
        A = "a" | B A
        B = ""
        T = "b" "b" "b"
    }
    .unwrap();
    let word = "abbb";
    assert!(grammar.recognize(word));
}
