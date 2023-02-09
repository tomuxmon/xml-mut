use crate::replacer::Replacer;

#[derive(Debug)]
pub enum ReplaceError {
    ReplacerOverlap(Replacer, Replacer),
    GeneratedXmlInvalid(String),
    DeleteNothing(String),
    DeletePathShouldStartWithAlias(String),
}
