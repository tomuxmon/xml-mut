mod delete_statement;
mod get_statement;
mod mutation;
mod set_statement;
mod statement;
mod where_clause;

pub mod prelude {
    pub use super::delete_statement::*;
    pub use super::get_statement::*;
    pub use super::mutation::*;
    pub use super::set_statement::*;
    pub use super::statement::*;
    pub use super::where_clause::*;
}
