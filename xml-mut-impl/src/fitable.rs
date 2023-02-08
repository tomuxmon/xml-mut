use crate::prelude::{NodeExtensions, Valueable};
use roxmltree::Node;
use xml_mut_parse::prelude::{Predicate, PredicateEquals, PredicateNodeExists};

pub trait Fitable {
    fn fits_predicates(&self, predicates: &[Predicate], alias: &str) -> bool {
        predicates.iter().all(|p| self.fits_predicate(p, alias))
    }
    fn fits_predicate(&self, predicate: &Predicate, alias: &str) -> bool {
        match predicate {
            Predicate::NodeExists(p) => self.fits_predicate_exists(p, alias),
            Predicate::Equals(p) => self.fits_predicate_equals(p, alias),
        }
    }
    fn fits_predicate_exists(&self, predicate: &PredicateNodeExists, alias: &str) -> bool;
    fn fits_predicate_equals(&self, predicate: &PredicateEquals, alias: &str) -> bool;
}

impl<'a, 'input: 'a> Fitable for Node<'a, 'input> {
    fn fits_predicate_exists(&self, predicate: &PredicateNodeExists, alias: &str) -> bool {
        self.find_first_child_element_aliased(&predicate.node_path, alias)
            .is_some()
    }
    fn fits_predicate_equals(&self, predicate: &PredicateEquals, alias: &str) -> bool {
        self.get_value(&predicate.left_side, alias)
            .map(|val| val == predicate.right_side)
            .unwrap_or(false)
    }
}
