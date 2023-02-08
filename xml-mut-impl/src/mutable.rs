use crate::prelude::{Fitable, NodeExtensions, ReplaceError, Replacer, Valueable};
use roxmltree::Node;
use xml_mut_parse::prelude::{Mutation, ValueVariant};

pub trait Mutable {
    // TODO: method to get element node last attribute possition (or node closing tag position)
    // TODO: method to construct an attribute with a name and value pair
    fn is_fit(&self, mutation: &Mutation) -> bool;
    fn get_mutation_replaces(&self, mutation: &Mutation) -> Vec<Replacer>;
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
    fn get_mutation_replaces(&self, mutation: &Mutation) -> Vec<Replacer> {
        let mut replacers: Vec<Replacer> = vec![];
        if let Some(set_op) = mutation.set.clone() {
            for asg in set_op.assignments.into_iter() {
                let mut should_construct_attricute = false;

                let bounds = if let Some(bounds) =
                    self.get_value_bounds(&asg.left_side, mutation.get.node_selector.alias)
                {
                    bounds
                } else {
                    // NOTE: but it could also be a text node we are setting
                    // self is wrong node here. should take the one that we got bounds from

                    let pos = if self.attributes().len() > 0 {
                        let atr = self
                            .attributes()
                            .last()
                            .expect("already know there is more then one attribute");
                        atr.position() + atr.name().len() + atr.value().len() + 2
                    } else {
                        self.position() + self.tag_name().name().len() + 1
                    };
                    should_construct_attricute = true;
                    pos..pos
                };

                // TODO: collect failed get_value()
                if let Some(replacement) = match asg.right_side {
                    ValueVariant::Selector(selector) => {
                        self.get_value(&selector, mutation.get.node_selector.alias)
                    }
                    ValueVariant::LiteralString(val) => Some(val.to_string()),
                } {
                    let replacement = if should_construct_attricute {
                        let aa = "=\"".to_string() + replacement.as_str() + "\"";
                        aa
                    } else {
                        replacement
                    };

                    replacers.push(Replacer {
                        bounds,
                        replacement,
                    });
                }
            }
        }

        if let Some(delete_op) = mutation.delete.clone() {
            if let Some((&path_start, node_path)) = delete_op.node_path.split_first() {
                if mutation.get.node_selector.alias.to_lowercase() == path_start.to_lowercase() {
                    if let Some(deletable_node) = self.find_first_child_element(node_path) {
                        replacers.push(Replacer {
                            bounds: deletable_node.get_bounds(),
                            replacement: "".to_string(),
                        });
                    } else {
                        println!("found nothing to delete");
                    }
                } else {
                    println!("delete path should start with an alias");
                }
            }
        }

        // TOOD: self closing element replacer when empty node

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
