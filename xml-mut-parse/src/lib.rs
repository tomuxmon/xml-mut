mod delete_statement;
mod get_statement;
mod mutation;
mod set_statement;
mod structure;
mod tests;
mod where_clause;

pub mod prelude {
    pub use super::delete_statement::*;
    pub use super::get_statement::*;
    pub use super::mutation::*;
    pub use super::set_statement::*;
    pub use super::structure::*;
    pub use super::where_clause::*;
}
