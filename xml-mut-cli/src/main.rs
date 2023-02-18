use roxmltree::*;
use xml_mut_impl::prelude::*;
use xml_mut_parse::prelude::*;

fn main() {
    // note: set p@version = p/version@>text should imply where exists p/version
    // since it is required in PredicateEquals.right_side

    let mutation_definition = r###"get ItemGroup/PackageReference as p
where exists p/version
set p@version = p/version@>text
delete p/version"###;

    let xml = r###"<ItemGroup>
<doodles Version="sus"></doodles>
<doodles Version="sus"> </doodles>
<PackageReference Include="zuzu" Version="sus"> bleep <version>3.0</version> bloop </PackageReference>
<PackageReference Include="zizi"> <Version>3.0</Version> </PackageReference>
<PackageReference><Version>4</Version></PackageReference>
<PackageReference Include="zohan" Version="4.0" />
</ItemGroup>"###;

    let (_, mutation) = mutation(mutation_definition).expect("could not parse mutation");
    println!("{mutation:?}");

    let doc: Document = Document::parse(xml).unwrap();

    debug_node(&doc.root_element());

    for node in doc.descendants().filter(|n| n.is_fit(&mutation)) {
        debug_node(&node);

        // TODO: perform node update, set operation
        let replacers = node.get_replacers(&mutation);

        for replacer in &replacers {
            let aa = &doc.input_text()[replacer.bounds.clone()];
            println!("{replacer:?}; original text: '{aa}'");
        }
        let full_replacer = node.apply(&replacers).unwrap();
        println!(" -> {full_replacer:?}");
    }
}

fn debug_node(node: &Node) {
    let bounds = node.range();

    println!(
        "found mutatable node: '{:?}', bounds: {:?}",
        node.tag_name(),
        bounds
    );

    debug_children(node, "".to_string());
}

fn debug_children(node: &Node, indent: String) {
    let name = node.tag_name();
    let text = node.get_input_text();
    
    let bounds = node.range();
    let node_type = node.node_type();
    println!("{indent}{node_type:?} ({bounds:?}); name: {name:?}; full text: {text:?};");

    for child in node.children() {
        debug_children(&child, indent.clone() + " -> ");
    }
}
