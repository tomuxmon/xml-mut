use crate::prelude::AttributeExtensions;
use roxmltree::{Attribute, Node};

pub trait NodeExtensions {
    fn get_tag_end_position(&self) -> usize;

    fn get_input_text(&self) -> &str;
    fn is_element_with_name(&self, name: &str) -> bool;
    fn get_attribute_with_name(&self, name: &str) -> Option<Attribute>;

    fn has_parent_elemnt_path(&self, node_path: &[&str]) -> bool;
    fn find_first_child_element(&self, node_path: &[&str]) -> Option<Box<Self>>;
    fn find_first_child_element_aliased(
        &self,
        node_path: &[&str],
        alias: &str,
    ) -> Option<Box<Self>>;
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
        // TODO: rewrite using simple loop (no recursion)
        // TODO: use node.ancestors() and zip!
        if let Some((last, node_path_remaining)) = node_path.split_last() {
            if self.is_element_with_name(last) {
                if let Some(ref parent) = self.parent_element() {
                    parent.has_parent_elemnt_path(node_path_remaining)
                } else {
                    node_path_remaining.is_empty()
                }
            } else {
                // Not an element or tag name does not match
                false
            }
        } else {
            // NOTE: all matched so far. is fit.
            true
        }
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

    fn find_first_child_element_aliased(
        &self,
        node_path: &[&str],
        alias: &str,
    ) -> Option<Box<Self>> {
        if let Some((&path_start, node_path)) = node_path.split_first() {
            if alias.to_lowercase() == path_start.to_lowercase() {
                self.find_first_child_element(node_path)
            } else {
                None
            }
        } else {
            None
        }
    }
}
