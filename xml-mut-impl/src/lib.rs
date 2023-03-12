mod attribute_ext;
mod document_ext;
mod errors;
mod fitable;
mod mutable;
mod new_xml;
mod node_ext;
mod replacer;
mod valueable;

pub mod prelude {
    pub use super::attribute_ext::*;
    pub use super::document_ext::*;
    pub use super::errors::*;
    pub use super::fitable::*;
    pub use super::mutable::*;
    pub use super::new_xml::*;
    pub use super::node_ext::*;
    pub use super::replacer::*;
    pub use super::valueable::*;
}
