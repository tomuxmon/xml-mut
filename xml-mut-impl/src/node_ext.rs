use roxmltree::{Attribute, Node, NodeId, NodeType};
use std::ops::Range;

pub trait NodeExtensions {
    fn get_bounds(&self) -> Range<usize>;
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
    fn get_bounds(&self) -> Range<usize> {
        let pos_start = self.position();
        let pos_end = match self.node_type() {
            NodeType::Text => self.position() + self.text().map_or(0, |t| t.len()),
            _ => self.next_sibling().map(|n| n.position()).unwrap_or(
                self.document()
                    .get_node(NodeId::new(self.id().get() + 1))
                    .map(|s| s.position())
                    .unwrap_or(self.document().input_text().len()),
            ),
        };
        pos_start..pos_end
    }

    fn get_input_text(&self) -> &str {
        &self.document().input_text()[self.get_bounds()]
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
