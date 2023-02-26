use crate as bnf;
use bnf_core::Grammar;
use bnf_macros::grammar;

pub fn grammar_bnf() -> Grammar {
    grammar! {
        syntax          = rule | rule syntax
        rule            = opt_whitespace "<" rule_name ">" opt_whitespace "::=" opt_whitespace expression line_end
        opt_whitespace  = " " opt_whitespace | ""
        expression      = list | list opt_whitespace "|" opt_whitespace expression
        line_end        = opt_whitespace eol | line_end line_end
        list            = term | term opt_whitespace list
        term            = literal | "<" rule_name ">"
        literal         = "\"" text1 "\"" | "'" text2 "'"
        text1           = "" | character1 text1
        text2           = "" | character2 text2
        character       = letter | digit | symbol
        letter          = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O"
                        | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" | "a" | "b" | "c" | "d"
                        | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s"
                        | "t" | "u" | "v" | "w" | "x" | "y" | "z"
        digit           = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
        symbol          = "|" | " " | "!" | "#" | "$" | "%" | "&" | "(" | ")" | "*" | "+" | "," | "-" | "." | "/"
                        | ":" | ";" | ">" | "=" | "<" | "?" | "@" | "[" | "\\" | "]" | "^" | "_" | "`" | "{" | "}"
                        | "~"
        character1      = character | "'"
        character2      = character | "\""
        rule_name       = letter | rule_name rule_char
        rule_char       = letter | digit | "-"
        eol             = "\r\n" | "\n"
    }
    .unwrap()
}

pub fn grammar_int() -> Grammar {
    grammar! {
        integer                   = "0" | unsigned_nonzero_integer | sign unsigned_nonzero_integer
        unsigned_nonzero_integer  = nonzero_digit digits
        digits                    = digits digit | ""
        digit                     = "0" | nonzero_digit
        nonzero_digit             = "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
        sign                      = "+" | "-"
    }
    .unwrap()
}
