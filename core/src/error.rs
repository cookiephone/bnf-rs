#[derive(Debug)]
pub enum Error {
    InvalidGrammarSyntaxError(String),
    UnknownNonterminalError(String),
    NotATerminalError(String),
    InfinitelyRecursiveProductionError(String),
    EmptyGrammarError(String),
}
