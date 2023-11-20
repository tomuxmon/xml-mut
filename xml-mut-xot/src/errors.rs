#[derive(Debug)]
pub enum Error {
    DeleteNothing(String),
    NameNotFound(String),
    NotAnElement,
    TextNodeNotFound,
    TailTextNodeNotFound,
    DeleteNameIsInvalid,
    XotError(xot::Error),
    AssignmentSourceValueNotFound(String),
    ElementNotFound,
    NothingToAdd,
    ParentNotFound,
}
