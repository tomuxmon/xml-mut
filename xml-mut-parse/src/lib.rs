mod delete_clause;
mod get_clause;
mod mutation;
mod set_clause;
mod statement;
mod where_clause;

pub mod prelude {
    pub use super::delete_clause::*;
    pub use super::get_clause::*;
    pub use super::mutation::*;
    pub use super::set_clause::*;
    pub use super::statement::*;
    pub use super::where_clause::*;
}
