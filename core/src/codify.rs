pub trait Codify {
    fn codify(&self, prefix: &str) -> String;
}
