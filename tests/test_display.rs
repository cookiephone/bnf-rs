use bnf::*;

#[test]
fn test_display_term() {
    assert_eq!(Term::Terminal("foo".to_owned()).to_string(), "\"foo\"");
    assert_eq!(
        Term::Terminal("foo\n\t\r\\".to_owned()).to_string(),
        "\"foo\\n\\t\\r\\\\\""
    );
}

#[test]
fn test_display_alternatives() {
    assert_eq!(
        Alternatives::from(vec![vec![
            Term::Nonterminal("opt_whitespace".to_string()),
            Term::Terminal("<".to_string()),
            Term::Nonterminal("rule_name".to_string()),
            Term::Terminal(">".to_string()),
            Term::Nonterminal("opt_whitespace".to_string()),
            Term::Terminal("::=".to_string()),
            Term::Nonterminal("opt_whitespace".to_string()),
            Term::Nonterminal("expression".to_string()),
            Term::Nonterminal("line_end".to_string()),
        ]]).to_string(),
        "<opt_whitespace> \"<\" <rule_name> \">\" <opt_whitespace> \"::=\" <opt_whitespace> <expression> <line_end>"
    );
}

#[test]
fn test_display_rule() {
    assert_eq!(
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
        }.to_string(),
        "<rule> ::= <opt_whitespace> \"<\" <rule_name> \">\" <opt_whitespace> \"::=\" <opt_whitespace> <expression> <line_end>"
    );
}

#[test]
fn test_display_grammar() {
    assert_eq!(
        playground::grammar_bnf().to_string(),
        "<syntax> ::= <rule> | <rule> <syntax>\n\
        <character2> ::= <character> | \"\\\"\"\n\
        <character> ::= <letter> | <digit> | <symbol>\n\
        <digit> ::= \"0\" | \"1\" | \"2\" | \"3\" | \"4\" | \"5\" | \"6\" | \"7\" | \"8\" | \"9\"\n\
        <eol> ::= \"\\r\\n\" | \"\\n\"\n\
        <expression> ::= <list> | <list> <opt_whitespace> \"|\" <opt_whitespace> <expression>\n\
        <letter> ::= \"A\" | \"B\" | \"C\" | \"D\" | \"E\" | \"F\" | \"G\" | \"H\" | \"I\" | \"J\" \
            | \"K\" | \"L\" | \"M\" | \"N\" | \"O\" | \"P\" | \"Q\" | \"R\" | \"S\" | \"T\" \
            | \"U\" | \"V\" | \"W\" | \"X\" | \"Y\" | \"Z\" | \"a\" | \"b\" | \"c\" | \"d\" | \"e\" \
            | \"f\" | \"g\" | \"h\" | \"i\" | \"j\" | \"k\" | \"l\" | \"m\" | \"n\" | \"o\" | \"p\" \
            | \"q\" | \"r\" | \"s\" | \"t\" | \"u\" | \"v\" | \"w\" | \"x\" | \"y\" | \"z\"\n\
        <line_end> ::= <opt_whitespace> <eol> | <opt_whitespace> <eol> <line_end>\n\
        <list> ::= <term> | <term> <opt_whitespace> <list>\n\
        <literal> ::= \"\\\"\" <text1> \"\\\"\" | \"\\\'\" <text2> \"\\\'\"\n\
        <opt_whitespace> ::= \" \" <opt_whitespace> | \"\"\n\
        <rule> ::= <opt_whitespace> \"<\" <rule_name> \">\" \
            <opt_whitespace> \"::=\" <opt_whitespace> <expression> <line_end>\n\
        <rule_char> ::= <letter> | <digit> | \"-\"\n\
        <rule_name> ::= <letter> | <rule_name> <rule_char>\n\
        <symbol> ::= \"|\" | \" \" | \"!\" | \"#\" | \"$\" | \"%\" | \"&\" | \"(\" | \")\" | \"*\" \
            | \"+\" | \",\" | \"-\" | \".\" | \"/\" | \":\" | \";\" | \">\" | \"=\" | \"<\" | \"?\" \
            | \"@\" | \"[\" | \"\\\\\" | \"]\" | \"^\" | \"_\" | \"`\" | \"{\" | \"}\" | \"~\"\n\
            <character1> ::= <character> | \"\\\'\"\n\
        <term> ::= <literal> | \"<\" <rule_name> \">\"\n\
        <text1> ::= \"\" | <character1> <text1>\n\
        <text2> ::= \"\" | <character2> <text2>"
    );
}
