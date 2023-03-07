use crate::prelude::{NodeExtensions, Valueable};
use roxmltree::Node;
use xml_mut_data::{Predicate, PredicateEquals, PredicateNodeExists};

pub trait Fitable {
    fn fits_predicates(&self, predicates: &[Predicate]) -> bool {
        predicates.iter().all(|p| self.fits_predicate(p))
    }
    fn fits_predicate(&self, predicate: &Predicate) -> bool {
        match predicate {
            Predicate::NodeExists(p) => self.fits_predicate_exists(p),
            Predicate::Equals(p) => self.fits_predicate_equals(p),
        }
    }
    fn fits_predicate_exists(&self, predicate: &PredicateNodeExists) -> bool;
    fn fits_predicate_equals(&self, predicate: &PredicateEquals) -> bool;
}

impl<'a, 'input: 'a> Fitable for Node<'a, 'input> {
    fn fits_predicate_exists(&self, predicate: &PredicateNodeExists) -> bool {
        self.find_first_child_element(&predicate.node_path)
            .is_some()
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
