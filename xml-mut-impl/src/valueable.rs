use crate::prelude::{
    AssignError, AttributeExtensions, DeleteError, NewXml, NodeExtensions, Replacer,
};
use roxmltree::*;
use std::ops::Range;
use xml_mut_data::*;

// TODO: String -> &'a str
// <'a, 'input: 'a> : 'input lives longer then 'a here;
pub trait Valueable {
    fn get_bounds(&self, ending: &ValueSelector) -> Option<Range<usize>>;

    fn get_value(&self, ending: &ValueSelector) -> Option<&str>;
    fn get_child_value(&self, selector: &ValuePath) -> Option<String>;
    fn get_value_of(&self, selector: &ValueVariant) -> Option<String> {
        match selector {
            ValueVariant::Selector(selector) => self.get_child_value(selector),
            ValueVariant::LiteralString(val) => Some(val.to_string()),
        }
    }

    fn assign(&self, assignment: &ValueAssignment) -> Result<Replacer, AssignError>;
    fn delete(&self, path: &PathVariant) -> Result<Replacer, DeleteError>;
}

impl<'a, 'input: 'a> Valueable for Node<'a, 'input> {
    fn get_bounds(&self, ending: &ValueSelector) -> Option<Range<usize>> {
        match ending {
            ValueSelector::Attribute(name) => {
                self.get_attribute_with_name(name).map(|a| a.value_range())
            }
            ValueSelector::Text => {
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
            ValueSelector::Tail => {
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

    fn get_value(&self, ending: &ValueSelector) -> Option<&str> {
        self.get_bounds(ending)
            .map(|b| &self.document().input_text()[b])
    }

    fn get_child_value(&self, selector: &ValuePath) -> Option<String> {
        self.find_first_child_element(&selector.node_path)
            .and_then(|c| c.get_value(&selector.selector).map(|v| v.to_string()))
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
        if let ValueSelector::Attribute(attribute_name) = assignment.target.selector {
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
        if assignment.target.selector == ValueSelector::Text
            && assignment_node
                .get_bounds(&assignment.target.selector)
                .is_none()
        {
            return Ok(assignment_node.get_new_node_text_replacer(replacement));
        }

        // NOTE: the rest should have a proper bounds,
        // no more special cases
        let bounds = match assignment_node.get_bounds(&assignment.target.selector) {
            Some(b) => b,
            None => {
                return Err(AssignError::AssignmentTargetBoundsNotFound(format!(
                    "Assignment Node {:?} could not produce valid target bounds. Path {:?}, source {:?}.",
                    assignment_node.tag_name(),
                    &assignment.target.node_path,
                    &assignment.target.selector
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
            PathVariant::Value(v) => (&v.node_path, Some(&v.selector)),
            PathVariant::Node(p) => (p, None),
        };

        let delete_node = if let Some(node) = self.find_first_child_element(path) {
            node
        } else {
            return Err(DeleteError::DeleteNothing(
                "Delete target node not found.".to_string(),
            ));
        };

        let bounds = if let Some(source) = maybe_source {
            if let ValueSelector::Attribute(name) = source {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bounds_01() {
        let xml = r###"<A b="zuzu"> text <B foo="bar"/> tail of B </A>"###;
        let doc = Document::parse(xml).expect("could not parse xml");

        let attribute_source = ValueSelector::Attribute("b");
        let text_source = ValueSelector::Text;

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

        let text_source = ValueSelector::Text;

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

        let text_source = ValueSelector::Text;

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

        let text_source = ValueSelector::Tail;

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
}
