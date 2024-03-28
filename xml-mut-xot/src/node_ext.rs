use crate::ok_or_return_false;
use xot::{Node, Xot};

pub trait NodeExtensions {
    fn tail_text_node(&self, node: Node) -> Option<Node>;
    fn text_node(&self, node: Node) -> Option<Node>;
    fn is_element_with_name(&self, node: Node, name: &str) -> bool;
    fn find_parent_elemnt(&self, node: Node, node_path: &[&str]) -> Option<Node>;
    fn find_first_child_element(&self, node: Node, node_path: &[&str]) -> Option<Node>;
    /// Returns the name without a namespace if it is an element
    fn get_name(&self, node: Node) -> Option<&str>;
}

impl NodeExtensions for Xot {
    fn tail_text_node(&self, node: Node) -> Option<Node> {
        let node = self.next_sibling(node)?;
        if self.is_text(node) {
            Some(node)
        } else {
            None
        }
    }
    fn text_node(&self, node: Node) -> Option<Node> {
        let node = self.first_child(node)?;
        if self.is_text(node) {
            Some(node)
        } else {
            None
        }
    }
    fn is_element_with_name(&self, node: Node, name: &str) -> bool {
        let element = ok_or_return_false!(self.element(node)); // element access panics! unreachable!("Try to access a freed node")
        let (el_name, _) = self.name_ns_str(element.name());
        el_name == name
    }
    fn find_parent_elemnt(&self, node: Node, node_path: &[&str]) -> Option<Node> {
        let mut current_node = node;
        for name in node_path.iter().rev() {
            if !self.is_element_with_name(current_node, name) {
                // panicks! when node already freed
                return None;
            }
            if let Some(n) = self.parent(current_node) {
                current_node = n;
            } else {
                break;
            }
        }
        Some(current_node)
    }
    fn find_first_child_element(&self, node: Node, node_path: &[&str]) -> Option<Node> {
        let mut current_node = node;
        for &name in node_path {
            if let Some(child_node) = self
                .children(current_node)
                .find(|n| self.is_element_with_name(*n, name))
            {
                current_node = child_node;
            } else {
                return None;
            }
        }
        Some(current_node)
    }
    fn get_name(&self, node: Node) -> Option<&str> {
        let (name, _) = self.name_ns_str(self.element(node)?.name());
        Some(name)
    }
}
