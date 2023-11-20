use crate::ok_or_return_false;
use xot::{Node, Xot};

pub trait NodeExtensions {
    fn is_element_with_name(&self, node: Node, name: &str) -> bool;
    fn find_parent_elemnt(&self, node: Node, node_path: &[&str]) -> Option<Node>;
    // TODO: node_path_exists(Node, &[&str])
    // TODO: iter_children(Node, &[&str])
    fn find_first_child_element(&self, node: Node, node_path: &[&str]) -> Option<Node>;
    /// Returns the name without a namespace if it is an element
    fn get_name(&self, node: Node) -> Option<&str>;
}

impl NodeExtensions for Xot {
    fn is_element_with_name(&self, node: Node, name: &str) -> bool {
        let element = ok_or_return_false!(self.element(node));
        let (el_name, _) = self.name_ns_str(element.name());
        el_name == name
    }
    fn find_parent_elemnt(&self, node: Node, node_path: &[&str]) -> Option<Node> {
        let mut current_node = node;
        for name in node_path.iter().rev() {
            if !self.is_element_with_name(current_node, name) {
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
