use xml_mut_data::{NodePath, PathVariant, ValuePath, ValueSelector};
use xml_mut_parse::prelude::*;

#[test]
fn parse_delete_clause_01() {
    let fragment = "delete ItemGroup/PackageRef/fomo";
    let (_, b) = delete_clause(fragment).expect("could not parse delete statement");
    assert_eq!(b.delete_word, "delete");
    assert_eq!(b.targets.len(), 1);

    assert_eq!(
        b.targets[0],
        PathVariant::Node(NodePath {
            path: vec!["ItemGroup", "PackageRef", "fomo"]
        })
    );
}

#[test]
fn parse_delete_clause_02() {
    let fragment = "delete ItemGroup/PackageRef, ItemGroup[@nom]";
    let (_, b) = delete_clause(fragment).expect("could not parse delete statement");
    assert_eq!(b.delete_word, "delete");
    assert_eq!(b.targets.len(), 2);

    assert_eq!(
        b.targets[0],
        PathVariant::Node(NodePath {
            path: vec!["ItemGroup", "PackageRef"]
        })
    );

    assert_eq!(
        b.targets[1],
        PathVariant::Value(ValuePath {
            node_path: NodePath {
                path: vec!["ItemGroup"]
            },
            selector: ValueSelector::Attribute("nom")
        })
    );
}
