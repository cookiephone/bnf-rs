extern crate proc_macro;
use itertools::Itertools;
use quote::quote;

#[proc_macro]
pub fn grammar(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut grammar_builder = bnf_core::Grammar::builder();
    let mut tokens = input.into_iter().multipeek();
    // consume tokens, each iteration corresponds to one production rule
    loop {
        // if no more tokens, the previous rule was the last, therefore exit the loop
        if tokens.peek().is_none() {
            break;
        }
        // check if the 2nd token (1st was peeked before) is the expected symbol for production rules
        match tokens.peek() {
            Some(proc_macro::TokenTree::Punct(punct)) if punct.as_char() == '=' => (),
            Some(tokentree) => {
                let s = tokentree.to_string();
                return quote! { std::result::Result::<bnf_core::Grammar, bnf_core::Error>::Err(bnf_core::Error::InvalidGrammarSyntaxError(std::format!("unexpected token encountered: {}", #s).to_owned())) }.into()
            }
            None => return quote! { std::result::Result::<bnf_core::Grammar, bnf_core::Error>::Err(bnf_core::Error::InvalidGrammarSyntaxError("expected token but got none".to_owned())) }.into(),
        }
        // consume the first token, which should correspond to the lhs of the production rule
        let lhs = match tokens.next() {
            Some(proc_macro::TokenTree::Ident(ident)) => bnf_core::Term::Nonterminal(ident.to_string()),
            Some(tokentree) => {
                let s = tokentree.to_string();
                return quote! { std::result::Result::<bnf_core::Grammar, bnf_core::Error>::Err(bnf_core::Error::InvalidGrammarSyntaxError(std::format!("unexpected token encountered: {}", #s).to_owned())) }.into()
            }
            None => return quote! { std::result::Result::<bnf_core::Grammar, bnf_core::Error>::Err(bnf_core::Error::InvalidGrammarSyntaxError("expected token but got none".to_owned())) }.into(),
        };
        // skip the token representing the symbol for the production rule
        tokens.next();
        // consume rhs of the production rule
        let mut rhs = bnf_core::Alternatives::new();
        let mut alternative = Vec::new();
        let mut valid_rule = false;
        let mut expect_term = false;
        loop {
            // skip one token
            tokens.peek();
            // check if the 2nd token (1st was peeked before) is the expected symbol for production rules
            match tokens.peek() {
                Some(proc_macro::TokenTree::Punct(punct)) if valid_rule && punct.as_char() == '=' => {
                    rhs.add_alternative(alternative);
                    break;
                },
                Some(proc_macro::TokenTree::Punct(punct)) if punct.as_char() == '=' => { return quote! { std::result::Result::<bnf_core::Grammar, bnf_core::Error>::Err(bnf_core::Error::InvalidGrammarSyntaxError("rule right-hand-side requires at least one term".to_owned())) }.into() },
                _ => ()
            }
            // check if we need a new alternative, or add a term to the current alternative
            match tokens.next() {
                Some(proc_macro::TokenTree::Punct(punct)) if punct.as_char() == '|' && expect_term => { return quote! { std::result::Result::<bnf_core::Grammar, bnf_core::Error>::Err(bnf_core::Error::InvalidGrammarSyntaxError("expected a term after the '|' symbol in the right-hand-side of the rule".to_owned())) }.into() },
                Some(proc_macro::TokenTree::Punct(punct)) if punct.as_char() == '|' => {
                    rhs.add_alternative(alternative);
                    alternative = Vec::new();
                    expect_term = true;
                },
                Some(proc_macro::TokenTree::Ident(ident)) => {
                    alternative.push(bnf_core::Term::Nonterminal(ident.to_string()));
                    expect_term = false;
                    valid_rule = true;
                },
                Some(proc_macro::TokenTree::Literal(literal)) => {
                    let mut literal = literal.to_string();
                    literal.pop();
                    literal.remove(0);
                    alternative.push(bnf_core::Term::Terminal(literal));
                    expect_term = false;
                    valid_rule = true;
                },
                Some(tokentree) => {
                    let s = tokentree.to_string();
                    return quote! { std::result::Result::<bnf_core::Grammar, bnf_core::Error>::Err(bnf_core::Error::InvalidGrammarSyntaxError(std::format!("unexpected token encountered: {}", #s).to_owned())) }.into()
                }
                None if valid_rule && !expect_term => {
                    rhs.add_alternative(alternative);
                    break;
                },
                None => return quote! { std::result::Result::<bnf_core::Grammar, bnf_core::Error>::Err(bnf_core::Error::InvalidGrammarSyntaxError("expected token but got none".to_owned())) }.into(),
            }
        }
        grammar_builder = grammar_builder.rule(bnf_core::Rule { lhs, rhs });
        tokens.reset_peek();
    }
    let grammar = grammar_builder.build();
    let code = format!(
        "std::result::Result::<bnf_core::Grammar, bnf_core::Error>::Ok({})",
        bnf_core::Codify::codify(&grammar, "bnf::")
    );
    code.parse().unwrap()
}
