use crate::prelude::AttributeExtensions;
use roxmltree::{Attribute, Node};
use std::ops::Deref;

// TODO: use ExpandedName directly when comparing names 
pub trait NodeExtensions {
    fn get_tag_end_position(&self) -> usize;

    fn get_input_text(&self) -> &str;
    fn is_element_with_name(&self, name: &str) -> bool;
    fn get_attribute_with_name(&self, name: &str) -> Option<Attribute>;

    fn has_parent_elemnt_path(&self, node_path: &[&str]) -> bool;
    // TODO: play with lifetimes and get rid of this box
    fn find_first_child_element(&self, node_path: &[&str]) -> Option<Box<Self>>;
}

impl<'a, 'input: 'a> NodeExtensions for Node<'a, 'input> {
    fn get_tag_end_position(&self) -> usize {
        if let Some(pos) = self.attributes().last().map(|a| a.range().end) {
            pos
        } else {
            self.range().start + self.tag_name().name().len() + 1
        }
    }

    fn get_input_text(&self) -> &str {
        &self.document().input_text()[self.range()]
    }

    fn is_element_with_name(&self, name: &str) -> bool {
        self.is_element() && self.tag_name().name().to_lowercase() == name.to_lowercase()
    }

    fn get_attribute_with_name(&self, name: &str) -> Option<Attribute> {
        self.attributes()
            .find(|a| a.name().to_lowercase() == name.to_lowercase())
    }

    fn has_parent_elemnt_path(&self, node_path: &[&str]) -> bool {
        let mut current_node = Box::new(Some(*self));
        for name in node_path.iter().rev() {
            let current = if let Some(node) = current_node.deref() {
                node
            } else {
                return false;
            };
            if !current.is_element_with_name(name) {
                return false;
            }
            current_node = if let Some(ref parent) = current.parent_element() {
                Box::new(Some(*parent))
            } else {
                Box::new(None)
            };
        }
        true
    }

    fn find_first_child_element(&self, node_path: &[&str]) -> Option<Box<Self>> {
        let mut current_node = Box::new(*self);
        for &name in node_path {
            if let Some(child_node) = current_node
                .children()
                .find(|n| n.is_element_with_name(name))
            {
                current_node = Box::new(child_node);
            } else {
                return None;
            }
        }
        Some(current_node)
    }
}
