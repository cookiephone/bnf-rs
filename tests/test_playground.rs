use bnf::*;

#[test]
fn test_playground_bnf() {
    let grammar = playground::grammar_bnf();
    let correct = Grammar::builder()
        .rules(&vec![
            Rule {
                lhs: Term::Nonterminal("syntax".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Nonterminal("rule".to_string())],
                    vec![
                        Term::Nonterminal("rule".to_string()),
                        Term::Nonterminal("syntax".to_string()),
                    ],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("rule".to_string()),
                rhs: Alternatives::from(vec![vec![
                    Term::Nonterminal("opt_whitespace".to_string()),
                    Term::Terminal("<".to_string()),
                    Term::Nonterminal("rule_name".to_string()),
                    Term::Terminal(">".to_string()),
                    Term::Nonterminal("opt_whitespace".to_string()),
                    Term::Terminal("::=".to_string()),
                    Term::Nonterminal("opt_whitespace".to_string()),
                    Term::Nonterminal("expression".to_string()),
                    Term::Nonterminal("line_end".to_string()),
                ]]),
            },
            Rule {
                lhs: Term::Nonterminal("opt_whitespace".to_string()),
                rhs: Alternatives::from(vec![
                    vec![
                        Term::Terminal(" ".to_string()),
                        Term::Nonterminal("opt_whitespace".to_string()),
                    ],
                    vec![Term::Terminal("".to_string())],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("expression".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Nonterminal("list".to_string())],
                    vec![
                        Term::Nonterminal("list".to_string()),
                        Term::Nonterminal("opt_whitespace".to_string()),
                        Term::Terminal("|".to_string()),
                        Term::Nonterminal("opt_whitespace".to_string()),
                        Term::Nonterminal("expression".to_string()),
                    ],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("line_end".to_string()),
                rhs: Alternatives::from(vec![
                    vec![
                        Term::Nonterminal("opt_whitespace".to_string()),
                        Term::Nonterminal("eol".to_string()),
                    ],
                    vec![
                        Term::Nonterminal("line_end".to_string()),
                        Term::Nonterminal("line_end".to_string()),
                    ],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("list".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Nonterminal("term".to_string())],
                    vec![
                        Term::Nonterminal("term".to_string()),
                        Term::Nonterminal("opt_whitespace".to_string()),
                        Term::Nonterminal("list".to_string()),
                    ],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("term".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Nonterminal("literal".to_string())],
                    vec![
                        Term::Terminal("<".to_string()),
                        Term::Nonterminal("rule_name".to_string()),
                        Term::Terminal(">".to_string()),
                    ],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("literal".to_string()),
                rhs: Alternatives::from(vec![
                    vec![
                        Term::Terminal("\"".to_string()),
                        Term::Nonterminal("text1".to_string()),
                        Term::Terminal("\"".to_string()),
                    ],
                    vec![
                        Term::Terminal("'".to_string()),
                        Term::Nonterminal("text2".to_string()),
                        Term::Terminal("'".to_string()),
                    ],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("text1".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Terminal("".to_string())],
                    vec![
                        Term::Nonterminal("character1".to_string()),
                        Term::Nonterminal("text1".to_string()),
                    ],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("text2".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Terminal("".to_string())],
                    vec![
                        Term::Nonterminal("character2".to_string()),
                        Term::Nonterminal("text2".to_string()),
                    ],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("character".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Nonterminal("letter".to_string())],
                    vec![Term::Nonterminal("digit".to_string())],
                    vec![Term::Nonterminal("symbol".to_string())],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("letter".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Terminal("A".to_string())],
                    vec![Term::Terminal("B".to_string())],
                    vec![Term::Terminal("C".to_string())],
                    vec![Term::Terminal("D".to_string())],
                    vec![Term::Terminal("E".to_string())],
                    vec![Term::Terminal("F".to_string())],
                    vec![Term::Terminal("G".to_string())],
                    vec![Term::Terminal("H".to_string())],
                    vec![Term::Terminal("I".to_string())],
                    vec![Term::Terminal("J".to_string())],
                    vec![Term::Terminal("K".to_string())],
                    vec![Term::Terminal("L".to_string())],
                    vec![Term::Terminal("M".to_string())],
                    vec![Term::Terminal("N".to_string())],
                    vec![Term::Terminal("O".to_string())],
                    vec![Term::Terminal("P".to_string())],
                    vec![Term::Terminal("Q".to_string())],
                    vec![Term::Terminal("R".to_string())],
                    vec![Term::Terminal("S".to_string())],
                    vec![Term::Terminal("T".to_string())],
                    vec![Term::Terminal("U".to_string())],
                    vec![Term::Terminal("V".to_string())],
                    vec![Term::Terminal("W".to_string())],
                    vec![Term::Terminal("X".to_string())],
                    vec![Term::Terminal("Y".to_string())],
                    vec![Term::Terminal("Z".to_string())],
                    vec![Term::Terminal("a".to_string())],
                    vec![Term::Terminal("b".to_string())],
                    vec![Term::Terminal("c".to_string())],
                    vec![Term::Terminal("d".to_string())],
                    vec![Term::Terminal("e".to_string())],
                    vec![Term::Terminal("f".to_string())],
                    vec![Term::Terminal("g".to_string())],
                    vec![Term::Terminal("h".to_string())],
                    vec![Term::Terminal("i".to_string())],
                    vec![Term::Terminal("j".to_string())],
                    vec![Term::Terminal("k".to_string())],
                    vec![Term::Terminal("l".to_string())],
                    vec![Term::Terminal("m".to_string())],
                    vec![Term::Terminal("n".to_string())],
                    vec![Term::Terminal("o".to_string())],
                    vec![Term::Terminal("p".to_string())],
                    vec![Term::Terminal("q".to_string())],
                    vec![Term::Terminal("r".to_string())],
                    vec![Term::Terminal("s".to_string())],
                    vec![Term::Terminal("t".to_string())],
                    vec![Term::Terminal("u".to_string())],
                    vec![Term::Terminal("v".to_string())],
                    vec![Term::Terminal("w".to_string())],
                    vec![Term::Terminal("x".to_string())],
                    vec![Term::Terminal("y".to_string())],
                    vec![Term::Terminal("z".to_string())],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("digit".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Terminal("0".to_string())],
                    vec![Term::Terminal("1".to_string())],
                    vec![Term::Terminal("2".to_string())],
                    vec![Term::Terminal("3".to_string())],
                    vec![Term::Terminal("4".to_string())],
                    vec![Term::Terminal("5".to_string())],
                    vec![Term::Terminal("6".to_string())],
                    vec![Term::Terminal("7".to_string())],
                    vec![Term::Terminal("8".to_string())],
                    vec![Term::Terminal("9".to_string())],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("symbol".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Terminal("|".to_string())],
                    vec![Term::Terminal(" ".to_string())],
                    vec![Term::Terminal("!".to_string())],
                    vec![Term::Terminal("#".to_string())],
                    vec![Term::Terminal("$".to_string())],
                    vec![Term::Terminal("%".to_string())],
                    vec![Term::Terminal("&".to_string())],
                    vec![Term::Terminal("(".to_string())],
                    vec![Term::Terminal(")".to_string())],
                    vec![Term::Terminal("*".to_string())],
                    vec![Term::Terminal("+".to_string())],
                    vec![Term::Terminal(",".to_string())],
                    vec![Term::Terminal("-".to_string())],
                    vec![Term::Terminal(".".to_string())],
                    vec![Term::Terminal("/".to_string())],
                    vec![Term::Terminal(":".to_string())],
                    vec![Term::Terminal(";".to_string())],
                    vec![Term::Terminal(">".to_string())],
                    vec![Term::Terminal("=".to_string())],
                    vec![Term::Terminal("<".to_string())],
                    vec![Term::Terminal("?".to_string())],
                    vec![Term::Terminal("@".to_string())],
                    vec![Term::Terminal("[".to_string())],
                    vec![Term::Terminal("\\".to_string())],
                    vec![Term::Terminal("]".to_string())],
                    vec![Term::Terminal("^".to_string())],
                    vec![Term::Terminal("_".to_string())],
                    vec![Term::Terminal("`".to_string())],
                    vec![Term::Terminal("{".to_string())],
                    vec![Term::Terminal("}".to_string())],
                    vec![Term::Terminal("~".to_string())],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("character1".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Nonterminal("character".to_string())],
                    vec![Term::Terminal("'".to_string())],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("character2".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Nonterminal("character".to_string())],
                    vec![Term::Terminal("\"".to_string())],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("rule_name".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Nonterminal("letter".to_string())],
                    vec![
                        Term::Nonterminal("rule_name".to_string()),
                        Term::Nonterminal("rule_char".to_string()),
                    ],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("rule_char".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Nonterminal("letter".to_string())],
                    vec![Term::Nonterminal("digit".to_string())],
                    vec![Term::Terminal("-".to_string())],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("eol".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Terminal("\r\n".to_string())],
                    vec![Term::Terminal("\n".to_string())],
                ]),
            },
        ])
        .build();
    assert_eq!(grammar, correct);
}

#[test]
fn test_playground_int() {
    let grammar = playground::grammar_int();
    let correct = Grammar::builder()
        .rules(&vec![
            Rule {
                lhs: Term::Nonterminal("integer".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Terminal("0".to_string())],
                    vec![Term::Nonterminal("unsigned_nonzero_integer".to_string())],
                    vec![
                        Term::Nonterminal("sign".to_string()),
                        Term::Nonterminal("unsigned_nonzero_integer".to_string()),
                    ],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("unsigned_nonzero_integer".to_string()),
                rhs: Alternatives::from(vec![vec![
                    Term::Nonterminal("nonzero_digit".to_string()),
                    Term::Nonterminal("digits".to_string()),
                ]]),
            },
            Rule {
                lhs: Term::Nonterminal("digits".to_string()),
                rhs: Alternatives::from(vec![
                    vec![
                        Term::Nonterminal("digits".to_string()),
                        Term::Nonterminal("digit".to_string()),
                    ],
                    vec![Term::Terminal("".to_string())],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("digit".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Terminal("0".to_string())],
                    vec![Term::Nonterminal("nonzero_digit".to_string())],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("nonzero_digit".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Terminal("1".to_string())],
                    vec![Term::Terminal("2".to_string())],
                    vec![Term::Terminal("3".to_string())],
                    vec![Term::Terminal("4".to_string())],
                    vec![Term::Terminal("5".to_string())],
                    vec![Term::Terminal("6".to_string())],
                    vec![Term::Terminal("7".to_string())],
                    vec![Term::Terminal("8".to_string())],
                    vec![Term::Terminal("9".to_string())],
                ]),
            },
            Rule {
                lhs: Term::Nonterminal("sign".to_string()),
                rhs: Alternatives::from(vec![
                    vec![Term::Terminal("+".to_string())],
                    vec![Term::Terminal("-".to_string())],
                ]),
            },
        ])
        .build();
    assert_eq!(grammar, correct);
}
