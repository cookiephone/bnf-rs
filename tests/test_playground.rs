use bnf::*;

#[test]
fn test_playground_bnf() {
    let grammar = corpus::grammar_bnf();
    let correct = Grammar::builder()
        .rules(&vec![
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("syntax")),
                rhs: Alternatives::from(vec![
                    vec![Term::nonterminal("rule")],
                    vec![Term::nonterminal("rule"), Term::nonterminal("syntax")],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("rule")),
                rhs: Alternatives::from(vec![vec![
                    Term::nonterminal("opt_whitespace"),
                    Term::terminal("<"),
                    Term::nonterminal("rule_name"),
                    Term::terminal(">"),
                    Term::nonterminal("opt_whitespace"),
                    Term::terminal("::="),
                    Term::nonterminal("opt_whitespace"),
                    Term::nonterminal("expression"),
                    Term::nonterminal("line_end"),
                ]]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("opt_whitespace")),
                rhs: Alternatives::from(vec![
                    vec![Term::terminal(" "), Term::nonterminal("opt_whitespace")],
                    vec![Term::terminal("")],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("expression")),
                rhs: Alternatives::from(vec![
                    vec![Term::nonterminal("list")],
                    vec![
                        Term::nonterminal("list"),
                        Term::nonterminal("opt_whitespace"),
                        Term::terminal("|"),
                        Term::nonterminal("opt_whitespace"),
                        Term::nonterminal("expression"),
                    ],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("line_end")),
                rhs: Alternatives::from(vec![
                    vec![
                        Term::nonterminal("opt_whitespace"),
                        Term::nonterminal("eol"),
                    ],
                    vec![
                        Term::nonterminal("opt_whitespace"),
                        Term::nonterminal("eol"),
                        Term::nonterminal("line_end"),
                    ],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("list")),
                rhs: Alternatives::from(vec![
                    vec![Term::nonterminal("term")],
                    vec![
                        Term::nonterminal("term"),
                        Term::nonterminal("opt_whitespace"),
                        Term::nonterminal("list"),
                    ],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("term")),
                rhs: Alternatives::from(vec![
                    vec![Term::nonterminal("literal")],
                    vec![
                        Term::terminal("<"),
                        Term::nonterminal("rule_name"),
                        Term::terminal(">"),
                    ],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("literal")),
                rhs: Alternatives::from(vec![
                    vec![
                        Term::terminal("\""),
                        Term::nonterminal("text1"),
                        Term::terminal("\""),
                    ],
                    vec![
                        Term::terminal("'"),
                        Term::nonterminal("text2"),
                        Term::terminal("'"),
                    ],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("text1")),
                rhs: Alternatives::from(vec![
                    vec![Term::terminal("")],
                    vec![Term::nonterminal("character1"), Term::nonterminal("text1")],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("text2")),
                rhs: Alternatives::from(vec![
                    vec![Term::terminal("")],
                    vec![Term::nonterminal("character2"), Term::nonterminal("text2")],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("character")),
                rhs: Alternatives::from(vec![
                    vec![Term::nonterminal("letter")],
                    vec![Term::nonterminal("digit")],
                    vec![Term::nonterminal("symbol")],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("letter")),
                rhs: Alternatives::from(vec![
                    vec![Term::terminal("A")],
                    vec![Term::terminal("B")],
                    vec![Term::terminal("C")],
                    vec![Term::terminal("D")],
                    vec![Term::terminal("E")],
                    vec![Term::terminal("F")],
                    vec![Term::terminal("G")],
                    vec![Term::terminal("H")],
                    vec![Term::terminal("I")],
                    vec![Term::terminal("J")],
                    vec![Term::terminal("K")],
                    vec![Term::terminal("L")],
                    vec![Term::terminal("M")],
                    vec![Term::terminal("N")],
                    vec![Term::terminal("O")],
                    vec![Term::terminal("P")],
                    vec![Term::terminal("Q")],
                    vec![Term::terminal("R")],
                    vec![Term::terminal("S")],
                    vec![Term::terminal("T")],
                    vec![Term::terminal("U")],
                    vec![Term::terminal("V")],
                    vec![Term::terminal("W")],
                    vec![Term::terminal("X")],
                    vec![Term::terminal("Y")],
                    vec![Term::terminal("Z")],
                    vec![Term::terminal("a")],
                    vec![Term::terminal("b")],
                    vec![Term::terminal("c")],
                    vec![Term::terminal("d")],
                    vec![Term::terminal("e")],
                    vec![Term::terminal("f")],
                    vec![Term::terminal("g")],
                    vec![Term::terminal("h")],
                    vec![Term::terminal("i")],
                    vec![Term::terminal("j")],
                    vec![Term::terminal("k")],
                    vec![Term::terminal("l")],
                    vec![Term::terminal("m")],
                    vec![Term::terminal("n")],
                    vec![Term::terminal("o")],
                    vec![Term::terminal("p")],
                    vec![Term::terminal("q")],
                    vec![Term::terminal("r")],
                    vec![Term::terminal("s")],
                    vec![Term::terminal("t")],
                    vec![Term::terminal("u")],
                    vec![Term::terminal("v")],
                    vec![Term::terminal("w")],
                    vec![Term::terminal("x")],
                    vec![Term::terminal("y")],
                    vec![Term::terminal("z")],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("digit")),
                rhs: Alternatives::from(vec![
                    vec![Term::terminal("0")],
                    vec![Term::terminal("1")],
                    vec![Term::terminal("2")],
                    vec![Term::terminal("3")],
                    vec![Term::terminal("4")],
                    vec![Term::terminal("5")],
                    vec![Term::terminal("6")],
                    vec![Term::terminal("7")],
                    vec![Term::terminal("8")],
                    vec![Term::terminal("9")],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("symbol")),
                rhs: Alternatives::from(vec![
                    vec![Term::terminal("|")],
                    vec![Term::terminal(" ")],
                    vec![Term::terminal("!")],
                    vec![Term::terminal("#")],
                    vec![Term::terminal("$")],
                    vec![Term::terminal("%")],
                    vec![Term::terminal("&")],
                    vec![Term::terminal("(")],
                    vec![Term::terminal(")")],
                    vec![Term::terminal("*")],
                    vec![Term::terminal("+")],
                    vec![Term::terminal(",")],
                    vec![Term::terminal("-")],
                    vec![Term::terminal(".")],
                    vec![Term::terminal("/")],
                    vec![Term::terminal(":")],
                    vec![Term::terminal(";")],
                    vec![Term::terminal(">")],
                    vec![Term::terminal("=")],
                    vec![Term::terminal("<")],
                    vec![Term::terminal("?")],
                    vec![Term::terminal("@")],
                    vec![Term::terminal("[")],
                    vec![Term::terminal("\\")],
                    vec![Term::terminal("]")],
                    vec![Term::terminal("^")],
                    vec![Term::terminal("_")],
                    vec![Term::terminal("`")],
                    vec![Term::terminal("{")],
                    vec![Term::terminal("}")],
                    vec![Term::terminal("~")],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("character1")),
                rhs: Alternatives::from(vec![
                    vec![Term::nonterminal("character")],
                    vec![Term::terminal("'")],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("character2")),
                rhs: Alternatives::from(vec![
                    vec![Term::nonterminal("character")],
                    vec![Term::terminal("\"")],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("rule_name")),
                rhs: Alternatives::from(vec![
                    vec![Term::nonterminal("letter")],
                    vec![
                        Term::nonterminal("rule_name"),
                        Term::nonterminal("rule_char"),
                    ],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("rule_char")),
                rhs: Alternatives::from(vec![
                    vec![Term::nonterminal("letter")],
                    vec![Term::nonterminal("digit")],
                    vec![Term::terminal("-")],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("eol")),
                rhs: Alternatives::from(vec![
                    vec![Term::terminal("\r\n")],
                    vec![Term::terminal("\n")],
                ]),
            },
        ])
        .build();
    assert_eq!(grammar, correct);
}

#[test]
fn test_playground_int() {
    let grammar = corpus::grammar_int();
    let correct = Grammar::builder()
        .rules(&[
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("integer")),
                rhs: Alternatives::from(vec![
                    vec![Term::terminal("0")],
                    vec![Term::nonterminal("unsigned_nonzero_integer")],
                    vec![
                        Term::nonterminal("sign"),
                        Term::nonterminal("unsigned_nonzero_integer"),
                    ],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("unsigned_nonzero_integer")),
                rhs: Alternatives::from(vec![vec![
                    Term::nonterminal("nonzero_digit"),
                    Term::nonterminal("digits"),
                ]]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("digits")),
                rhs: Alternatives::from(vec![
                    vec![Term::nonterminal("digits"), Term::nonterminal("digit")],
                    vec![Term::terminal("")],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("digit")),
                rhs: Alternatives::from(vec![
                    vec![Term::terminal("0")],
                    vec![Term::nonterminal("nonzero_digit")],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("nonzero_digit")),
                rhs: Alternatives::from(vec![
                    vec![Term::terminal("1")],
                    vec![Term::terminal("2")],
                    vec![Term::terminal("3")],
                    vec![Term::terminal("4")],
                    vec![Term::terminal("5")],
                    vec![Term::terminal("6")],
                    vec![Term::terminal("7")],
                    vec![Term::terminal("8")],
                    vec![Term::terminal("9")],
                ]),
            },
            Rule {
                lhs: std::rc::Rc::new(Term::nonterminal("sign")),
                rhs: Alternatives::from(vec![vec![Term::terminal("+")], vec![Term::terminal("-")]]),
            },
        ])
        .build();
    assert_eq!(grammar, correct);
}
