use crate::prelude::{NodeExtensions, Valueable};
use roxmltree::Node;
use xml_mut_data::{Predicate, PredicateEquals, PredicateExists};

pub trait Fitable {
    fn fits_predicates(&self, predicates: &[Predicate]) -> bool {
        predicates.iter().all(|p| self.fits_predicate(p))
    }
    fn fits_predicate(&self, predicate: &Predicate) -> bool {
        match predicate {
            Predicate::Exists(p) => self.fits_predicate_exists(p),
            Predicate::Equals(p) => self.fits_predicate_equals(p),
        }
    }
    fn fits_predicate_exists(&self, predicate: &PredicateExists) -> bool;
    fn fits_predicate_equals(&self, predicate: &PredicateEquals) -> bool;
}

impl<'a, 'input: 'a> Fitable for Node<'a, 'input> {
    fn fits_predicate_exists(&self, predicate: &PredicateExists) -> bool {
        let (path, source_maybe) = match &predicate.path {
            xml_mut_data::PathVariant::Path(p) => (p, None),
            xml_mut_data::PathVariant::Value(v) => (&v.node_path, Some(&v.source)),
        };
        let child_node = if let Some(node) = self.find_first_child_element(path) {
            node
        } else {
            return false;
        };
        let value_source = if let Some(ref value_source) = source_maybe {
            value_source
        } else {
            return false;
        };
        child_node.get_bounds(value_source).is_some()
    }

    fn fits_predicate_equals(&self, predicate: &PredicateEquals) -> bool {
        let right_side_value = if let Some(value) = self.get_value_of(&predicate.right_side) {
            value
        } else {
            return false;
        };
        let left_side_value = if let Some(value) = self.get_child_value(&predicate.left_side) {
            value
        } else {
            return false;
        };
        right_side_value == left_side_value
    }
}
