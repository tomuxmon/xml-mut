use roxmltree::*;
use xml_mut_parse::prelude::*;

fn main() {
    // note: set p@version = p/version@>text should imply where exists p/version
    // since it is required in PredicateEquals.right_side

    let mutation_definition = r###"get ItemGroup/PackageReference as p
where exists p/version
set p@version = p/version@>text
delete p/version"###;

    let xml = r###"<ItemGroup>

    <PackageReference Include="Microsoft.CodeAnalysis.Analyzers">
        <version>3.3.3</version>
    </PackageReference>

    <PackageReference Include="Microsoft.CodeAnalysis.BannedApiAnalyzers">
    <version>3.3.3</version>
    </PackageReference>

    <PackageReference Include="Microsoft.CodeAnalysis.CSharp.Workspaces" Version="4.4.0" />

</ItemGroup>"###;

    let (_, mutation) = mutation(mutation_definition).expect("could not parse mutation");
    println!("{:?}", mutation);

    let doc: Document = Document::parse(xml).unwrap();

    for node in doc.descendants().filter(|n| {
        n.has_parent_elemnt_path(&mutation.get.node_selector.path)
            && n.fits_predicates(
                &mutation.where_clause.predicates,
                mutation.get.node_selector.alias,
            )
    }) {
        println!("{:?}", node.tag_name());
    }

    println!("Hello, world!");
}

pub trait Searchable<'a, 'input> {
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
        if let Some((path_start, node_path)) = predicate.node_path.split_first() {
            if alias == *path_start {
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
    fn get_value(&self, selector: &ValueSelector, alias: &str) -> Option<&str>;
    fn get_child_element(&self, node_path: &[&str]) -> Option<Box<Self>>;
}

impl<'a, 'input: 'a> Searchable<'a, 'input> for Node<'a, 'input> {
    fn has_parent_elemnt_path(&self, node_path: &[&str]) -> bool {
        if let Some((last, node_path_remaining)) = node_path.split_last() {
            if self.is_element() && self.has_tag_name(*last) {
                if let Some(parent) = self.parent_element() {
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
                n.is_element()
                    && n.has_tag_name(*first)
                    && n.has_child_element_path(node_path_remaining)
            })
        } else {
            true
        }
    }

    fn get_value(&self, selector: &ValueSelector, alias: &str) -> Option<&str> {
        // selector.node_path should yield only a single element?
        // select first found for now.
        if let Some(child_node) = self.get_child_element(&selector.node_path) {
            match selector.ending {
                SelectorEnding::AttributeName(name) => child_node.attribute(name),
                SelectorEnding::NodeText => child_node.text(),
            }
        } else {
            None
        }
    }

    fn get_child_element(&self, node_path: &[&str]) -> Option<Box<Self>> {
        let mut current_node = Box::new(*self);
        for &name in node_path {
            if let Some(child_node) = current_node
                .children()
                .find(|n| n.is_element() && n.has_tag_name(name))
            {
                current_node = Box::new(child_node);
            } else {
                return None;
            }
        }
        Some(current_node)
    }
}
