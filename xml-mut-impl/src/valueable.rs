use crate::prelude::{AssignError, AttributeExtensions, DeleteError, NodeExtensions, Replacer};
use roxmltree::*;
use std::ops::Range;
use xml_mut_data::*;

// TODO: String -> &'a str
// <'a, 'input: 'a> : 'input lives longer then 'a here;
pub trait Valueable {
    fn get_bounds(&self, ending: &ValueSource) -> Option<Range<usize>>;

    fn get_value(&self, ending: &ValueSource) -> Option<&str>;
    fn get_child_value(&self, selector: &ValuePath) -> Option<String>;
    fn get_value_of(&self, selector: &ValueVariant) -> Option<String> {
        match selector {
            ValueVariant::Selector(selector) => self.get_child_value(selector),
            ValueVariant::LiteralString(val) => Some(val.to_string()),
        }
    }

    fn get_new_attribute_replacer(&self, attribute_name: &str, value: String) -> Replacer;
    fn get_new_node_text_replacer(&self, value: String) -> Replacer;
    fn get_new_sub_replacer(&self, path: &ValuePath, value: String) -> Replacer;

    fn assign(&self, assignment: &ValueAssignment) -> Result<Replacer, AssignError>;
    fn delete(&self, path: &PathVariant) -> Result<Replacer, DeleteError>;
}

impl<'a, 'input: 'a> Valueable for Node<'a, 'input> {
    fn get_bounds(&self, ending: &ValueSource) -> Option<Range<usize>> {
        match ending {
            ValueSource::Attribute(name) => {
                self.get_attribute_with_name(name).map(|a| a.value_range())
            }
            ValueSource::Text => {
                if let Some(child) = self.first_child() {
                    if child.is_text() {
                        Some(child.range())
                    } else {
                        let pos = child.range().start;
                        Some(pos..pos)
                    }
                } else {
                    let node_end_tag = format!("</{}>", self.tag_name().name());
                    let node_raw_text = self.get_input_text();
                    if node_raw_text.ends_with(node_end_tag.as_str()) {
                        let pos = self.range().end - node_end_tag.len();
                        Some(pos..pos)
                    } else {
                        None
                    }
                }
                // NOTE: happy path like this does not cover ending tag positioning
                // self.first_child()
                //     .filter(|c| c.is_text())
                //     .map(|c| c.range())
            }
            ValueSource::Tail => {
                if !self.is_element() {
                    return None;
                }
                match self.next_sibling() {
                    Some(node) if node.is_text() => Some(node.range()),
                    Some(node) => {
                        let pos = node.range().start;
                        Some(pos..pos)
                    }
                    None => match self.parent_element().and_then(|p| p.next_sibling()) {
                        Some(next) => {
                            let pos = next.range().start;
                            Some(pos..pos)
                        }
                        None => None,
                    },
                }
            }
        }
    }

    fn get_value(&self, ending: &ValueSource) -> Option<&str> {
        self.get_bounds(ending)
            .map(|b| &self.document().input_text()[b])
    }

    fn get_child_value(&self, selector: &ValuePath) -> Option<String> {
        self.find_first_child_element(&selector.node_path)
            .and_then(|c| c.get_value(&selector.source).map(|v| v.to_string()))
    }

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

    fn assign(&self, assignment: &ValueAssignment) -> Result<Replacer, AssignError> {
        // capture all the permutations in a single struct (desired outcome : bounds wtih value to be replaced)
        // target ValueSource::Attribute (may get attribute or not, if no attribute one must be constructed)
        //      // special case when attribute does not exist and we need to construct it
        // target ValueSource::Text (may get next text node or not, if no text node just get bounds of in between xml tags)
        // target ValueSource::Tail (may get last child text node or not, if no text node just get bounds of in between xml tags of the last child element node and self)

        // source ValueVariant::Selector (may get value or not, if no value bail out, do it early)
        //      source ValueSource::Attribute
        //      source ValueSource::Text
        //      source ValueSource::Tail
        // source ValueVariant::LiteralString (will always contain value)

        let replacement = match self.get_value_of(&assignment.source) {
            Some(val) => val,
            None => {
                // NOTE: there is no predicate ensuring attribute existance yet

                return Err(AssignError::AssignmentSourceValueNotFound(format!(
                    "Node {:?} does not contain a sub node at {:?}.",
                    self.tag_name(),
                    &assignment.source
                )));
            }
        };

        let assignment_node = match self.find_first_child_element(&assignment.target.node_path) {
            Some(node) => node,
            None => {
                return Ok(self.get_new_sub_replacer(&assignment.target, replacement));
            }
        };

        // NOTE: special case when attribute does not exist
        // and we need to construct it
        if let ValueSource::Attribute(attribute_name) = assignment.target.source {
            if assignment_node
                .get_attribute_with_name(attribute_name)
                .is_none()
            {
                return Ok(assignment_node.get_new_attribute_replacer(attribute_name, replacement));
            }
        }

        // NOTE: special case whan assigning to a non existing text node
        // directly under assignment node.
        // here we are sure that the node is
        // self closing element with no children
        if assignment.target.source == ValueSource::Text
            && assignment_node
                .get_bounds(&assignment.target.source)
                .is_none()
        {
            return Ok(assignment_node.get_new_node_text_replacer(replacement));
        }

        // NOTE: the rest should have a proper bounds,
        // no more special cases
        let bounds = match assignment_node.get_bounds(&assignment.target.source) {
            Some(b) => b,
            None => {
                return Err(AssignError::AssignmentTargetBoundsNotFound(format!(
                    "Assignment Node {:?} could not produce valid target bounds. Path {:?}, source {:?}.",
                    assignment_node.tag_name(),
                    &assignment.target.node_path,
                    &assignment.target.source
                )))
            }
        };

        Ok(Replacer {
            bounds,
            replacement,
        })
    }

    fn delete(&self, path_variant: &PathVariant) -> Result<Replacer, DeleteError> {
        let (path, maybe_source) = match path_variant {
            PathVariant::Value(v) => (&v.node_path, Some(&v.source)),
            PathVariant::Path(p) => (p, None),
        };

        let delete_node = if let Some(node) = self.find_first_child_element(path) {
            node
        } else {
            return Err(DeleteError::DeleteNothing(
                "Delete target node not found.".to_string(),
            ));
        };

        let bounds = if let Some(source) = maybe_source {
            if let ValueSource::Attribute(name) = source {
                if let Some(range) = delete_node.get_attribute_with_name(name).map(|a| a.range()) {
                    range
                } else {
                    return Err(DeleteError::DeleteNoBounds(format!(
                        "Bounds do not exist for delete target attribute '{}'",
                        name
                    )));
                }
            } else if let Some(bounds) = delete_node.get_bounds(source) {
                bounds
            } else {
                return Err(DeleteError::DeleteNoBounds(
                    "Bounds do not exist for a delete target".to_string(),
                ));
            }
        } else {
            delete_node.range()
        };

        Ok(Replacer {
            bounds,
            replacement: "".to_string(),
        })
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

        // TODO: construct the value in a brutal way
        let mut path_value = String::new();
        let last_idx = remaining_path.len() - 1;
        // NOTE: opening tags
        for (i, name) in remaining_path.iter().enumerate() {
            path_value.push('<');
            path_value.push_str(name);
            if i == last_idx {
                if let ValueSource::Attribute(attribute_name) = path.source {
                    path_value.push_str(&format!(" {attribute_name}=\"{value}\""));
                }
            }
            path_value.push('>');
        }
        if ValueSource::Text == path.source {
            path_value.push_str(&value);
        }
        // NOTE: closing tags
        for (i, name) in remaining_path.iter().enumerate().rev() {
            path_value.push_str("</");
            path_value.push_str(name);
            path_value.push('>');
            if i == last_idx && ValueSource::Tail == path.source {
                path_value.push_str(&value);
            }
        }

        // current_node?
        // <a/> - self closing -> replacer at bounds: 2..4, replacement : format!(">{new_pathed_value}</{current_node_name}>");
        // <a><g/></a> - contains sub nodes -> replacer at bounds: 3..3, replacement: new_pathed_value
        if let Some(bounds) = current_node.get_bounds(&ValueSource::Text) {
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
    use super::*;

    #[test]
    fn get_bounds_01() {
        let xml = r###"<A b="zuzu"> text <B foo="bar"/> tail of B </A>"###;
        let doc = Document::parse(xml).expect("could not parse xml");

        let attribute_source = ValueSource::Attribute("b");
        let text_source = ValueSource::Text;

        let node = doc.root().first_child().expect("first child should be A");

        let attribute_range = node
            .get_bounds(&attribute_source)
            .expect("attribute b should have bounds");

        let text_range = node
            .get_bounds(&text_source)
            .expect("text should have bounds");

        assert_eq!(attribute_range, 6..10);
        assert_eq!(&doc.input_text()[attribute_range], "zuzu");

        assert_eq!(text_range, 12..18);
        assert_eq!(&doc.input_text()[text_range], " text ");
    }

    #[test]
    fn get_bounds_text_01() {
        let xml = r###"<A b="zuzu"></A>"###;
        let doc = Document::parse(xml).expect("could not parse xml");

        let text_source = ValueSource::Text;

        let node = doc.root().first_child().expect("first child should be A");

        let text_range = node
            .get_bounds(&text_source)
            .expect("text should have empty bounds");

        assert_eq!(text_range, 12..12);
        assert_eq!(&doc.input_text()[text_range], "");
    }

    #[test]
    fn get_bounds_text_02() {
        let xml = r###"<A b="zuzu"><B></B></A>"###;
        let doc = Document::parse(xml).expect("could not parse xml");

        let text_source = ValueSource::Text;

        let node = doc.root().first_child().expect("first child should be A");

        let text_range = node
            .get_bounds(&text_source)
            .expect("text should have empty bounds");

        assert_eq!(text_range, 12..12);
        assert_eq!(&doc.input_text()[text_range], "");
    }

    #[test]
    fn get_bounds_tail_01() {
        let xml = r###"<A b="zuzu"><B></B> B tail </A>"###;
        let doc = Document::parse(xml).expect("could not parse xml");

        let text_source = ValueSource::Tail;

        let node = doc
            .descendants()
            .find(|n| n.has_tag_name("B"))
            .expect("should B");

        let tail_range = node
            .get_bounds(&text_source)
            .expect("tail should have bounds");

        assert_eq!(tail_range, 19..27);
        assert_eq!(&doc.input_text()[tail_range], " B tail ");
    }

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
