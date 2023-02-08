use super::node_ext::*;
use roxmltree::*;
use std::ops::Range;
use xml_mut_parse::prelude::*;

pub trait Valueable {
    fn get_value(&self, selector: &ValueSelector, alias: &str) -> Option<String>;
    fn get_value_bounds(&self, selector: &ValueSelector, alias: &str) -> Option<Range<usize>>;

    fn get_ending_value(&self, ending: &SelectorEnding) -> Option<String>;
    fn get_ending_value_bounds(&self, ending: &SelectorEnding) -> Option<Range<usize>>;
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
}
