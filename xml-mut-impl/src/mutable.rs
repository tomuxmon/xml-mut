use crate::prelude::{Fitable, NodeExtensions, ReplaceError, Replacer, Valueable};
use roxmltree::Node;
use xml_mut_parse::prelude::Mutation;

pub trait Mutable {
    // TODO: method to get element node last attribute possition (or node closing tag position)
    // TODO: method to construct an attribute with a name and value pair
    fn is_fit(&self, mutation: &Mutation) -> bool;
    fn get_replacers(&self, mutation: &Mutation) -> Vec<Replacer>;
    fn apply(&self, replacers: &[Replacer]) -> Result<Box<Self>, ReplaceError>;
}

impl<'a, 'input: 'a> Mutable for Node<'a, 'input> {
    fn is_fit(&self, mutation: &Mutation) -> bool {
        self.has_parent_elemnt_path(&mutation.get.node_selector.path)
            && self.fits_predicates(
                &mutation.where_clause.predicates,
                mutation.get.node_selector.alias,
            )
    }
    fn get_replacers(&self, mutation: &Mutation) -> Vec<Replacer> {
        let mut replacers: Vec<Replacer> = vec![];
        if let Some(set_op) = mutation.set.clone() {
            for ref assignment in set_op.assignments.into_iter() {
                match self.assign(assignment, mutation.get.node_selector.alias) {
                    Ok(replacer) => replacers.push(replacer),
                    Err(error) => println!("{error:?}"), // TODO: better error handling, collect maybe?
                }
            }
        }
        if let Some(ref delete_op) = mutation.delete.clone() {
            match self.delete(delete_op, mutation.get.node_selector.alias) {
                Ok(replacer) => replacers.push(replacer),
                Err(error) => println!("{error:?}"), // TODO: better error handling, collect maybe?
            }
        }
        replacers
    }

    fn apply(&self, replacers: &[Replacer]) -> Result<Box<Self>, ReplaceError> {
        // TODO: validate replacer overlaps
        // TODO: reconstruct node with replacers applied
        // TODO: reparse resulting node to validate it.
        // return re parsed node

        todo!()
    }
}
