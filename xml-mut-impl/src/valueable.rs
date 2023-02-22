use crate::prelude::{AssignError, AttributeExtensions, DeleteError, NodeExtensions, Replacer};
use roxmltree::*;
use std::ops::Range;
use xml_mut_data::*;

pub trait Valueable {
    fn get_value(&self, selector: &ValuePath) -> Option<String>;
    fn get_value_bounds(&self, selector: &ValuePath) -> Option<Range<usize>>;

    fn get_value_of(&self, selector: &ValueVariant) -> Option<String>;

    fn get_source_value(&self, ending: &ValueSource) -> Option<String>;
    fn get_source_bounds(&self, ending: &ValueSource) -> Option<Range<usize>>;

    fn get_new_attribute_replacer(&self, attribute_name: &str, value: String) -> Replacer;

    fn assign(&self, assignment: &ValueAssignment) -> Result<Replacer, AssignError>;
    fn delete(&self, delete: &DeleteStatement) -> Result<Replacer, DeleteError>;
}

impl<'a, 'input: 'a> Valueable for Node<'a, 'input> {
    fn get_source_bounds(&self, ending: &ValueSource) -> Option<Range<usize>> {
        match ending {
            ValueSource::Attribute(name) => {
                self.get_attribute_with_name(name).map(|a| a.value_range())
            }
            ValueSource::Text => self
                .first_child()
                .filter(|c| c.is_text())
                .map(|c| c.range()),
            ValueSource::Tail => todo!(),
        }
    }

    fn get_source_value(&self, ending: &ValueSource) -> Option<String> {
        // TODO: String -> &'a str
        match ending {
            ValueSource::Attribute(name) => self
                .get_attribute_with_name(name)
                .map(|a| a.value().to_string()),
            ValueSource::Text => self.text().map(|t| t.to_string()),
            ValueSource::Tail => todo!(),
        }
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

    fn get_value_bounds(&self, selector: &ValuePath) -> Option<Range<usize>> {
        // selector.node_path should yield only a single element?
        // select first found for now.
        self.find_first_child_element(&selector.node_path)
            .and_then(|c| c.get_source_bounds(&selector.source))
    }

    fn get_value(&self, selector: &ValuePath) -> Option<String> {
        self.find_first_child_element(&selector.node_path)
            .and_then(|c| c.get_source_value(&selector.source))
    }

    fn get_value_of(&self, selector: &ValueVariant) -> Option<String> {
        match selector {
            ValueVariant::Selector(selector) => self.get_value(selector),
            ValueVariant::LiteralString(val) => Some(val.to_string()),
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
        let bounds = match assignment_node.get_source_bounds(&assignment.target.source) {
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

    fn delete(&self, delete: &DeleteStatement) -> Result<Replacer, DeleteError> {
        if let Some(deletable_node) = self.find_first_child_element(&delete.node_path) {
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
