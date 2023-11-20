use crate::{
    ok_or_return_false,
    prelude::{NodeExtensions, Valueable},
};
use xml_mut_data::{
    Mutation, PathVariant, Predicate, PredicateEquals, PredicateExists, ValueSelector,
};
use xot::{Node, Xot};

pub trait Fitable {
    fn fits_predicates(&self, node: Node, predicates: &[Predicate]) -> bool {
        predicates.iter().all(|p| self.fits_predicate(node, p))
    }
    fn fits_predicate(&self, node: Node, predicate: &Predicate) -> bool {
        match predicate {
            Predicate::Exists(p) => self.fits_predicate_exists(node, p),
            Predicate::Equals(p) => self.fits_predicate_equals(node, p),
        }
    }
    fn fits_predicate_exists(&self, node: Node, predicate: &PredicateExists) -> bool;
    fn fits_predicate_equals(&self, node: Node, predicate: &PredicateEquals) -> bool;
    fn is_fit(&self, node: Node, mutation: &Mutation) -> bool;
}

impl Fitable for Xot {
    fn fits_predicate_exists(&self, node: Node, predicate: &PredicateExists) -> bool {
        let (path, source_maybe) = match &predicate.path {
            PathVariant::Node(p) => (p, None),
            PathVariant::Value(v) => (&v.node_path, Some(&v.selector)),
        };

        let node = ok_or_return_false!(self.find_first_child_element(node, path));
        let value_source = ok_or_return_false!(source_maybe);

        match value_source {
            ValueSelector::Attribute(name) => {
                let name_id = ok_or_return_false!(self.name(name));
                let element = ok_or_return_false!(self.element(node));
                return element.get_attribute(name_id).is_some();
            }
            ValueSelector::Text => {
                let child = ok_or_return_false!(self.first_child(node));
                self.is_text(child)
            }
            ValueSelector::Tail => {
                let sibling = ok_or_return_false!(self.next_sibling(node));
                self.is_text(sibling)
            }
            ValueSelector::Name => true,
        }
    }

    fn fits_predicate_equals(&self, node: Node, predicate: &PredicateEquals) -> bool {
        let right_side_value = ok_or_return_false!(self.get_value_of(node, &predicate.right_side));
        let left_side_value = ok_or_return_false!(self.get_child_value(node, &predicate.left_side));
        right_side_value == left_side_value
    }

    fn is_fit(&self, node: Node, mutation: &Mutation) -> bool {
        self.find_parent_elemnt(node, &mutation.get_clause.node_selector.path)
            .is_some()
            && self.fits_predicates(
                node,
                &(if let Some(w) = &mutation.where_clause {
                    w.predicates.clone()
                } else {
                    vec![]
                }),
            )
            && self.fits_predicates(
                node,
                &(if let Some(set_statement) = &mutation.set_clause {
                    set_statement.imply_predicates()
                } else {
                    vec![]
                }),
            )
    }
}
