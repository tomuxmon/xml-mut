mod errors;
mod fitable;
mod macros;
mod node_ext;
mod valuable;

pub mod prelude {
    pub use super::errors::*;
    pub use super::fitable::*;
    pub use super::macros::*;
    pub use super::node_ext::*;
    pub use super::valuable::*;
}

// TODO: instead of direct replacer.
// define operations to be performed.
//
