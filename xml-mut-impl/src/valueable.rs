use crate::prelude::{AssignError, AttributeExtensions, DeleteError, NodeExtensions, Replacer};
use roxmltree::*;
use std::ops::Range;
use xml_mut_data::*;

pub trait Valueable {
    fn get_value_bounds(&self, ending: &ValueSource) -> Option<Range<usize>>;

    fn get_value(&self, ending: &ValueSource) -> Option<String>;
    fn get_child_value(&self, selector: &ValuePath) -> Option<String>;
    fn get_value_of(&self, selector: &ValueVariant) -> Option<String> {
        match selector {
            ValueVariant::Selector(selector) => self.get_child_value(selector),
            ValueVariant::LiteralString(val) => Some(val.to_string()),
        }
    }

    fn get_new_attribute_replacer(&self, attribute_name: &str, value: String) -> Replacer;

    fn assign(&self, assignment: &ValueAssignment) -> Result<Replacer, AssignError>;
    fn delete(&self, path: &NodePath) -> Result<Replacer, DeleteError>;
}

impl<'a, 'input: 'a> Valueable for Node<'a, 'input> {
    fn get_value_bounds(&self, ending: &ValueSource) -> Option<Range<usize>> {
        match ending {
            ValueSource::Attribute(name) => {
                self.get_attribute_with_name(name).map(|a| a.value_range())
            }
            ValueSource::Text => self
                .first_child()
                .filter(|c| c.is_text())
                .map(|c| c.range()),
            ValueSource::Tail => self.last_child().filter(|c| c.is_text()).map(|c| c.range()),
        }
    }

    fn get_value(&self, ending: &ValueSource) -> Option<String> {
        // TODO: String -> &'a str
        match ending {
            ValueSource::Attribute(name) => self
                .get_attribute_with_name(name)
                .map(|a| a.value().to_string()),
            ValueSource::Text => self.text().map(|t| t.to_string()),
            ValueSource::Tail => self.tail().map(|t| t.to_string()),
        }
    }

    fn get_child_value(&self, selector: &ValuePath) -> Option<String> {
        self.find_first_child_element(&selector.node_path)
            .and_then(|c| c.get_value(&selector.source))
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

        let assignment_node = match self.find_first_child_element(&assignment.target.node_path) {
            Some(node) => node,
            None => {
                return Err(AssignError::AssignmentTargetNotFound(format!(
                    "Node {:?} does not contain a sub node with as path {:?}.",
                    self.tag_name(),
                    &assignment.target.node_path
                )))
            }
        };

        let replacement = match assignment_node.get_value_of(&assignment.source) {
            Some(val) => val,
            None => {
                return Err(AssignError::AssignmentSourceValueNotFound(format!(
                    "Node {:?} does not contain a sub node at {:?}.",
                    self.tag_name(),
                    &assignment.source
                )))
            }
        };

        // NOTE: special case when attribute does not exist and we need to construct it
        if let ValueSource::Attribute(attribute_name) = assignment.target.source {
            if assignment_node
                .get_attribute_with_name(attribute_name)
                .is_none()
            {
                return Ok(assignment_node.get_new_attribute_replacer(attribute_name, replacement));
            }
        }

        // TODO: special case whan assigning to a text node of non existing node
        // should it be automatically created or should there be an opt it?

        // NOTE: the rest should have a proper bounds, no more special cases
        let bounds = match assignment_node.get_value_bounds(&assignment.target.source) {
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

    fn delete(&self, path: &NodePath) -> Result<Replacer, DeleteError> {
        if let Some(deletable_node) = self.find_first_child_element(path) {
            Ok(Replacer {
                bounds: deletable_node.range(),
                replacement: "".to_string(),
            })
        } else {
            Err(DeleteError::DeleteNothing(
                "found nothing to delete".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_value_bounds_01() {
        let xml = r###"<A b="zuzu"> text <B foo="bar"/> tail </A>"###;
        let doc = Document::parse(xml).expect("could not parse xml");

        let attribute_source = ValueSource::Attribute("b");
        let tail_source = ValueSource::Tail;
        let text_source = ValueSource::Text;

        let node = doc.root().first_child().expect("first child should be A");

        let attribute_range = node
            .get_value_bounds(&attribute_source)
            .expect("attribute b should have bounds");

        let text_range = node
            .get_value_bounds(&text_source)
            .expect("text should have bounds");

        let tail_range = node
            .get_value_bounds(&tail_source)
            .expect("text should have bounds");

        assert_eq!(attribute_range, 6..10);
        assert_eq!(&doc.input_text()[attribute_range], "zuzu");

        assert_eq!(text_range, 12..18);
        assert_eq!(&doc.input_text()[text_range], " text ");

        assert_eq!(tail_range, 32..38);
        assert_eq!(&doc.input_text()[tail_range], " tail ");
    }

    #[test]
    fn get_value_bounds_02() {
        let xml = r###"<A b="zuzu"></A>"###;
        let doc = Document::parse(xml).expect("could not parse xml");

        let text_source = ValueSource::Text;

        let node = doc.root().first_child().expect("first child should be A");

        let text_range = node
            .get_value_bounds(&text_source)
            .expect("text should have empty bounds");

        assert_eq!(text_range, 12..12);
        assert_eq!(&doc.input_text()[text_range], "");
    }
}
