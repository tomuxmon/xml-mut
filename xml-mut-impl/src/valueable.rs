use crate::{replace_error::ReplaceError, replacer::Replacer};
use super::node_ext::*;
use roxmltree::*;
use std::ops::Range;
use xml_mut_parse::prelude::*;

pub trait Valueable {
    fn get_value(&self, selector: &ValueSelector, alias: &str) -> Option<String>;
    fn get_value_bounds(&self, selector: &ValueSelector, alias: &str) -> Option<Range<usize>>;

    fn get_ending_value(&self, ending: &SelectorEnding) -> Option<String>;
    fn get_ending_value_bounds(&self, ending: &SelectorEnding) -> Option<Range<usize>>;

    fn assign(&self, assignment: &ValueAssignment, alias: &str) -> Result<Replacer, ReplaceError>;
    fn delete(&self, delete: &DeleteStatement, alias: &str) -> Result<Replacer, ReplaceError>;
}

impl<'a, 'input: 'a> Valueable for Node<'a, 'input> {
    fn get_ending_value_bounds(&self, ending: &SelectorEnding) -> Option<Range<usize>> {
        match ending {
            SelectorEnding::AttributeName(name) => self.get_attribute_with_name(name).map(|a| {
                a.position() + a.name().len() + 2
                    ..a.position() + a.name().len() + a.value().len() + 2
            }),
            SelectorEnding::NodeText => self
                .first_child()
                .filter(|c| c.is_text())
                .map(|c| c.get_bounds()),
        }
    }

    fn get_ending_value(&self, ending: &SelectorEnding) -> Option<String> {
        // TODO: String -> &'a str
        match ending {
            SelectorEnding::AttributeName(name) => self
                .get_attribute_with_name(name)
                .map(|a| a.value().to_string()),
            SelectorEnding::NodeText => self.text().map(|t| t.to_string()),
        }
    }

    // TODO: instead should return a structure with ither node or attribute and bounds
    fn get_value_bounds(&self, selector: &ValueSelector, alias: &str) -> Option<Range<usize>> {
        // selector.node_path should yield only a single element?
        // select first found for now.
        self.find_first_child_element_aliased(&selector.node_path, alias)
            .and_then(|c| c.get_ending_value_bounds(&selector.ending))
    }

    fn get_value(&self, selector: &ValueSelector, alias: &str) -> Option<String> {
        self.find_first_child_element_aliased(&selector.node_path, alias)
            .and_then(|c| c.get_ending_value(&selector.ending))
    }

    fn assign(&self, assignment: &ValueAssignment, alias: &str) -> Result<Replacer, ReplaceError> {
        if let Some(node) =
            self.find_first_child_element_aliased(&assignment.target.node_path, alias)
        {
            // capture all the permutations in a single struct (desired outcome : bounds wtih value to be replaced)
            // target SelectorEnding::AttributeName (may get attribute or not, if no attribute one must be constructed)
            // target SelectorEnding::NodeText (may get next text node or not, if no text node just get bounds of in between xml tags)
            // source ValueVariant::Selector (may get value or not, if no value bail out, do it early)
            // source ValueVariant::LiteralString (will always contain value)

            let bounds_maybe = node.get_ending_value_bounds(&assignment.target.ending);

            match &assignment.target.ending {
                SelectorEnding::AttributeName(name) => {}
                SelectorEnding::NodeText => {}
            }

            match &assignment.source {
                ValueVariant::Selector(selector) => {
                    self.get_value(selector, alias);
                }
                ValueVariant::LiteralString(val) => {
                    // replacers.push(Replacer {
                    //     bounds,
                    //     replacement,
                    // });

                    //Some(val.to_string());
                }
            }
        } else {
            // nothing to mutate, return an info message
        }

        // let mut should_construct_attricute = false;

        // let bounds = if let Some(bounds) =
        //     self.get_value_bounds(&asg.left_side, alias)
        // {
        //     bounds
        // } else {
        //     // NOTE: but it could also be a text node we are setting
        //     // self is wrong node here. should take the one that we got bounds from

        //     let pos = if self.attributes().len() > 0 {
        //         let atr = self
        //             .attributes()
        //             .last()
        //             .expect("already know there is more then one attribute");
        //         atr.position() + atr.name().len() + atr.value().len() + 2
        //     } else {
        //         self.position() + self.tag_name().name().len() + 1
        //     };
        //     should_construct_attricute = true;
        //     pos..pos
        // };

        // TODO: collect failed get_value()
        // if let Some(replacement) = match asg.right_side {
        //     ValueVariant::Selector(selector) => {
        //         self.get_value(&selector, alias)
        //     }
        //     ValueVariant::LiteralString(val) => Some(val.to_string()),
        // } {
        //     let replacement = if should_construct_attricute {
        //         let aa = "=\"".to_string() + replacement.as_str() + "\"";
        //         aa
        //     } else {
        //         replacement
        //     };

        //     replacers.push(Replacer {
        //         bounds,
        //         replacement,
        //     });
        // }

        todo!()
    }

    fn delete(&self, delete: &DeleteStatement, alias: &str) -> Result<Replacer, ReplaceError> {
        if let Some((&path_start, node_path)) = delete.node_path.split_first() {
            if alias.to_lowercase() == path_start.to_lowercase() {
                if let Some(deletable_node) = self.find_first_child_element(node_path) {
                    Ok(Replacer {
                        bounds: deletable_node.get_bounds(),
                        replacement: "".to_string(),
                    })
                } else {
                    Err(ReplaceError::DeleteNothing(
                        "found nothing to delete".to_string(),
                    ))
                }
            } else {
                Err(ReplaceError::DeletePathShouldStartWithAlias(
                    "delete path should start with an alias".to_string(),
                ))
            }
        } else {
            Err(ReplaceError::DeletePathShouldStartWithAlias(
                "delete path should have at least 1 element - alias".to_string(),
            ))
        }
    }
}
