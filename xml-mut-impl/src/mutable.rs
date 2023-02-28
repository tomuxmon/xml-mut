use crate::prelude::{Fitable, NodeExtensions, ReplaceError, Replacer, Valueable};
use roxmltree::{Document, Node};
use xml_mut_data::Mutation;

pub trait Mutable {
    fn is_fit(&self, mutation: &Mutation) -> bool;
    fn apply(&self, mutation: &Mutation) -> Result<Replacer, ReplaceError>;

    fn get_replacers(&self, mutation: &Mutation) -> Vec<Replacer>;
    fn apply_replacers(&self, replacers: &[Replacer]) -> Result<Replacer, ReplaceError>;
}

impl<'a, 'input: 'a> Mutable for Node<'a, 'input> {
    fn is_fit(&self, mutation: &Mutation) -> bool {
        self.has_parent_elemnt_path(&mutation.get.node_selector.path)
            && self.fits_predicates(
                &(if let Some(w) = &mutation.where_clause {
                    w.predicates.clone()
                } else {
                    vec![]
                }),
            )
            && self.fits_predicates(
                &(if let Some(set_statement) = &mutation.set {
                    set_statement.imply_predicates()
                } else {
                    vec![]
                }),
            )
    }

    fn get_replacers(&self, mutation: &Mutation) -> Vec<Replacer> {
        let mut replacers: Vec<Replacer> = vec![];
        if let Some(set_op) = mutation.set.clone() {
            for ref assignment in set_op.assignments.into_iter() {
                match self.assign(assignment) {
                    Ok(replacer) => replacers.push(replacer),
                    Err(error) => println!("{error:?}"), // TODO: better error handling, collect maybe?
                }
            }
        }
        if let Some(ref delete_statement) = mutation.delete.clone() {
            match self.delete(&delete_statement.node_path) {
                Ok(replacer) => replacers.push(replacer),
                Err(error) => println!("{error:?}"), // TODO: better error handling, collect maybe?
            }
        }
        replacers
    }

    fn apply_replacers(&self, replacers: &[Replacer]) -> Result<Replacer, ReplaceError> {
        // todo: validate replacer.bounds.is_empty()
        // TODO: validate replacer overlaps
        // TODO: replacers should be inside node bounds
        // TODO: reconstruct node with replacers applied
        // TODO: reparse resulting node to validate it.
        // return re parsed node
        // calculate new string size based on replacer bounds and value difference

        let node_bounds = self.range();
        let current_text = &self.document().input_text()[node_bounds.clone()];
        let new_len = usize::try_from(
            i32::try_from(node_bounds.end - node_bounds.start).unwrap_or(0)
                + replacers.iter().map(|r| r.len_diff()).sum::<i32>(),
        )
        .unwrap_or(current_text.len());
        let mut new_text = String::with_capacity(new_len);
        let mut replacers_sorted = replacers.to_vec();
        replacers_sorted.sort_by(|a, b| a.bounds_cmp(b));

        let mut offset = node_bounds.start;
        for replacer in replacers_sorted {
            new_text.push_str(&self.document().input_text()[offset..replacer.bounds.start]);
            new_text.push_str(&replacer.replacement);
            offset = replacer.bounds.end;
        }
        new_text.push_str(&self.document().input_text()[offset..node_bounds.end]);

        let doc = if let Ok(doc) = Document::parse(&new_text) {
            doc
        } else {
            return Err(ReplaceError::GeneratedXmlInvalid(
                "xml is invalid after applying replacers".to_string(),
            ));
        };

        let node = doc.root_element();
        if node.first_element_child().is_none() && node.text().map_or(true, |t| t.trim().is_empty())
        {
            new_text = format!("{} {}", &new_text[0..node.get_tag_end_position()], "/>");
        }

        Ok(Replacer {
            bounds: node_bounds,
            replacement: new_text,
        })
    }

    fn apply(&self, mutation: &Mutation) -> Result<Replacer, ReplaceError> {
        self.apply_replacers(&self.get_replacers(mutation))
    }
}
