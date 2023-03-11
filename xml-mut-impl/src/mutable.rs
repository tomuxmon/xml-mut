use crate::prelude::{Fitable, NodeExtensions, Replacer, Valueable};
use roxmltree::Node;
use xml_mut_data::Mutation;

pub trait Mutable {
    fn is_fit(&self, mutation: &Mutation) -> bool;
    fn get_replacers(&self, mutation: &Mutation) -> Vec<Replacer>;
}

impl<'a, 'input: 'a> Mutable for Node<'a, 'input> {
    fn is_fit(&self, mutation: &Mutation) -> bool {
        self.has_parent_elemnt_path(&mutation.get_clause.node_selector.path)
            && self.fits_predicates(
                &(if let Some(w) = &mutation.where_clause {
                    w.predicates.clone()
                } else {
                    vec![]
                }),
            )
            && self.fits_predicates(
                &(if let Some(set_statement) = &mutation.set_clause {
                    set_statement.imply_predicates()
                } else {
                    vec![]
                }),
            )
    }

    fn get_replacers(&self, mutation: &Mutation) -> Vec<Replacer> {
        let mut replacers: Vec<Replacer> = vec![];
        if let Some(set_clause) = mutation.set_clause.clone() {
            for ref assignment in set_clause.assignments.into_iter() {
                // TODO: replacers should be inside node bounds
                // TODO: validate replacer.bounds.is_empty()
                match self.assign(assignment) {
                    Ok(replacer) => replacers.push(replacer),
                    Err(error) => println!("{error:?}"), // TODO: better error handling, collect maybe?
                }
            }
        }
        if let Some(ref delete_clause) = mutation.delete_clause.clone() {
            // TODO: count the number of replacers on top of this node and
            // when children is empty spawn another replacer triming end tag.
            // TODO: replacers should be inside node bounds
            // TODO: validate replacer.bounds.is_empty()
            match self.delete(&delete_clause.node_path) {
                Ok(replacer) => replacers.push(replacer),
                Err(error) => println!("{error:?}"), // TODO: better error handling, collect maybe?
            }
        }
        replacers
    }
}
