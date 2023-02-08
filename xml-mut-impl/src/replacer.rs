use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Replacer {
    // TODO: String -> &'a str
    pub bounds: Range<usize>,
    pub replacement: String,
}
