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
        self.get_value(&predicate.left_side)
            .map(|val| val == predicate.right_side)
            .unwrap_or(false)
    }
}
