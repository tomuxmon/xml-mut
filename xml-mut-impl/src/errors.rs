use crate::replacer::Replacer;

#[derive(Debug)]
pub enum DeleteError {
    DeleteNothing(String),
    DeleteNoBounds(String),
    DeleteBoundsEmpty(String),
}

#[derive(Debug)]
pub enum AssignError {
    AssignmentSourceValueNotFound(String),
    AssignmentTargetBoundsNotFound(String),
    AssignmentTargetBoundsEmpty(String),
}

#[derive(Debug)]
pub enum ReplaceError {
    NoChange,
    ReplacerOverlap(Replacer, Replacer),
    GeneratedXmlInvalid(roxmltree::Error),
}
