use crate::replacer::Replacer;

pub enum ReplaceError {
    OverlappingReplacer(Replacer, Replacer),
    InvalidGeneratedXml(String),
}
