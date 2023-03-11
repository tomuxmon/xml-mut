use crate::replacer::Replacer;

#[derive(Debug)]
pub enum DeleteError {
    DeleteNothing(String),
    DeleteNoBounds(String),
}

#[derive(Debug)]
pub enum AssignError {
    AssignmentTargetNotFound(String),
    AssignmentSourceValueNotFound(String),
    AssignmentTargetBoundsNotFound(String),
}

#[derive(Debug)]
pub enum ReplaceError {
    ReplacerOverlap(Replacer, Replacer),
    GeneratedXmlInvalid(String),
}
