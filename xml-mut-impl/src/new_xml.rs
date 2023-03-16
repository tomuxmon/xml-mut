use crate::prelude::{NodeExtensions, Replacer, Valueable};
use roxmltree::Node;
use xml_mut_data::{ValuePath, ValueSelector};

pub trait NewXml {
    fn get_new_attribute_replacer(&self, attribute_name: &str, value: String) -> Replacer;
    fn get_new_node_text_replacer(&self, value: String) -> Replacer;
    fn get_new_sub_replacer(&self, path: &ValuePath, value: String) -> Replacer;
}

impl<'a, 'input: 'a> NewXml for Node<'a, 'input> {
    fn get_new_attribute_replacer(&self, attribute_name: &str, value: String) -> Replacer {
        let pos = self.get_tag_end_position();

        // TODO: string escaping
        // TODO: pick quotes for attribute value enclosement

        let replacement: String = format!(" {attribute_name}=\"{value}\"");

        Replacer {
            bounds: pos..pos,
            replacement,
        }
    }

    fn get_new_node_text_replacer(&self, value: String) -> Replacer {
        // here we are sure that the node is self closing element with no shildren
        let pos = self.get_tag_end_position();
        let pos_end = self.range().end;
        let name = self.tag_name().name();
        let replacement = format!(">{value}</{name}>");
        Replacer {
            bounds: pos..pos_end,
            replacement,
        }
    }

    fn get_new_sub_replacer(&self, path: &ValuePath, value: String) -> Replacer {
        // we know that full path.node_path does not exist
        // yet part of the path could still exist
        // walk the path and only create at some point

        let mut current_node = Box::new(*self);
        let mut remaining_path = path.node_path.path.clone();
        remaining_path.reverse();
        while let Some(name) = remaining_path.pop() {
            if let Some(child_node) = current_node
                .children()
                .find(|n| n.is_element_with_name(name))
            {
                current_node = Box::new(child_node);
            } else {
                remaining_path.push(name);
                break;
            }
        }
        remaining_path.reverse();

        // NOTE: remaining_path should be at least len() of 1 here.
        // or else find_first_child_element in the assign would have found the node

        // attribute:
        // <a z="makaron" />
        // <a><b><c z="makaron"></b></a>

        // text
        // <a>makaron</a>
        // <a><b><c>makaron</c></b></a>

        // tail
        // <a/>makaron
        // <a><b><c/>makaron</b></a>

        // NOTE: construct the value in a brutal way
        let mut path_value = String::new();
        let last_idx = remaining_path.len() - 1;
        // NOTE: opening tags
        for (i, name) in remaining_path.iter().enumerate() {
            path_value.push('<');
            if ValueSelector::Name == path.selector && i == last_idx {
                path_value.push_str(&value);
            } else {
                path_value.push_str(name);
            }
            if i == last_idx {
                if let ValueSelector::Attribute(attribute_name) = path.selector {
                    path_value.push_str(&format!(" {attribute_name}=\"{value}\""));
                }
            }
            path_value.push('>');
        }
        if ValueSelector::Text == path.selector {
            path_value.push_str(&value);
        }
        // NOTE: closing tags
        for (i, name) in remaining_path.iter().enumerate().rev() {
            path_value.push_str("</");
            if i == last_idx && ValueSelector::Name == path.selector {
                path_value.push_str(&value);
            } else {
                path_value.push_str(name);
            }
            path_value.push('>');
            if i == last_idx && ValueSelector::Tail == path.selector {
                path_value.push_str(&value);
            }
        }

        // current_node?
        // <a/> - self closing -> replacer at bounds: 2..4, replacement : format!(">{new_pathed_value}</{current_node_name}>");
        // <a><g/></a> - contains sub nodes -> replacer at bounds: 3..3, replacement: new_pathed_value
        if let Some(bounds) = current_node.get_bounds(&ValueSelector::Text) {
            Replacer {
                bounds: bounds.start..bounds.start,
                replacement: path_value,
            }
        } else {
            let current_node_name = current_node.tag_name().name();
            Replacer {
                bounds: current_node.get_tag_end_position()..current_node.range().end,
                replacement: format!(">{path_value}</{current_node_name}>"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use roxmltree::Document;

    use super::*;

    #[test]
    fn get_new_node_text_replacer_01() {
        let xml = r###"<A b="zuzu"/>"###;
        let doc = Document::parse(xml).expect("could not parse xml");
        let node = doc.root().first_child().expect("first child should be A");
        let new_text = "smooth operator";

        let replacement = node.get_new_node_text_replacer(new_text.to_string());

        assert_eq!(replacement.bounds, 11..13);
        assert_eq!(replacement.replacement, ">smooth operator</A>");
    }

    #[test]
    fn get_new_node_text_replacer_02() {
        // 2 spaces is the test here
        let xml = r###"<A b="zuzu"  />"###;
        let doc = Document::parse(xml).expect("could not parse xml");
        let node = doc.root().first_child().expect("first child should be A");
        let new_text = "smooth operator";

        let replacement = node.get_new_node_text_replacer(new_text.to_string());

        assert_eq!(replacement.bounds, 11..15);
        assert_eq!(replacement.replacement, ">smooth operator</A>");
    }
}
