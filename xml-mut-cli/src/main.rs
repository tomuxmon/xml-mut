use roxmltree::*;
use std::ops::Range;
use xml_mut_parse::prelude::*;

fn main() {
    // note: set p@version = p/version@>text should imply where exists p/version
    // since it is required in PredicateEquals.right_side

    let mutation_definition = r###"get ItemGroup/PackageReference as p
where exists p/version
set p@version = p/version@>text
delete p/version"###;

    let xml = r###"<ItemGroup>
<PackageReference Include="zuzu" Version="sus"> bleep <version>3.0</version> bloop </PackageReference>
<PackageReference Include="zizi"> <Version>3.0</Version> </PackageReference>
<PackageReference Include="zohan" Version="4.0" />
</ItemGroup>"###;

    let (_, mutation) = mutation(mutation_definition).expect("could not parse mutation");
    println!("{mutation:?}");

    let doc: Document = Document::parse(xml).unwrap();

    for node in doc.descendants().filter(|n| {
        n.has_parent_elemnt_path(&mutation.get.node_selector.path)
            && n.fits_predicates(
                &mutation.where_clause.predicates,
                mutation.get.node_selector.alias,
            )
    }) {
        let bounds = node.get_bounds();

        println!(
            "found mutatable node: '{:?}', bounds: {:?}",
            node.tag_name(),
            bounds
        );

        // TODO: itterate descendants and reconstruct node text while mutating it
        for a in node.descendants() {
            let name = a.tag_name();
            let text = a.get_input_text();
            let bounds = a.get_bounds();
            let node_type = a.node_type();
            println!(" {node_type:?} ({bounds:?}); name: {name:?}; full text: {text:?};");
        }

        // TODO: perform node update, set operation
        if let Some(set_op) = mutation.set.clone() {
            for asg in set_op.assignments.into_iter() {
                // node.position()
                // TODO: method to get element node last attribute possition (or node closing tag position)
                // TODO: method to construct an attribute with a name and value pair
                // punch holes in the immutable document and write it
                // TODO: do not forget you can do Some(ref val)
                // node.position()

                // TODO: get or construct a target node and value receiver
                let left_side_val =
                    node.get_value(&asg.left_side, mutation.get.node_selector.alias);

                // TOOD: set nodes value with new value
                let right_side_val = match asg.right_side {
                    ValueVariant::Selector(selector) => {
                        node.get_value(&selector, mutation.get.node_selector.alias)
                    }
                    ValueVariant::LiteralString(val) => Some(val.to_string()),
                };
                // TODO: write xml

                println!("left side value: '{left_side_val:?}'");
                println!("right side value: '{right_side_val:?}'");
            }
        }

        // TODO: perform node removal, delete operation
        if let Some(delete_op) = mutation.delete.clone() {
            if let Some((&path_start, node_path)) = delete_op.node_path.split_first() {
                if mutation.get.node_selector.alias.to_lowercase() == path_start.to_lowercase() {
                    if let Some(deletable_node) = node.find_first_child_element(node_path) {
                        println!("will delete '{:?}'", deletable_node.tag_name());
                    } else {
                        println!("found nothing to delete");
                    }
                } else {
                    println!("delete path should start with an alias");
                }
            }
        }
    }
}

pub trait Searchable<'a, 'input> {
    fn get_bounds(&self) -> Range<usize>;
    fn get_input_text(&self) -> &str;
    fn has_parent_elemnt_path(&self, node_path: &[&str]) -> bool;
    fn has_child_element_path(&self, node_path: &[&str]) -> bool;
    fn fits_predicates(&self, predicates: &[Predicate], alias: &str) -> bool {
        predicates.iter().all(|p| self.fits_predicate(p, alias))
    }
    fn fits_predicate(&self, predicate: &Predicate, alias: &str) -> bool {
        match predicate {
            Predicate::NodeExists(p) => self.fits_predicate_exists(p, alias),
            Predicate::Equals(p) => self.fits_predicate_equals(p, alias),
        }
    }
    fn fits_predicate_exists(&self, predicate: &PredicateNodeExists, alias: &str) -> bool {
        if let Some((&path_start, node_path)) = predicate.node_path.split_first() {
            if alias.to_lowercase() == path_start.to_lowercase() {
                self.has_child_element_path(node_path)
            } else {
                false
            }
        } else {
            false
        }
    }
    fn fits_predicate_equals(&self, predicate: &PredicateEquals, alias: &str) -> bool {
        self.get_value(&predicate.left_side, alias)
            .map(|val| val == predicate.right_side)
            .unwrap_or(false)
    }
    fn is_element_with_name(&self, name: &str) -> bool;
    fn get_attribute_with_name(&self, name: &str) -> Option<Attribute>;
    fn get_ending_value(&self, ending: &SelectorEnding) -> Option<String>;
    fn get_value(&self, selector: &ValueSelector, alias: &str) -> Option<String>;
    fn find_first_child_element(&self, node_path: &[&str]) -> Option<Box<Self>>;
}

impl<'a, 'input: 'a> Searchable<'a, 'input> for Node<'a, 'input> {
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

    fn has_child_element_path(&self, node_path: &[&str]) -> bool {
        if let Some((first, node_path_remaining)) = node_path.split_first() {
            self.children().into_iter().any(|n| {
                n.is_element_with_name(first) && n.has_child_element_path(node_path_remaining)
            })
        } else {
            true
        }
    }
    fn is_element_with_name(&self, name: &str) -> bool {
        self.is_element() && self.tag_name().name().to_lowercase() == name.to_lowercase()
    }

    fn get_attribute_with_name(&self, name: &str) -> Option<Attribute> {
        self.attributes()
            .find(|a| a.name().to_lowercase() == name.to_lowercase())
    }

    fn get_ending_value(&self, ending: &SelectorEnding) -> Option<String> {
        match ending {
            SelectorEnding::AttributeName(name) => self
                .get_attribute_with_name(name)
                .map(|a| a.value().to_string()),
            SelectorEnding::NodeText => self.text().map(|t| t.to_string()),
        }
    }

    fn get_value(&self, selector: &ValueSelector, alias: &str) -> Option<String> {
        // selector.node_path should yield only a single element?
        // select first found for now.
        if let Some((&path_start, node_path)) = selector.node_path.split_first() {
            if alias.to_lowercase() == path_start.to_lowercase() {
                if let Some(child_node) = self.find_first_child_element(node_path) {
                    child_node.get_ending_value(&selector.ending)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
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
}
