use std::fmt;

#[derive(Debug)]
pub enum Error {
    DeleteNothing(String),
    NameNotFound(String),
    NotAnElement,
    NotATextNode,
    DeleteNameIsInvalid,
    XotError(xot::Error),
    AssignmentSourceValueNotFound(String),
    NothingToAdd,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "xml mut xot error"
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DeleteNothing(name) => {
                write!(f, "Failed to delete node with name: {}", name)
            }
            Error::NameNotFound(name) => {
                write!(f, "Failed to find node with name: {}", name)
            }
            Error::NotAnElement => write!(f, "The node is not an element."),
            Error::NotATextNode => write!(f, "The node is not a text node."),
            Error::DeleteNameIsInvalid => write!(f, "The delete name is invalid."),
            Error::XotError(err) => write!(f, "Xot error: {}", err),
            Error::AssignmentSourceValueNotFound(name) => {
                write!(
                    f,
                    "Failed to find assignment source value for name: {}",
                    name
                )
            }
            Error::NothingToAdd => write!(f, "There is nothing to add."),
        }
    }
}
