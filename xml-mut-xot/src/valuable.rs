use crate::prelude::{Error, Fitable, NodeExtensions};
use xml_mut_data::{
    Mutation, PathVariant, ValueAssignment, ValuePath, ValueSelector, ValueVariant,
};
use xot::{Node, Xot};

pub trait Valueable {
    fn get_value(&self, node: Node, selector: &ValueSelector) -> Option<&str>;
    fn get_child_value(&self, node: Node, path: &ValuePath) -> Option<&str>;
    fn get_value_of<'a>(&'a self, node: Node, variant: &'a ValueVariant) -> Option<&str> {
        match variant {
            ValueVariant::Selector(path) => self.get_child_value(node, path),
            ValueVariant::LiteralString(val) => Some(val),
        }
    }
    fn add_sub_tree(&mut self, node: Node, path: &ValuePath, value: String) -> Result<(), Error>;
    fn assign(&mut self, node: Node, assignment: &ValueAssignment) -> Result<(), Error>;
    fn delete(&mut self, node: Node, path_variant: &PathVariant) -> Result<(), Error>;
    fn apply(&mut self, node: Node, mutation: &Mutation) -> Result<(), Error>;
    fn apply_all(&mut self, node: Node, mutations: &[&Mutation]) -> Result<(), Error>;
}

impl Valueable for Xot {
    fn get_value(&self, node: Node, selector: &ValueSelector) -> Option<&str> {
        match selector {
            ValueSelector::Attribute(name) => self.element(node)?.get_attribute(self.name(name)?),
            ValueSelector::Text => self.first_child(node).and_then(|t| self.text_str(t)),
            ValueSelector::Tail => self.next_sibling(node).and_then(|t| self.text_str(t)),
            ValueSelector::Name => self.get_name(node),
        }
    }
    fn get_child_value(&self, node: Node, path: &ValuePath) -> Option<&str> {
        self.find_first_child_element(node, &path.node_path)
            .and_then(|n| self.get_value(n, &path.selector))
    }

    fn assign(&mut self, node: Node, assignment: &ValueAssignment) -> Result<(), Error> {
        let replacement = self
            .get_value_of(node, &assignment.source)
            .ok_or(
                // NOTE: there is no predicate ensuring attribute existance yet
                Error::AssignmentSourceValueNotFound(format!(
                    "Node {:?} does not contain a sub node at {:?}.",
                    self.get_name(node).unwrap_or("noname"),
                    &assignment.source
                )),
            )?
            .to_string();

        // TODO: instead of self.find_first_child_element
        // use enumerable and assign for all matches
        let node = match self.find_first_child_element(node, &assignment.target.node_path) {
            Some(n) => n,
            None => {
                self.add_sub_tree(node, &assignment.target, replacement)?;
                return Ok(());
            }
        };

        match assignment.target.selector {
            ValueSelector::Attribute(name) => {
                let name_id = self.add_name(name);
                let element = self.element_mut(node).ok_or(Error::ElementNotFound)?;
                element.set_attribute(name_id, replacement);
            }
            ValueSelector::Text => {
                if let Some(text) = self.first_child(node).and_then(|n| self.text_mut(n)) {
                    text.set(replacement);
                } else {
                    let text_node = self.new_text(replacement.as_str());
                    self.prepend(node, text_node).map_err(Error::XotError)?;
                    return Ok(());
                }
            }
            ValueSelector::Tail => {
                if let Some(text) = self.next_sibling(node).and_then(|n| self.text_mut(n)) {
                    text.set(replacement);
                } else {
                    let text_node = self.new_text(replacement.as_str());
                    self.insert_after(node, text_node)
                        .map_err(Error::XotError)?;
                    return Ok(());
                }
            }
            ValueSelector::Name => {
                let name = self.add_name(replacement.as_str());
                let element = self.element_mut(node).ok_or(Error::ElementNotFound)?;
                element.set_name(name);
            }
        }

        Ok(())
    }

    fn add_sub_tree(&mut self, node: Node, path: &ValuePath, value: String) -> Result<(), Error> {
        let mut current_node = node;
        let mut path_vec = path.node_path.to_vec();
        path_vec.reverse();

        loop {
            if let Some(name) = path_vec.pop() {
                if let Some(child_node) = self
                    .children(current_node)
                    .find(|n| self.is_element_with_name(*n, name))
                {
                    current_node = child_node;
                } else {
                    // depleated, can create sub tree from thid point
                    path_vec.push(name);
                    path_vec.reverse();
                    break;
                }
            } else {
                return Err(Error::NothingToAdd);
            }
        }

        if ValueSelector::Name == path.selector && path_vec.pop().is_some() {
            path_vec.push(value.as_str());
        }

        for name in path_vec {
            let name_id = self.add_name(name);
            let element_node = self.new_element(name_id);
            self.append(current_node, element_node)
                .map_err(Error::XotError)?;
            current_node = element_node;
        }

        match path.selector {
            ValueSelector::Attribute(name) => {
                let attribute_name = self.add_name(name);
                let element = self.element_mut(current_node).unwrap();
                element.set_attribute(attribute_name, value);
            }
            ValueSelector::Text => {
                self.append_text(current_node, value.as_str())
                    .map_err(Error::XotError)?;
            }
            ValueSelector::Tail => {
                let text_node = self.new_text(value.as_str());
                self.insert_after(current_node, text_node)
                    .map_err(Error::XotError)?;
            }
            ValueSelector::Name => (),
        }

        Ok(())
    }

    fn delete(&mut self, node: Node, path_variant: &PathVariant) -> Result<(), Error> {
        let (path, maybe_source) = match path_variant {
            PathVariant::Value(v) => (&v.node_path, Some(&v.selector)),
            PathVariant::Node(p) => (p, None),
        };

        let node = self
            .find_first_child_element(node, path)
            .ok_or(Error::DeleteNothing(
                "Delete target node not found.".to_string(),
            ))?;

        if let Some(source) = maybe_source {
            match source {
                ValueSelector::Attribute(name) => {
                    let name_id = self
                        .name(name)
                        .ok_or(Error::NameNotFound(name.to_string()))?;
                    let element = self.element_mut(node).ok_or(Error::NotAnElement)?;
                    element.remove_attribute(name_id);
                }
                ValueSelector::Text => {
                    // makes no sense to raise error when nothing to delete?
                    let text_node = self.first_child(node).ok_or(Error::TextNodeNotFound)?;
                    let text = self.text_mut(text_node).ok_or(Error::TextNodeNotFound)?;
                    text.set("");
                }
                ValueSelector::Tail => {
                    // makes no sense to raise error when nothing to delete?
                    let text_node = self.next_sibling(node).ok_or(Error::TailTextNodeNotFound)?;
                    let text = self
                        .text_mut(text_node)
                        .ok_or(Error::TailTextNodeNotFound)?;
                    text.set("");
                }
                ValueSelector::Name => {
                    return Err(Error::DeleteNameIsInvalid);
                }
            }

            return Ok(());
        }

        self.remove(node).map_err(Error::XotError)?;

        Ok(())
    }

    fn apply(&mut self, node: Node, mutation: &Mutation) -> Result<(), Error> {
        if let Some(set_clause) = mutation.set_clause.clone() {
            for ref assignment in set_clause.assignments.into_iter() {
                self.assign(node, assignment)?;
            }
        }
        if let Some(ref delete_clause) = mutation.delete_clause.clone() {
            for path_var in &delete_clause.targets {
                self.delete(node, path_var)?;
            }
        }

        Ok(())
    }

    fn apply_all(&mut self, node: Node, mutations: &[&Mutation]) -> Result<(), Error> {
        for &mutation in mutations {
            let nodes: Vec<Node> = self.descendants(node).collect();
            for node in nodes {
                if self.is_fit(node, mutation) {
                    self.apply(node, mutation)?;
                }
            }
        }
        Ok(())
    }
}
